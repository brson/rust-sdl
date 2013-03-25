// Setup linking for all targets.
#[cfg(target_os="macos")]
mod mac {
    #[cfg(mac_framework)]
    #[link_args="-framework SDL"]
    extern {}

    #[cfg(mac_dylib)]
    #[link_args="-lSDL"]
    extern {}
}

#[cfg(target_os="win32")]
#[cfg(target_os="linux")]
#[cfg(target_os="freebsd")]
mod others {
    #[link_args="-lSDL"]
    extern {}
}

pub mod ll {
    use core::libc::{c_int, c_uint, c_schar, uint32_t};

    pub type SDL_errorcode = c_uint;
    pub static SDL_ENOMEM: SDL_errorcode = 0;
    pub static SDL_EFREAD: SDL_errorcode = 1;
    pub static SDL_EFWRITE: SDL_errorcode = 2;
    pub static SDL_EFSEEK: SDL_errorcode = 3;
    pub static SDL_UNSUPPORTED: SDL_errorcode = 4;
    pub static SDL_LASTERROR: SDL_errorcode = 5;

    pub type SDL_InitFlag = uint32_t;
    pub static SDL_INIT_TIMER: SDL_InitFlag = 0x00000001;
    pub static SDL_INIT_AUDIO: SDL_InitFlag = 0x00000010;
    pub static SDL_INIT_VIDEO: SDL_InitFlag = 0x00000020;
    pub static SDL_INIT_CDROM: SDL_InitFlag = 0x00000100;
    pub static SDL_INIT_JOYSTICK: SDL_InitFlag = 0x00000200;
    pub static SDL_INIT_NOPARACHUTE: SDL_InitFlag = 0x00100000;
    pub static SDL_INIT_EVENTTHREAD: SDL_InitFlag = 0x01000000;
    pub static SDL_INIT_EVERYTHING: SDL_InitFlag = 0x0000FFFF;

    pub extern {
        fn SDL_ClearError();
        fn SDL_Error(code: SDL_errorcode);
        fn SDL_SetError(fmt: *c_schar);
        fn SDL_GetError() -> *c_schar;
        fn SDL_Quit();
        fn SDL_QuitSubSystem(flags: SDL_InitFlag);
        fn SDL_Init(flags: uint32_t) -> c_int;
        fn SDL_InitSubSystem(flags: SDL_InitFlag) -> c_int;
        fn SDL_WasInit(flags: SDL_InitFlag) -> SDL_InitFlag;
    }
}

#[deriving(Eq)]
pub struct Rect {
    pub x: i16,
    pub y: i16,
    pub w: u16,
    pub h: u16
}

pub fn Rect(x: i16, y: i16, w: u16, h: u16) -> Rect {
    Rect { x: x, y: y, w: w, h: h }
}

pub impl Rect {
    fn new(x: i16, y: i16, w: u16, h: u16) -> Rect {
        Rect {
            x: x,
            y: y,
            w: w,
            h: h
        }
    }
}

#[deriving(Eq)]
pub enum InitFlag {
    pub InitTimer = ll::SDL_INIT_TIMER as int,
    pub InitAudio = ll::SDL_INIT_AUDIO as int,
    pub InitVideo = ll::SDL_INIT_VIDEO as int,
    pub InitCDRom = ll::SDL_INIT_CDROM as int,
    pub InitJoystick = ll::SDL_INIT_JOYSTICK as int,
    pub InitNoParachute = ll::SDL_INIT_NOPARACHUTE as int,
    pub InitEventThread = ll::SDL_INIT_EVENTTHREAD as int,
    pub InitEverything = ll::SDL_INIT_EVERYTHING as int,
}

#[deriving(Eq)]
pub enum Error {
    pub NoMemError = ll::SDL_ENOMEM as int,
    pub ReadError = ll::SDL_EFREAD as int,
    pub WriteError = ll::SDL_EFWRITE as int,
    pub SeekError = ll::SDL_EFSEEK as int,
    pub UnsupportedError = ll::SDL_UNSUPPORTED as int
}

pub fn init(flags: &[InitFlag]) -> bool {
    unsafe {
        ll::SDL_Init(do vec::foldl(0, flags) |flags, &flag| {
            flags | flag as ll::SDL_InitFlag
        }) == 0
    }
}

pub fn init_subsystem(flags: &[InitFlag]) -> bool {
    unsafe {
        ll::SDL_InitSubSystem(do vec::foldl(0, flags) |flags, &flag| {
            flags | flag as ll::SDL_InitFlag
        }) == 0
    }
}

pub fn quit_subsystem(flags: &[InitFlag]) {
    let flags = do vec::foldl(0, flags) |flags, &flag| {
        flags | flag as ll::SDL_InitFlag
    };

    unsafe { ll::SDL_QuitSubSystem(flags); }
}

pub fn quit() {
    unsafe { ll::SDL_Quit(); }
}

pub fn was_inited(flags: &[InitFlag]) -> ~[InitFlag] {
    let flags = do vec::foldl(0, flags) |flags, &flag| {
        flags | flag as ll::SDL_InitFlag
    };
    let bitflags = unsafe { ll::SDL_WasInit(flags) };

    do [InitTimer,
        InitAudio,
        InitVideo,
        InitCDRom,
        InitJoystick,
        InitNoParachute,
        InitEventThread,
        InitEverything].filter_mapped |&flag| {
        if bitflags & (flag as ll::SDL_InitFlag) != 0 { Some(flag) }
        else { None }
    }
}

pub fn get_error() -> ~str {
    unsafe {
        let cstr = ll::SDL_GetError();

        str::raw::from_c_str(cast::reinterpret_cast(&cstr))
    }
}

pub fn set_error(err: &str) {
    do str::as_c_str(err) |buf| {
        unsafe { ll::SDL_SetError(buf); }
    }
}

pub fn set_error_from_code(err: Error) {
    unsafe { ll::SDL_Error(err as ll::SDL_errorcode) }
}

pub fn clear_error() {
    unsafe { ll::SDL_ClearError(); }
}
