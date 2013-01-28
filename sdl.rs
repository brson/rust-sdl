use core::cast;
use core::cmp;
use core::libc::{c_int, c_char};
use core::vec::push;
use core::vec;
use util::init_flags_to_bitfield;
use ll;

pub enum InitFlag {
    InitTimer = 0x00000001,
    InitAudio = 0x00000010,
    InitVideo = 0x00000020,
    InitCDRom = 0x00000100,
    InitJoystick = 0x00000200,
    InitHaptic = 0x00001000,
    InitNoParachute = 0x00100000,
    InitEventThread = 0x01000000,
    InitEverything = 0x0000ffff,
}

impl InitFlag: cmp::Eq {
    pure fn eq(&self, other: &InitFlag) -> bool {
        *self as int == *other as int
    }
    pure fn ne(&self, other: &InitFlag) -> bool {
        !self.eq(other)
    }
}

pub enum ErrorFlag {
    ENoMem = 0,
    EFRead = 1,
    EFWrite = 2,
    EFSeek = 3,
    Unsupported = 4,
}

pub fn init(flags: &[InitFlag]) -> bool {
    unsafe {
        (ll::sdl::SDL::SDL_Init(init_flags_to_bitfield(flags)) == 0 as c_int)
    }
}

pub fn init_subsystem(flags: &[InitFlag]) -> bool {
    unsafe {
        (ll::sdl::SDL::SDL_InitSubSystem(init_flags_to_bitfield(flags)) == 0 as c_int)
    }
}

pub fn quit_subsystem(flags: &[InitFlag]) {
    unsafe {
        ll::sdl::SDL::SDL_QuitSubSystem(init_flags_to_bitfield(flags))
    }
}

pub fn quit() {
    unsafe {
        ll::sdl::SDL::SDL_Quit()
    }
}

pub fn was_init(flags: &[InitFlag]) -> ~[InitFlag] {
    unsafe {
        let bitflags = ll::sdl::SDL::SDL_WasInit(init_flags_to_bitfield(flags));
        let all_flags = ~[
            InitTimer,
            InitAudio,
            InitVideo,
            InitCDRom,
            InitJoystick,
            InitHaptic,
            InitNoParachute,
            InitEventThread,
        ];
        
        let mut vecflags = ~[];

        vec::map(all_flags, |flag| {
            if bitflags & (*flag as ll::sdl::SDL_InitFlag) != 0 as ll::sdl::SDL_InitFlag {
                push(&mut vecflags, *flag);
            }
        });
        move vecflags
    }
}

pub fn get_error() -> ~str {
    unsafe {
        let cstr = ll::error::SDL_GetError();

        // FIXME: Converting sbuf to *c_char
        let cstr = cast::reinterpret_cast(&cstr);
        str::raw::from_c_str(cstr)
    }
}

pub fn set_error(s: &str) {
    str::as_buf(s, |buf, _len| {
        unsafe {
            // FIXME: Converting sbuf to *c_char
            let buf = cast::reinterpret_cast(&buf);
            ll::error::SDL_SetError(buf)
        }
    });
}

pub fn error(code: ErrorFlag) {
    unsafe {
        ll::error::SDL_Error(code as ll::error::SDL_errorcode)
    }
}

pub fn clear_error() {
    unsafe {
        ll::error::SDL_ClearError()
    }
}
