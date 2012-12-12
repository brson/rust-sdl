use vec::push;

pub fn init(flags: &[InitFlag]) -> int {
    SDL::SDL_Init(util::init_flags_to_bitfield(flags)) as int
}

pub fn init_subsystem(flags: &[InitFlag]) -> int {
    SDL::SDL_InitSubSystem(util::init_flags_to_bitfield(flags)) as int
}

pub fn quit_subsystem(flags: &[InitFlag]) {
    SDL::SDL_QuitSubSystem(util::init_flags_to_bitfield(flags))
}

pub fn quit() {
    SDL::SDL_Quit()
}

pub fn was_init(flags: &[InitFlag]) -> ~[InitFlag] {
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
    move vecflags
}

pub fn get_error() -> ~str unsafe {
    let cstr = SDL::SDL_GetError();
    // FIXME: Converting sbuf to *c_char
    let cstr = cast::reinterpret_cast(&cstr);
    str::raw::from_c_str(cstr)
}

pub fn set_error(s: &str) {
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
    pub fn init_flags_to_bitfield(flags: &[InitFlag]) -> u32 {
        vec::foldl(0u32, flags, |flags, flag| {
            flags | *flag as u32
        })
    }
}
