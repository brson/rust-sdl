#![crate_name = "sdl_mixer"]
#![comment = "SDL_mixer binding"]
#![license = "MIT"]
#![crate_type = "lib"]

extern crate libc;
extern crate sdl;

use libc::{c_int};

use sdl::audio::{AudioFormat, Channels, Mono, Stereo};
use sdl::video::ll::SDL_RWFromFile; // XXX refactoring
use sdl::get_error;

// Setup linking for all targets.
#[cfg(not(target_os = "macos"))]
#[cfg(not(mac_framework))]
#[link(name = "SDL_mixer")]
extern {}

#[cfg(target_os = "macos", mac_framework)]
#[link(name = "SDL_mixer", kind = "framework")]
extern {}

pub mod ll {
    #![allow(non_camel_case_types)]

    use sdl::video::ll::SDL_RWops; // XXX refactoring

    use libc::c_int;

    pub struct Mix_Chunk {
        pub allocated: c_int,
        pub abuf: *mut u8,
        pub alen: u32,
        pub volume: u8,
    }

    extern "C" {
        pub fn Mix_OpenAudio(frequency: c_int, format: u16, channels: c_int, chunksize: c_int)
              -> c_int;
        pub fn Mix_QuerySpec(frequency: *mut c_int, format: *mut u16, channels: *mut c_int)
              -> c_int;
        pub fn Mix_LoadWAV_RW(src: *mut SDL_RWops, freesrc: c_int) -> *mut Mix_Chunk;
        pub fn Mix_FreeChunk(chunk: *mut Mix_Chunk);
        pub fn Mix_AllocateChannels(numchans: c_int) -> c_int;
        pub fn Mix_Playing(channel: c_int) -> c_int;
        pub fn Mix_PlayChannelTimed(channel: c_int, chunk: *mut Mix_Chunk, loops: c_int, ticks: c_int)
              -> c_int;
        pub fn Mix_GetChunk(channel: c_int) -> *mut Mix_Chunk;
        pub fn Mix_CloseAudio();
        pub fn Mix_Volume(channel: c_int, volume: c_int) -> c_int;
        pub fn Mix_ReserveChannels(num: c_int) -> c_int;
        pub fn Mix_GroupChannel(which: c_int, tag: c_int) -> c_int;
        pub fn Mix_GroupNewer(tag: c_int) -> c_int;
        pub fn Mix_HaltChannel(channel: c_int) -> c_int;
    }
}

pub struct Chunk {
    data: ChunkData
}

enum ChunkData {
    Borrowed(*mut ll::Mix_Chunk),
    Allocated(*mut ll::Mix_Chunk),
    OwnedBuffer(ChunkAndBuffer)
}

struct ChunkAndBuffer {
    pub buffer: Vec<u8>,
    pub ll_chunk: ll::Mix_Chunk
}

unsafe fn check_if_not_playing(ll_chunk_addr: *mut ll::Mix_Chunk) {
    // Verify that the chunk is not currently playing.
    //
    // TODO: I can't prove to myself that this is not racy, although I believe it is not
    // as long as all SDL calls are happening from the same thread. Somebody with better
    // knowledge of how SDL_mixer works internally should double check this, though.

    let mut frequency = 0;
    let mut format = 0;
    let mut channels = 0;
    if ll::Mix_QuerySpec(&mut frequency, &mut format, &mut channels) == 0 {
        channels = 0;
    }

    for ch in range(0, (channels as uint)) {
        if ll::Mix_GetChunk(ch as i32) == ll_chunk_addr {
            fail!("attempt to free a channel that's playing!")
        }
    }
}

impl Drop for Chunk {
    fn drop(&mut self) {
        unsafe {
            match self.data {
                Borrowed(_) => (),
                Allocated(ll_chunk) => {
                    check_if_not_playing(ll_chunk);
                    ll::Mix_FreeChunk(ll_chunk);
                },
                OwnedBuffer(ref mut chunk) => {
                    check_if_not_playing(&mut chunk.ll_chunk);
                }
            }
        }
    }
}

impl Chunk {
    pub fn new(mut buffer: Vec<u8>, volume: u8) -> Chunk {
        let buffer_addr: *mut u8 = buffer.as_mut_ptr();
        let buffer_len = buffer.len() as u32;
        Chunk {
            data: OwnedBuffer(
                ChunkAndBuffer {
                    buffer: buffer,
                    ll_chunk: ll::Mix_Chunk {
                        allocated: 0,
                        abuf: buffer_addr,
                        alen: buffer_len,
                        volume: volume
                    }
                }
            )
        }
    }

