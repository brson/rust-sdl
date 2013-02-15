use ll::audio::{AUDIO_S16LSB, AUDIO_S16MSB, AUDIO_S8, AUDIO_U16LSB, AUDIO_U16MSB, AUDIO_U8};

pub enum AudioFormat {
    U8AudioFormat,
    S8AudioFormat,
    U16LsbAudioFormat,
    S16LsbAudioFormat,
    U16MsbAudioFormat,
    S16MsbAudioFormat,
}

impl AudioFormat {
    fn to_ll_format(self) -> u16 {
        match self {
            U8AudioFormat => AUDIO_U8,
            S8AudioFormat => AUDIO_S8,
            U16LsbAudioFormat => AUDIO_U16LSB,
            S16LsbAudioFormat => AUDIO_S16LSB,
            U16MsbAudioFormat => AUDIO_U16MSB,
            S16MsbAudioFormat => AUDIO_S16MSB,
        }
    }
    static fn from_ll_format(x: u16) -> AudioFormat {
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

