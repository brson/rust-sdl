import libc::{c_void, c_int, c_char};

export surface;
export surface_flag;
export swsurface, hwsurface, asyncblit;
export video_mode_flag;
export anyformat, hwpalette, doublebuf, fullscreen, opengl,
       openglblit, resizable, noframe;
export set_video_mode, free_surface;
export load_bmp;
export display_format;
export blit_surface;
export flip;
export create_rgb_surface;
export fill_rect;
export lock_surface;
export unlock_surface;

type rw_ops = c_void;

type surface = {
    flags: u32,
    format: *c_void,
    w: c_int,
    h: c_int,
    pitch: u16,
    pixels: *c_void,
    offset: c_int
    // FIXME: Remaining are unlisted
};

type rect = {
    x: i16,
    y: i16,
    w: u16,
    h: u16
};

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
    SDL::SDL_SetVideoMode(width as c_int, height as c_int,
                          bitsperpixel as c_int, flags)
}

fn free_surface(surface: *surface) {
    SDL::SDL_FreeSurface(surface)
}

fn load_bmp(file: str) -> *surface unsafe {
    str::as_buf(file) {|buf|
        let  buf = unsafe::reinterpret_cast(buf);
        str::as_buf("rb") {|rbbuf|
            let rbbuf = unsafe::reinterpret_cast(rbbuf);
            SDL::SDL_LoadBMP_RW(SDL::SDL_RWFromFile(buf, rbbuf), 1 as c_int)
        }
    }
}

fn display_format(surface: *surface) -> *surface {
    SDL::SDL_DisplayFormat(surface)
}

fn blit_surface(src: *surface, srcrect: *rect,
                dst: *surface, dstrect: *rect) -> bool {
    let res = SDL::SDL_UpperBlit(src, srcrect, dst, dstrect);
    ret res == 0 as c_int;
}

fn flip(screen: *surface) -> bool {
    SDL::SDL_Flip(screen) == 0 as c_int
}

fn create_rgb_surface(
    surface_flags: [surface_flag],
    width: int, height: int, bits_per_pixel: int,
    rmask: u32, gmask: u32, bmask: u32, amask: u32) -> *surface {

    let flags = vec::foldl(0u32, surface_flags) {|flags, flag|
        flags | flag as u32
    };
    SDL::SDL_CreateRGBSurface(
        flags, width as c_int, height as c_int, bits_per_pixel as c_int,
        rmask, gmask, bmask, amask)
}

fn fill_rect(surface: *surface, rect: *rect, color: u32) {
    SDL::SDL_FillRect(surface, rect, color);
}

fn lock_surface(surface: *surface) {
    SDL::SDL_LockSurface(surface);
}

fn unlock_surface(surface: *surface) {
    SDL::SDL_UnlockSurface(surface);
}

native mod SDL {
    fn SDL_SetVideoMode(width: c_int, height: c_int, 
                        bitsperpixel: c_int, flags: u32) -> *surface;
    fn SDL_FreeSurface(surface: *surface);
    fn SDL_LoadBMP_RW(src: *rw_ops, freesrc: c_int) -> *surface;
    fn SDL_RWFromFile(file: *c_char, mode: *c_char) -> *rw_ops;
    fn SDL_DisplayFormat(surface: *surface) -> *surface;
    fn SDL_UpperBlit(src: *surface, srcrect: *rect,
                     dst: *surface, dstrect: *rect) -> c_int;
    fn SDL_Flip(screen: *surface) -> c_int;
    fn SDL_CreateRGBSurface(flags: u32, width: c_int, height: c_int,
                            bitsPerPixel: c_int,
                            Rmask: u32, Gmask: u32, Bmask: u32, Amask: u32) -> *surface;
    fn SDL_FillRect(dst: *surface, dstrect: *rect, color: u32);
    fn SDL_LockSurface(surface: *surface);
    fn SDL_UnlockSurface(surface: *surface);
}
