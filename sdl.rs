use vec::push;
use core::libc::types::common::c99::{int32_t};

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

pub fn init(flags: &[InitFlag]) -> int {
    ll::sdl::SDL::SDL_Init(util::init_flags_to_bitfield(flags)) as int
}

pub fn init_subsystem(flags: &[InitFlag]) -> int {
    ll::sdl::SDL::SDL_InitSubSystem(util::init_flags_to_bitfield(flags)) as int
}

pub fn quit_subsystem(flags: &[InitFlag]) {
    ll::sdl::SDL::SDL_QuitSubSystem(util::init_flags_to_bitfield(flags))
}

pub fn quit() {
    ll::sdl::SDL::SDL_Quit()
}

pub fn was_init(flags: &[InitFlag]) -> ~[InitFlag] {
    let bitflags = ll::sdl::SDL::SDL_WasInit(util::init_flags_to_bitfield(flags));
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
        if bitflags & (*flag as int32_t) != 0 as int32_t {
            push(&mut vecflags, *flag);
        }
    });
    move vecflags
}

pub fn get_error() -> ~str unsafe {
    let cstr = ll::error::SDL_GetError();
    // FIXME: Converting sbuf to *c_char
    let cstr = cast::reinterpret_cast(&cstr);
    str::raw::from_c_str(cstr)
}

pub fn set_error(s: &str) {
    str::as_buf(s, |buf, _len| {
        // FIXME: Converting sbuf to *c_char
        let buf = unsafe { cast::reinterpret_cast(&buf) };
        ll::error::SDL_SetError(buf)
    });
}

pub fn error(code: ErrorFlag) {
    ll::error::SDL_Error(code as ll::error::sdl_error_flag)
}

pub fn clear_error() {
    ll::error::SDL_ClearError()
}

mod util {
    pub fn init_flags_to_bitfield(flags: &[InitFlag]) -> u32 {
        vec::foldl(0u32, flags, |flags, flag| {
            flags | *flag as ll::sdl::sdl_init_flag 
        })
    }
}
