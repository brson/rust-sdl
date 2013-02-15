use audio::AudioFormat;
use core::cast::transmute;
use core::libc::{c_int, malloc, size_t};

pub mod ll {
    use core::libc::c_int;

    pub struct Mix_Chunk {
        allocated: c_int,
        abuf: *u8,
        alen: u32,
        volume: u8,
    }

    #[link_args = "-lSDL_mixer"]
    pub extern {
        fn Mix_OpenAudio(frequency: c_int, format: u16, channels: c_int, chunksize: c_int)
                      -> c_int;
        fn Mix_QuerySpec(frequency: *mut c_int, format: *mut u16, channels: *mut c_int) -> c_int;
        fn Mix_AllocateChannels(numchans: c_int) -> c_int;
        fn Mix_Playing(channel: c_int) -> c_int;
        fn Mix_PlayChannelTimed(channel: c_int, chunk: *Mix_Chunk, loops: c_int, ticks: c_int)
                             -> c_int;
        fn Mix_GetChunk(channel: c_int) -> *Mix_Chunk;
        fn Mix_CloseAudio();
    }
}

pub struct Chunk {
    buffer: ~[u8],
    priv ll_chunk: ll::Mix_Chunk
}

impl Drop for Chunk {
    fn finalize(&self) {
        unsafe {
            // Verify that the chunk is not currently playing.
            //
            // TODO: I can't prove to myself that this is not racy, although I believe it is not
            // as long as all SDL calls are happening from the same thread. Somebody with better
            // knowledge of how SDL_mixer works internally should double check this, though.

            let mut frequency = 0, format = 0, channels = 0;
            if ll::Mix_QuerySpec(&mut frequency, &mut format, &mut channels) == 0 {
                channels = 0;
            }

            let ll_chunk_addr: *ll::Mix_Chunk = &self.ll_chunk;
            for uint::range(0, channels as uint) |ch| {
                if ll::Mix_GetChunk(ch as i32) == ll_chunk_addr {
                    fail!(~"attempt to free a channel that's playing!")
                }
            }
        }
    }
}

impl Chunk {
    static pub fn new(buffer: ~[u8], volume: u8) -> Chunk {
        unsafe {
            let buffer_addr: *u8 = transmute(&buffer[0]);
            let buffer_len = buffer.len() as u32;
            Chunk {
                buffer: buffer,
                ll_chunk: ll::Mix_Chunk {
                    allocated: 0,
                    abuf: buffer_addr,
                    alen: buffer_len,
                    volume: volume
                }
            }
        }
    }

    fn volume(&self) -> u8 { self.ll_chunk.volume }

    fn play_timed(&self, channel: Option<c_int>, loops: c_int, ticks: c_int) -> c_int {
        unsafe {
            let ll_channel = match channel {
                None => -1,
                Some(channel) => channel,
            };
            ll::Mix_PlayChannelTimed(ll_channel, &self.ll_chunk, loops, ticks)
        }
    }

    fn play(&self, channel: Option<c_int>, loops: c_int) -> c_int {
        self.play_timed(channel, loops, -1)
    }
}

pub enum Channels {
    Mono,
    Stereo,
}

impl Channels {
    pub fn count(self) -> c_int { match self { Mono => 1, Stereo => 2 } }
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

struct Query {
    frequency: c_int,
    format: AudioFormat,
    channels: Channels,
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
            Some(channel) => ll::Mix_Playing(channel) as bool,
            None => ll::Mix_Playing(-1) as bool
        }
    }
}

