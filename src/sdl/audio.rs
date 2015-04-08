use self::ll::{AUDIO_S16LSB, AUDIO_S16MSB, AUDIO_S8, AUDIO_U16LSB, AUDIO_U16MSB, AUDIO_U8};
use self::ll::{SDL_LockAudio, SDL_MixAudio, SDL_OpenAudio};
use self::ll::{SDL_UnlockAudio};

use std::mem::{forget, transmute};
use libc::{c_int, c_void, uint16_t};
use std::ptr::null_mut;

pub mod ll {
    #![allow(non_camel_case_types)]

    use libc::{c_int, c_void, uint16_t};

    pub const AUDIO_U8: uint16_t = 0x0008;
    pub const AUDIO_S8: uint16_t = 0x8008;
    pub const AUDIO_U16LSB: uint16_t = 0x0010;
    pub const AUDIO_S16LSB: uint16_t = 0x8010;
    pub const AUDIO_U16MSB: uint16_t = 0x1010;
    pub const AUDIO_S16MSB: uint16_t = 0x9010;
    pub const AUDIO_U16: uint16_t = AUDIO_U16LSB;
    pub const AUDIO_S16: uint16_t = AUDIO_S16LSB;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SDL_AudioSpec {
        pub freq: c_int,
        pub format: u16,
        pub channels: u8,
        pub silence: u8,
        pub samples: u16,
        pub padding: u16,
        pub size: u32,
        pub callback: *mut u8,
        pub userdata: *mut c_void,
    }

    extern "C" {
        pub fn SDL_OpenAudio(desired: *mut SDL_AudioSpec, obtained: *mut SDL_AudioSpec) -> c_int;
        pub fn SDL_PauseAudio(pause_on: c_int);
        pub fn SDL_MixAudio(dst: *mut u8, src: *const u8, len: u32, volume: c_int);
        pub fn SDL_LockAudio();
        pub fn SDL_UnlockAudio();
        pub fn SDL_CloseAudio();
    }
}

#[derive(Copy, Clone)]
pub enum AudioFormat {
    U8 = AUDIO_U8 as isize,
    S8 = AUDIO_S8 as isize,
    U16Lsb = AUDIO_U16LSB as isize,
    S16Lsb = AUDIO_S16LSB as isize,
    U16Msb = AUDIO_U16MSB as isize,
    S16Msb = AUDIO_S16MSB as isize
}

pub static U16_AUDIO_FORMAT: AudioFormat = AudioFormat::U16Lsb;
pub static S16_AUDIO_FORMAT: AudioFormat = AudioFormat::S16Lsb;

impl AudioFormat {
    pub fn to_ll_format(self) -> uint16_t {
        match self {
            AudioFormat::U8 => AUDIO_U8,
            AudioFormat::S8 => AUDIO_S8,
            AudioFormat::U16Lsb => AUDIO_U16LSB,
            AudioFormat::S16Lsb => AUDIO_S16LSB,
            AudioFormat::U16Msb => AUDIO_U16MSB,
            AudioFormat::S16Msb => AUDIO_S16MSB,
        }
    }

    pub fn from_ll_format(x: uint16_t) -> AudioFormat {
        match x {
            AUDIO_U8 => AudioFormat::U8,
            AUDIO_S8 => AudioFormat::S8,
            AUDIO_U16LSB => AudioFormat::U16Lsb,
            AUDIO_S16LSB => AudioFormat::S16Lsb,
            AUDIO_U16MSB => AudioFormat::U16Msb,
            AUDIO_S16MSB => AudioFormat::S16Msb,
            _ => panic!("unexpected format")
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Channels {
    Mono,
    Stereo,
}

impl Channels {
    pub fn new(count: c_int) -> Channels { if count == 1 { Channels::Mono } else { Channels::Stereo } }
    pub fn count(self) -> c_int          { match self { Channels::Mono => 1, Channels::Stereo => 2 } }
}

pub type AudioCallback = fn(&mut [u8]);

#[derive(Copy)]
pub struct DesiredAudioSpec {
    pub freq: c_int,
    pub format: AudioFormat,
    pub channels: Channels,
    pub samples: u16,
    pub callback: AudioCallback,
}

impl DesiredAudioSpec {
    fn to_ll_spec(self) -> ll::SDL_AudioSpec {
        unsafe {
            let DesiredAudioSpec { freq, format, channels, samples, callback } = self;
            ll::SDL_AudioSpec {
                freq: freq,
                format: format.to_ll_format(),
                channels: channels.count() as u8,
                silence: 0,
                samples: samples,
                padding: 0,
                size: 0,
                callback: native_callback as *mut u8,
                userdata: transmute(Box::new(callback)),
            }
        }
    }
}

impl Clone for DesiredAudioSpec {
    fn clone(&self) -> DesiredAudioSpec {
        *self
    }
}

#[derive(Copy, Clone)]
pub struct ObtainedAudioSpec {
    pub freq: c_int,
    pub format: AudioFormat,
    pub channels: Channels,
    pub silence: u8,
    pub samples: u16,
    pub size: u32,
}

impl ObtainedAudioSpec {
    fn from_ll_spec(spec: &ll::SDL_AudioSpec) -> ObtainedAudioSpec {
        ObtainedAudioSpec {
            freq: spec.freq,
            format: AudioFormat::from_ll_format(spec.format),
            channels: Channels::new(spec.channels as c_int),
            silence: spec.silence,
            samples: spec.samples,
            size: spec.size,
        }
    }
}

extern fn native_callback(userdata: *const c_void, stream: *mut u8, len: c_int) {
    unsafe {
        let callback: Box<AudioCallback> = transmute(userdata);
        let buffer = transmute((stream, len as usize));
        (*callback)(buffer);
        forget(callback);   // Don't free the callback!
    }
}

pub fn open(desired: DesiredAudioSpec) -> Result<ObtainedAudioSpec,()> {
    unsafe {
        let mut ll_desired = desired.to_ll_spec();
        let mut ll_obtained = ll::SDL_AudioSpec {
            freq: 0,
            format: 0,
            channels: 0,
            silence: 0,
            samples: 0,
            padding: 0,
            size: 0,
            callback: null_mut(),
            userdata: null_mut(),
        };

        if SDL_OpenAudio(&mut ll_desired, &mut ll_obtained) < 0 {
            Err(())
        } else {
            Ok(ObtainedAudioSpec::from_ll_spec(&ll_obtained))
        }
    }
}

pub fn pause(paused: bool) {
    unsafe {
        ll::SDL_PauseAudio(paused as c_int);
    }
}

pub fn close() {
    unsafe {
        ll::SDL_CloseAudio();
    }
}

pub fn mix(dest: &mut [u8], src: &[u8], volume: c_int) {
    unsafe {
        assert!(dest.len() == src.len());
        SDL_MixAudio(&mut dest[0], &src[0], dest.len() as u32, volume);
    }
}

pub fn with_lock<F: Fn() -> bool>(f: F) -> bool {
    unsafe {
        SDL_LockAudio();
        let result = f();
        SDL_UnlockAudio();
        result
    }
}
