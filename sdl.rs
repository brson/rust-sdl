export InitFlag;
export InitTimer, InitAudio, InitVideo, InitCDRom, InitJoystick,
       InitNoParachute, InitEventThread, InitEverything;
export init, init_subsystem, quit_subsystem, quit;
export was_init;
export ErrorCode;
export ENoMem, EFRead, EFWrite, EFSeek, Unsupported;
export get_error, set_error, error, clear_error;
export video, keyboard, event;
import libc::{c_int, c_char};
import vec::push;

enum InitFlag {
    InitTimer       = 0x00000001,
    InitAudio       = 0x00000010,
    InitVideo       = 0x00000020,
    InitCDRom       = 0x00000100,
    InitJoystick    = 0x00000200,
    InitNoParachute = 0x00100000,
    InitEventThread = 0x01000000,
    InitEverything  = 0x0000FFFF,
}

impl InitFlag: cmp::Eq {
    pure fn eq(&&other: InitFlag) -> bool {
        self as int == other as int
    }
}

enum ErrorCode {
    ENoMem,
    EFRead,
    EFWrite,
    EFSeek,
    Unsupported
}

fn init(flags: ~[InitFlag]) -> int {
    SDL::SDL_Init(util::init_flags_to_bitfield(flags)) as int
}

fn init_subsystem(flags: ~[InitFlag]) -> int {
    SDL::SDL_InitSubSystem(util::init_flags_to_bitfield(flags)) as int
}

fn quit_subsystem(flags: ~[InitFlag]) {
    SDL::SDL_QuitSubSystem(util::init_flags_to_bitfield(flags))
}

fn quit() {
    SDL::SDL_Quit()
}

#[warn(non_implicitly_copyable_typarams)]
fn was_init(flags: ~[InitFlag]) -> ~[InitFlag] {
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
        if bitflags & (flag as c_int) != 0 as c_int {
            push(vecflags, flag);
        }
    });
    vecflags
}

fn get_error() -> ~str unsafe {
    let cstr = SDL::SDL_GetError();
    // FIXME: Converting sbuf to *c_char
    let cstr = unsafe::reinterpret_cast(&cstr);
    str::unsafe::from_c_str(cstr)
}

fn set_error(s: ~str) {
    str::as_buf(s, |buf, _len| {
        // FIXME: Converting sbuf to *c_char
        let buf = unsafe { unsafe::reinterpret_cast(&buf) };
        SDL::SDL_SetError(buf)
    });
}

fn error(code: ErrorCode) {
    SDL::SDL_Error(code as c_int)
}

fn clear_error() {
    SDL::SDL_ClearError()
}

mod util {
    fn init_flags_to_bitfield(flags: ~[InitFlag]) -> u32 {
        vec::foldl(0u32, flags, |flags, flag| {
            flags | flag as u32
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

