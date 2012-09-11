import libc::{c_void, c_int, c_char};

export Surface;
export SurfaceFlag;
export SWSurface, HWSurface, AsyncBlit;
export VideoModeFlag;
export AnyFormat, HWPalette, DoubleBuf, Fullscreen, OpenGL,
       OpenGLBlit, Resizable, NoFrame;
export set_video_mode, free_surface;
export load_bmp;
export display_format;
export blit_surface;
export flip;
export create_rgb_surface;
export fill_rect;
export lock_surface;
export unlock_surface;

type RWOps = c_void;

type Surface = {
    flags: u32,
    format: *c_void,
    w: c_int,
    h: c_int,
    pitch: u16,
    pixels: *c_void,
    offset: c_int
    // FIXME: Remaining are unlisted
};

type Rect = {
    x: i16,
    y: i16,
    w: u16,
    h: u16
};

enum SurfaceFlag {
    SWSurface = 0x00000000,
    HWSurface = 0x00000001,
    AsyncBlit = 0x00000004,
}

enum VideoModeFlag {
    AnyFormat  = 0x10000000,
    HWPalette  = 0x20000000,
    DoubleBuf  = 0x40000000,
    Fullscreen = 0x80000000,
    OpenGL     = 0x00000002,
    OpenGLBlit = 0x0000000A,
    Resizable  = 0x00000010,
    NoFrame    = 0x00000020,
}

fn set_video_mode(
    width: int,
    height: int,
    bitsperpixel: int,
    surface_flags: ~[SurfaceFlag],
    video_mode_flags: ~[VideoModeFlag]
) -> *Surface {
    let flags = vec::foldl(0u32, surface_flags, |flags, flag| {
        flags | flag as u32
    });
    let flags = vec::foldl(flags, video_mode_flags, |flags, flag| {
        flags | flag as u32
    });
    SDL::SDL_SetVideoMode(width as c_int, height as c_int, bitsperpixel as c_int, flags)
}

fn free_surface(surface: *Surface) {
    SDL::SDL_FreeSurface(surface)
}

fn load_bmp(file: ~str) -> *Surface unsafe {
    str::as_buf(file, |buf, _len| {
        let buf = unsafe::reinterpret_cast(&buf);
        str::as_buf(~"rb", |rbbuf, _len| {
            let rbbuf = unsafe::reinterpret_cast(&rbbuf);
            SDL::SDL_LoadBMP_RW(SDL::SDL_RWFromFile(buf, rbbuf), 1 as c_int)
        })
    })
}

fn display_format(surface: *Surface) -> *Surface {
    SDL::SDL_DisplayFormat(surface)
}

fn blit_surface(src: *Surface, srcrect: *Rect, dst: *Surface, dstrect: *Rect) -> bool {
    let res = SDL::SDL_UpperBlit(src, srcrect, dst, dstrect);
    return res == 0 as c_int;
}

fn flip(screen: *Surface) -> bool {
    SDL::SDL_Flip(screen) == 0 as c_int
}

fn create_rgb_surface(
    surface_flags: ~[SurfaceFlag],
    width: int, height: int, bits_per_pixel: int,
    rmask: u32, gmask: u32, bmask: u32, amask: u32) -> *Surface {

    let flags = vec::foldl(0u32, surface_flags, |flags, flag| {
        flags | flag as u32
    });
    SDL::SDL_CreateRGBSurface(
        flags, width as c_int, height as c_int, bits_per_pixel as c_int,
        rmask, gmask, bmask, amask)
}

fn fill_rect(surface: *Surface, rect: *Rect, color: u32) {
    SDL::SDL_FillRect(surface, rect, color);
}

fn lock_surface(surface: *Surface) {
    SDL::SDL_LockSurface(surface);
}

fn unlock_surface(surface: *Surface) {
    SDL::SDL_UnlockSurface(surface);
}

extern mod SDL {
    fn SDL_SetVideoMode(width: c_int, height: c_int, bitsperpixel: c_int, flags: u32) -> *Surface;
    fn SDL_FreeSurface(surface: *Surface);
    fn SDL_LoadBMP_RW(src: *RWOps, freesrc: c_int) -> *Surface;
    fn SDL_RWFromFile(file: *c_char, mode: *c_char) -> *RWOps;
    fn SDL_DisplayFormat(surface: *Surface) -> *Surface;
    fn SDL_UpperBlit(src: *Surface, srcrect: *Rect,
                     dst: *Surface, dstrect: *Rect) -> c_int;
    fn SDL_Flip(screen: *Surface) -> c_int;
    fn SDL_CreateRGBSurface(flags: u32, width: c_int, height: c_int,
                            bitsPerPixel: c_int,
                            Rmask: u32, Gmask: u32, Bmask: u32, Amask: u32) -> *Surface;
    fn SDL_FillRect(dst: *Surface, dstrect: *Rect, color: u32);
    fn SDL_LockSurface(surface: *Surface);
    fn SDL_UnlockSurface(surface: *Surface);
}
