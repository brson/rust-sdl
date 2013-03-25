use audio::{AudioFormat, Channels, Mono, Stereo};
use video::ll::SDL_RWFromFile; // XXX refactoring
use get_error;

use core::cast::transmute;
use core::libc::{c_int, malloc, size_t};

pub mod ll {
    use video::ll::SDL_RWops; // XXX refactoring

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
        fn Mix_LoadWAV_RW(src: *SDL_RWops, freesrc: c_int) -> *Mix_Chunk;
        fn Mix_FreeChunk(chunk: *Mix_Chunk);
        fn Mix_AllocateChannels(numchans: c_int) -> c_int;
        fn Mix_Playing(channel: c_int) -> c_int;
        fn Mix_PlayChannelTimed(channel: c_int, chunk: *Mix_Chunk, loops: c_int, ticks: c_int)
                             -> c_int;
        fn Mix_GetChunk(channel: c_int) -> *Mix_Chunk;
        fn Mix_CloseAudio();
        fn Mix_HaltChannel(channel: c_int) -> c_int;
    }
}

pub struct Chunk {
    priv data: ChunkData
}

enum ChunkData {
    Borrowed(*ll::Mix_Chunk),
    Allocated(*ll::Mix_Chunk),
    OwnedBuffer(ChunkAndBuffer)
}

struct ChunkAndBuffer {
    buffer: ~[u8],
    ll_chunk: ll::Mix_Chunk
}

unsafe fn check_if_not_playing(ll_chunk_addr: *ll::Mix_Chunk) {
    // Verify that the chunk is not currently playing.
    //
    // TODO: I can't prove to myself that this is not racy, although I believe it is not
    // as long as all SDL calls are happening from the same thread. Somebody with better
    // knowledge of how SDL_mixer works internally should double check this, though.

    let mut frequency = 0, format = 0, channels = 0;
    if ll::Mix_QuerySpec(&mut frequency, &mut format, &mut channels) == 0 {
        channels = 0;
    }

    for uint::range(0, channels as uint) |ch| {
        if ll::Mix_GetChunk(ch as i32) == ll_chunk_addr {
            fail!(~"attempt to free a channel that's playing!")
        }
    }
}

impl Drop for Chunk {
    fn finalize(&self) {
        unsafe {
            match self.data {
                Borrowed(_) => (),
                Allocated(ll_chunk) => {
                    check_if_not_playing(ll_chunk);
                    ll::Mix_FreeChunk(ll_chunk);
                },
                OwnedBuffer(ref chunk) => {
                    check_if_not_playing(&chunk.ll_chunk);
                }
            }
        }
    }
}

pub impl Chunk {
    pub fn new(buffer: ~[u8], volume: u8) -> ~Chunk {
        unsafe {
            let buffer_addr: *u8 = transmute(&buffer[0]);
            let buffer_len = buffer.len() as u32;
            ~Chunk {
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
    }

    fn from_wav(path: &Path) -> Result<~Chunk, ~str> {
        let raw = unsafe {
            do str::as_c_str(path.to_str()) |buf| {
                do str::as_c_str("rb") |mode_buf| {
                    ll::Mix_LoadWAV_RW(SDL_RWFromFile(buf, mode_buf), 1)
                }
            }
        };

        if raw.is_null() { Err(get_error()) }
        else { Ok(~Chunk { data: Allocated(raw) }) }
    }

    fn to_ll_chunk(&self) -> *ll::Mix_Chunk {
        match self.data {
            Borrowed(ll_chunk) => ll_chunk,
            Allocated(ll_chunk) => ll_chunk,
            OwnedBuffer(ref chunk) => {
                let ll_chunk: *ll::Mix_Chunk = &chunk.ll_chunk; ll_chunk
            }
        }
    }

    fn volume(&self) -> u8 {
        let ll_chunk: *ll::Mix_Chunk = self.to_ll_chunk();
        unsafe { (*ll_chunk).volume }
    }

    fn play_timed(&self, channel: Option<c_int>, loops: c_int, ticks: c_int) -> c_int {
        unsafe {
            let ll_channel = match channel {
                None => -1,
                Some(channel) => channel,
            };
            ll::Mix_PlayChannelTimed(ll_channel, self.to_ll_chunk(), loops, ticks)
        }
    }

    fn play(&self, channel: Option<c_int>, loops: c_int) -> c_int {
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

pub fn halt_channel(channel: c_int) -> c_int {
    unsafe {
        ll::Mix_HaltChannel(channel)
    }
}

