use libc::{c_int, c_char};
use vec::push;

pub enum InitFlag {
    pub InitTimer       = 0x00000001,
    pub InitAudio       = 0x00000010,
    pub InitVideo       = 0x00000020,
    pub InitCDRom       = 0x00000100,
    pub InitJoystick    = 0x00000200,
    pub InitNoParachute = 0x00100000,
    pub InitEventThread = 0x01000000,
    pub InitEverything  = 0x0000FFFF,
}

impl InitFlag: cmp::Eq {
    pure fn eq(other: &InitFlag) -> bool {
        self as int == *other as int
    }
    pure fn ne(other: &InitFlag) -> bool {
        !self.eq(other)
    }
}

pub enum ErrorCode {
    pub ENoMem,
    pub EFRead,
    pub EFWrite,
    pub EFSeek,
    pub Unsupported
}

pub fn init(flags: ~[InitFlag]) -> int {
    SDL::SDL_Init(util::init_flags_to_bitfield(flags)) as int
}

pub fn init_subsystem(flags: ~[InitFlag]) -> int {
    SDL::SDL_InitSubSystem(util::init_flags_to_bitfield(flags)) as int
}

pub fn quit_subsystem(flags: ~[InitFlag]) {
    SDL::SDL_QuitSubSystem(util::init_flags_to_bitfield(flags))
}

pub fn quit() {
    SDL::SDL_Quit()
}

#[warn(non_implicitly_copyable_typarams)]
pub fn was_init(flags: ~[InitFlag]) -> ~[InitFlag] {
    let bitflags = SDL::SDL_WasInit(util::init_flags_to_bitfield(flags));
    let all_flags = ~[
        InitTimer,
        InitAudio,
        InitVideo,
        InitCDRom,
        InitJoystick
    ];
    
    let mut vecflags = ~[];

    vec::map(all_flags, |flag| {
        if bitflags & (*flag as c_int) != 0 as c_int {
            push(&mut vecflags, *flag);
        }
    });
    vecflags
}

pub fn get_error() -> ~str unsafe {
    let cstr = SDL::SDL_GetError();
    // FIXME: Converting sbuf to *c_char
    let cstr = cast::reinterpret_cast(&cstr);
    str::raw::from_c_str(cstr)
}

pub fn set_error(s: ~str) {
    str::as_buf(s, |buf, _len| {
        // FIXME: Converting sbuf to *c_char
        let buf = unsafe { cast::reinterpret_cast(&buf) };
        SDL::SDL_SetError(buf)
    });
}

pub fn error(code: ErrorCode) {
    SDL::SDL_Error(code as c_int)
}

pub fn clear_error() {
    SDL::SDL_ClearError()
}

mod util {
    pub fn init_flags_to_bitfield(flags: ~[InitFlag]) -> u32 {
        vec::foldl(0u32, flags, |flags, flag| {
            flags | *flag as u32
        })
    }
}

extern mod SDL {
    fn SDL_Init(flags: u32) -> c_int;
    fn SDL_InitSubSystem(flags: u32) -> c_int;
    fn SDL_QuitSubSystem(flags: u32);
    fn SDL_Quit();
    fn SDL_WasInit(flags: u32) -> c_int;
    fn SDL_GetError() -> *c_char;
    // FIXME: This is actually a varargs call
    fn SDL_SetError(fmt: *c_char);
    fn SDL_Error(code: c_int);
    fn SDL_ClearError();
}

