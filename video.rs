export surface;
export surface_flag;
export swsurface, hwsurface, asyncblit;
export video_mode_flag;
export anyformat, hwpalette, doublebuf, fullscreen, opengl,
       openglblit, resizable, noframe;
export set_video_mode, free_surface;
export load_bmp;
export display_format;

type rw_ops = c::void;

// FIXME: Should be an enum
type surface = c::void;

enum surface_flag {
    swsurface = 0x00000000,
    hwsurface = 0x00000001,
    asyncblit = 0x00000004,
}

enum video_mode_flag {
    anyformat  = 0x10000000,
    hwpalette  = 0x20000000,
    doublebuf  = 0x40000000,
    fullscreen = 0x80000000,
    opengl     = 0x00000002,
    openglblit = 0x0000000A,
    resizable  = 0x00000010,
    noframe    = 0x00000020,
}

fn set_video_mode(
    width: int,
    height: int,
    bitsperpixel: int,
    surface_flags: [surface_flag],
    video_mode_flags: [video_mode_flag]
) -> *surface {
    let flags = vec::foldl(0u32, surface_flags) {|flags, flag|
        flags | flag as u32
    };
    let flags = vec::foldl(flags, video_mode_flags) {|flags, flag|
        flags | flag as u32
    };
    SDL::SDL_SetVideoMode(width as c::c_int, height as c::c_int,
                          bitsperpixel as c::c_int, flags)
}

fn free_surface(surface: *surface) {
    SDL::SDL_FreeSurface(surface)
}

fn load_bmp(file: str) -> *surface unsafe {
    str::as_buf(file) {|buf|
        let  buf = unsafe::reinterpret_cast(buf);
        str::as_buf("rb") {|rbbuf|
            let rbbuf = unsafe::reinterpret_cast(rbbuf);
            SDL::SDL_LoadBMP_RW(SDL::SDL_RWFromFile(buf, rbbuf), 1 as c::c_int)
        }
    }
}

fn display_format(surface: *surface) -> *surface {
    SDL::SDL_DisplayFormat(surface)
}

native mod SDL {
    fn SDL_SetVideoMode(width: c::c_int, height: c::c_int, 
                        bitsperpixel: c::c_int, flags: u32) -> *surface;
    fn SDL_FreeSurface(surface: *surface);
    fn SDL_LoadBMP_RW(src: *rw_ops, freesrc: c::c_int) -> *surface;
    fn SDL_RWFromFile(file: *c::c_char, mode: *c::c_char) -> *rw_ops;
    fn SDL_DisplayFormat(surface: *surface) -> *surface;
}