    pub fn from_wav(path: &Path) -> Result<Chunk, String> {
        let cpath = path.to_c_str();
        let mode = "rb".to_c_str();
        let raw = unsafe {
            ll::Mix_LoadWAV_RW(SDL_RWFromFile(cpath.as_ptr(), mode.as_ptr()), 1)
        };

        if raw.is_null() { Err(get_error()) }
        else { Ok(Chunk { data: Allocated(raw) }) }
    }

    pub fn to_ll_chunk(&self) -> *const ll::Mix_Chunk {
        match self.data {
            Borrowed(ll_chunk) => ll_chunk as *const _,
            Allocated(ll_chunk) => ll_chunk as *const _,
            OwnedBuffer(ref chunk) => {
                let ll_chunk: *const ll::Mix_Chunk = &chunk.ll_chunk;
                ll_chunk
            }
        }
    }

    pub fn to_mut_ll_chunk(&mut self) -> *mut ll::Mix_Chunk {
        match self.data {
            Borrowed(ll_chunk) => ll_chunk,
            Allocated(ll_chunk) => ll_chunk,
            OwnedBuffer(ref mut chunk) => {
                let ll_chunk: *mut ll::Mix_Chunk = &mut chunk.ll_chunk;
                ll_chunk
            }
        }
    }

    pub fn volume(&self) -> u8 {
        let ll_chunk: *const ll::Mix_Chunk = self.to_ll_chunk();
        unsafe { (*ll_chunk).volume }
    }

    pub fn play_timed(&mut self, channel: Option<c_int>, loops: c_int, ticks: c_int) -> c_int {
        unsafe {
            let ll_channel = match channel {
                None => -1,
                Some(channel) => channel,
            };
            ll::Mix_PlayChannelTimed(ll_channel, self.to_mut_ll_chunk(), loops, ticks)
        }
    }

    pub fn play(&mut self, channel: Option<c_int>, loops: c_int) -> c_int {
        self.play_timed(channel, loops, -1)
    }
}

pub fn open(frequency: c_int, format: AudioFormat, channels: Channels, chunksize: c_int)
         -> Result<(),()> {
    unsafe {
        if ll::Mix_OpenAudio(frequency, format.to_ll_format(), channels.count(), chunksize) == 0 {
            Ok(())
        } else {
            Err(())
        }
    }
}

pub fn close() {
    unsafe {
        ll::Mix_CloseAudio()
    }
}

pub struct Query {
    pub frequency: c_int,
    pub format: AudioFormat,
    pub channels: Channels,
}

pub fn query() -> Option<Query> {
    unsafe {
        let mut frequency = 0;
        let mut ll_format = 0;
        let mut ll_channels = 0;
        if ll::Mix_QuerySpec(&mut frequency, &mut ll_format, &mut ll_channels) == 0 {
            return None;
        }
        Some(Query {
            frequency: frequency,
            format: AudioFormat::from_ll_format(ll_format),
            channels: if ll_channels == 1 { Mono } else { Stereo }
        })
    }
}

pub fn allocate_channels(numchans: c_int) -> c_int {
    unsafe {
        ll::Mix_AllocateChannels(numchans)
    }
}

pub fn playing(channel: Option<c_int>) -> bool {
    unsafe {
        match channel {
            Some(channel) => ll::Mix_Playing(channel) == 0,
            None => ll::Mix_Playing(-1) == 0
        }
    }
}

pub fn num_playing(channel: Option<c_int>) -> c_int {
    unsafe {
        match channel {
            Some(channel) => ll::Mix_Playing(channel),
            None => ll::Mix_Playing(-1)
        }
    }
}

pub fn get_channel_volume(channel: Option<c_int>) -> c_int {
    unsafe {
        let ll_channel = channel.unwrap_or(-1);
        ll::Mix_Volume(ll_channel, -1)
    }
}

pub fn set_channel_volume(channel: Option<c_int>, volume: c_int) {
    unsafe {
        let ll_channel = channel.unwrap_or(-1);
        ll::Mix_Volume(ll_channel, volume);
    }
}

pub fn reserve_channels(num: c_int) -> c_int {
    unsafe { ll::Mix_ReserveChannels(num) }
}

pub fn group_channel(which: Option<c_int>, tag: Option<c_int>) -> bool {
    unsafe {
        let ll_which = which.unwrap_or(-1);
        let ll_tag = tag.unwrap_or(-1);
        ll::Mix_GroupChannel(ll_which, ll_tag) != 0
    }
}

pub fn newest_in_group(tag: Option<c_int>) -> Option<c_int> {
    unsafe {
        let ll_tag = tag.unwrap_or(-1);
        let channel = ll::Mix_GroupNewer(ll_tag);
        if channel == -1 {None} else {Some(channel)}
    }
}

pub fn halt_channel(channel: c_int) -> c_int {
    unsafe {
        ll::Mix_HaltChannel(channel)
    }
}

