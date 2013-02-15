use libc::uint16_t;

pub mod ll {
    use libc::uint16_t;

    pub const AUDIO_U8: uint16_t = 0x0008;
    pub const AUDIO_S8: uint16_t = 0x8008;
    pub const AUDIO_U16LSB: uint16_t = 0x0010;
    pub const AUDIO_S16LSB: uint16_t = 0x8010;
    pub const AUDIO_U16MSB: uint16_t = 0x1010;
    pub const AUDIO_S16MSB: uint16_t = 0x9010;
    pub const AUDIO_U16: uint16_t = AUDIO_U16LSB;
    pub const AUDIO_S16: uint16_t = AUDIO_S16LSB;
}

// FIXME: Doesn't work. Huh?
// use ll::{AUDIO_S16LSB, AUDIO_S16MSB, AUDIO_S8, AUDIO_U16LSB, AUDIO_U16MSB, AUDIO_U8};

pub enum AudioFormat {
    U8AudioFormat = ll::AUDIO_U8 as int,
    S8AudioFormat = ll::AUDIO_S8 as int,
    U16LsbAudioFormat = ll::AUDIO_U16LSB as int,
    S16LsbAudioFormat = ll::AUDIO_S16LSB as int,
    U16MsbAudioFormat = ll::AUDIO_U16MSB as int,
    S16MsbAudioFormat = ll::AUDIO_S16MSB as int
}

impl AudioFormat {
    fn to_ll_format(self) -> uint16_t {
        match self {
            U8AudioFormat => ll::AUDIO_U8,
            S8AudioFormat => ll::AUDIO_S8,
            U16LsbAudioFormat => ll::AUDIO_U16LSB,
            S16LsbAudioFormat => ll::AUDIO_S16LSB,
            U16MsbAudioFormat => ll::AUDIO_U16MSB,
            S16MsbAudioFormat => ll::AUDIO_S16MSB,
        }
    }
    static fn from_ll_format(x: uint16_t) -> AudioFormat {
        /* FIXME: Strangest rustc bug here, importing the consts doesn't work
           So we'll manually expand the values for now
        match x {
            AUDIO_U8 => U8AudioFormat,
            AUDIO_S8 => S8AudioFormat,
            AUDIO_U16LSB => U16LsbAudioFormat,
            AUDIO_S16LSB => S16LsbAudioFormat,
            AUDIO_U16MSB => U16MsbAudioFormat,
            AUDIO_S16MSB => S16MsbAudioFormat,
            _ => die!(~"unexpected format")
        }*/
        match x {
            0x0008 => U8AudioFormat,
            0x8008 => S8AudioFormat,
            0x0010 => U16LsbAudioFormat,
            0x8010 => S16LsbAudioFormat,
            0x1010 => U16MsbAudioFormat,
            0x9010 => S16MsbAudioFormat,
            _ => die!(~"unexpected format")
        }
    }
}

