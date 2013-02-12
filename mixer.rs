use audio::AudioFormat;
use ll::mixer::{Mix_AllocateChannels, Mix_Chunk, Mix_CloseAudio, Mix_GetChunk, Mix_OpenAudio, };
use ll::mixer::{Mix_PlayChannelTimed, Mix_Playing, Mix_QuerySpec};

use core::cast::transmute;
use core::libc::{c_int, malloc, size_t};

pub struct Chunk {
    buffer: ~[u8],
    priv ll_chunk: Mix_Chunk
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
            if Mix_QuerySpec(&mut frequency, &mut format, &mut channels) == 0 {
                channels = 0;
            }

            let ll_chunk_addr: *Mix_Chunk = &self.ll_chunk;
            for uint::range(0, channels as uint) |ch| {
                if Mix_GetChunk(ch as i32) == ll_chunk_addr {
                    die!(~"attempt to free a channel that's playing!")
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
                ll_chunk: Mix_Chunk {
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
            Mix_PlayChannelTimed(ll_channel, &self.ll_chunk, loops, ticks)
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
        if Mix_OpenAudio(frequency, format.to_ll_format(), channels.count(), chunksize) == 0 {
            Ok(())
        } else {
            Err(())
        }
    }
}

pub fn close() {
    unsafe {
        Mix_CloseAudio()
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
        if Mix_QuerySpec(&mut frequency, &mut ll_format, &mut ll_channels) == 0 {
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
        Mix_AllocateChannels(numchans)
    }
}

pub fn playing(channel: Option<c_int>) -> bool {
    unsafe {
        match channel {
            Some(channel) => Mix_Playing(channel) as bool,
            None => Mix_Playing(-1) as bool
        }
    }
}

