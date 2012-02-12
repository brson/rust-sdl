import c = ctypes;

export init_flag;
export init_timer, init_audio, init_video, init_cdrom, init_joystick,
       init_noparachute, init_eventthread, init_everything;
export init, init_subsystem, quit_subsystem, quit;
export was_init;
export get_error, set_error;

enum init_flag {
    init_timer       = 0x00000001,
    init_audio       = 0x00000010,
    init_video       = 0x00000020,
    init_cdrom       = 0x00000100,
    init_joystick    = 0x00000200,
    init_noparachute = 0x00100000,
    init_eventthread = 0x01000000,
    init_everything  = 0x0000FFFF,
}

fn init(flags: [init_flag]) -> int {
    SDL::SDL_Init(util::flags_to_bitfield(flags)) as int
}

fn init_subsystem(flags: [init_flag]) -> int {
    SDL::SDL_InitSubSystem(util::flags_to_bitfield(flags)) as int
}

fn quit_subsystem(flags: [init_flag]) {
    SDL::SDL_QuitSubSystem(util::flags_to_bitfield(flags))
}

fn quit() {
    SDL::SDL_Quit()
}

fn was_init(flags: [init_flag]) -> [init_flag] {
    let bitflags = SDL::SDL_WasInit(util::flags_to_bitfield(flags));
    let all_flags = [
        init_timer,
        init_audio,
        init_video,
        init_cdrom,
        init_joystick
    ];
    vec::foldl([], all_flags) {|vecflags, flag|
        if bitflags & (flag as c::c_int) != 0 as c::c_int {
            vecflags + [flag]
        } else {
            vecflags
        }
    }
}

fn get_error() -> str unsafe {
    let cstr = SDL::SDL_GetError();
    // FIXME: Converting sbuf to *c_char
    let cstr = unsafe::reinterpret_cast(cstr);
    str::from_cstr(cstr)
}

fn set_error(s: str) {
    str::as_buf(s) {|buf|
        // FIXME: Converting sbuf to *c_char
        let buf = unsafe { unsafe::reinterpret_cast(buf) };
        SDL::SDL_SetError(buf)
    }
}

mod util {
    fn flags_to_bitfield(flags: [init_flag]) -> u32 {
        vec::foldl(0u32, flags) {|flags, flag|
            flags | flag as u32
        }
    }
}

native mod SDL {
    fn SDL_Init(flags: u32) -> c::c_int;
    fn SDL_InitSubSystem(flags: u32) -> c::c_int;
    fn SDL_QuitSubSystem(flags: u32);
    fn SDL_Quit();
    fn SDL_WasInit(flags: u32) -> c::c_int;
    fn SDL_GetError() -> *c::c_char;
    // FIXME: This is actually a varargs call
    fn SDL_SetError(fmt: *c::c_char);
}

#[test]
fn test_init() {
    init([init_everything]);
    quit();
}

#[test]
fn test_was_init() {
    init([init_timer, init_video]);
    // FIXME
    quit();
}