use self::ll::{AUDIO_S16LSB, AUDIO_S16MSB, AUDIO_S8, AUDIO_U16LSB, AUDIO_U16MSB, AUDIO_U8};
use self::ll::{SDL_LockAudio, SDL_MixAudio, SDL_OpenAudio};
use self::ll::{SDL_UnlockAudio};

use std::cast::{forget, transmute};
use std::libc::{c_int, c_void, uint16_t};
use std::ptr::null;

pub mod ll {
    use std::libc::{c_int, c_void, uint16_t};

    pub static AUDIO_U8: uint16_t = 0x0008;
    pub static AUDIO_S8: uint16_t = 0x8008;
    pub static AUDIO_U16LSB: uint16_t = 0x0010;
    pub static AUDIO_S16LSB: uint16_t = 0x8010;
    pub static AUDIO_U16MSB: uint16_t = 0x1010;
    pub static AUDIO_S16MSB: uint16_t = 0x9010;
    pub static AUDIO_U16: uint16_t = AUDIO_U16LSB;
    pub static AUDIO_S16: uint16_t = AUDIO_S16LSB;

    pub struct SDL_AudioSpec {
        freq: c_int,
        format: u16,
        channels: u8,
        silence: u8,
        samples: u16,
        padding: u16,
        size: u32,
        callback: *u8,
        userdata: *c_void,
    }

    extern {
        pub fn SDL_OpenAudio(desired: *mut SDL_AudioSpec, obtained: *mut SDL_AudioSpec) -> c_int;
        pub fn SDL_PauseAudio(pause_on: c_int);
        pub fn SDL_MixAudio(dst: *mut u8, src: *u8, len: u32, volume: c_int);
        pub fn SDL_LockAudio();
        pub fn SDL_UnlockAudio();
        pub fn SDL_CloseAudio();
    }
}

pub enum AudioFormat {
    U8AudioFormat = AUDIO_U8 as int,
    S8AudioFormat = AUDIO_S8 as int,
    U16LsbAudioFormat = AUDIO_U16LSB as int,
    S16LsbAudioFormat = AUDIO_S16LSB as int,
    U16MsbAudioFormat = AUDIO_U16MSB as int,
    S16MsbAudioFormat = AUDIO_S16MSB as int
}

pub static U16_AUDIO_FORMAT: AudioFormat = U16LsbAudioFormat;
pub static S16_AUDIO_FORMAT: AudioFormat = S16LsbAudioFormat;

impl AudioFormat {
    pub fn to_ll_format(self) -> uint16_t {
        match self {
            U8AudioFormat => AUDIO_U8,
            S8AudioFormat => AUDIO_S8,
            U16LsbAudioFormat => AUDIO_U16LSB,
            S16LsbAudioFormat => AUDIO_S16LSB,
            U16MsbAudioFormat => AUDIO_U16MSB,
            S16MsbAudioFormat => AUDIO_S16MSB,
        }
    }

    pub fn from_ll_format(x: uint16_t) -> AudioFormat {
        match x {
            AUDIO_U8 => U8AudioFormat,
            AUDIO_S8 => S8AudioFormat,
            AUDIO_U16LSB => U16LsbAudioFormat,
            AUDIO_S16LSB => S16LsbAudioFormat,
            AUDIO_U16MSB => U16MsbAudioFormat,
            AUDIO_S16MSB => S16MsbAudioFormat,
            _ => fail!(~"unexpected format")
        }
    }
}

#[deriving(Eq)]
pub enum Channels {
    Mono,
    Stereo,
}

impl Channels {
    pub fn new(count: c_int) -> Channels { if count == 1 { Mono } else { Stereo } }
    pub fn count(self) -> c_int          { match self { Mono => 1, Stereo => 2 } }
}

pub type AudioCallback = ~fn(&mut [u8]);

pub struct DesiredAudioSpec {
    freq: c_int,
    format: AudioFormat,
    channels: Channels,
    samples: u16,
    callback: AudioCallback,
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
                callback: native_callback,
                userdata: transmute(~callback),
            }
        }
    }
}

pub struct ObtainedAudioSpec {
    freq: c_int,
    format: AudioFormat,
    channels: Channels,
    silence: u8,
    samples: u16,
    size: u32,
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

extern fn native_callback(userdata: *c_void, stream: *mut u8, len: c_int) {
    unsafe {
        let callback: ~AudioCallback = transmute(userdata);
        let buffer = transmute((stream, len as uint));
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
            callback: null(),
            userdata: null(),
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

pub fn with_lock<R>(f: &fn() -> R) -> R {
    unsafe {
        SDL_LockAudio();
        let result = f();
        SDL_UnlockAudio();
        result
    }
}

