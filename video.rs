use libc::{c_void, c_int, c_char};

type RWOps = c_void;

pub type Surface = {
    flags: u32,
    format: *c_void,
    w: c_int,
    h: c_int,
    pitch: u16,
    pixels: *c_void,
    offset: c_int
    // FIXME: Remaining are unlisted
};

pub type Rect = {
    x: i16,
    y: i16,
    w: u16,
    h: u16
};

pub enum SurfaceFlag {
    pub SWSurface = 0x00000000,
    pub HWSurface = 0x00000001,
    pub AsyncBlit = 0x00000004,
}

pub enum VideoModeFlag {
    pub AnyFormat  = 0x10000000,
    pub HWPalette  = 0x20000000,
    pub DoubleBuf  = 0x40000000,
    pub Fullscreen = 0x80000000,
    pub OpenGL     = 0x00000002,
    pub OpenGLBlit = 0x0000000A,
    pub Resizable  = 0x00000010,
    pub NoFrame    = 0x00000020,
}

pub fn set_video_mode(
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

pub fn free_surface(surface: *Surface) {
    SDL::SDL_FreeSurface(surface)
}

pub fn load_bmp(file: ~str) -> *Surface unsafe {
    str::as_buf(file, |buf, _len| {
        let buf = cast::reinterpret_cast(&buf);
        str::as_buf(~"rb", |rbbuf, _len| {
            let rbbuf = cast::reinterpret_cast(&rbbuf);
            SDL::SDL_LoadBMP_RW(SDL::SDL_RWFromFile(buf, rbbuf), 1 as c_int)
        })
    })
}

pub fn display_format(surface: *Surface) -> *Surface {
    SDL::SDL_DisplayFormat(surface)
}

pub fn blit_surface(src: *Surface, srcrect: *Rect, dst: *Surface, dstrect: *Rect) -> bool {
    let res = SDL::SDL_UpperBlit(src, srcrect, dst, dstrect);
    return res == 0 as c_int;
}

pub fn flip(screen: *Surface) -> bool {
    SDL::SDL_Flip(screen) == 0 as c_int
}

pub fn create_rgb_surface(
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

pub fn fill_rect(surface: *Surface, rect: *Rect, color: u32) {
    SDL::SDL_FillRect(surface, rect, color);
}

pub fn lock_surface(surface: *Surface) {
    SDL::SDL_LockSurface(surface);
}

pub fn unlock_surface(surface: *Surface) {
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
