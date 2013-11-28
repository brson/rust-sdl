use std::cast;
use std::libc::{c_int, c_float};
use std::ptr;
use std::rand;
use std::rand::Rng;
use std::vec;

use Rect;
use get_error;

pub mod ll {
    use Rect;

    use std::libc::{c_void, c_uint, c_int, c_float, c_schar, c_uchar, uint8_t, uint16_t};
    use std::libc::{uint32_t, int32_t};

    pub type SDL_Rect = Rect;

    struct SDL_RWops_Anon {
        data: [c_uchar, ..24],
    }

    pub struct SDL_RWops {
         seek: *uint8_t,
         read: *uint8_t,
         write: *uint8_t,
         close: *uint8_t,
         _type: uint32_t,
         hidden: SDL_RWops_Anon
    }

    pub struct SDL_Surface {
         flags: uint32_t,
         format: *SDL_PixelFormat,
         w: c_int,
         h: c_int,
         pitch: uint16_t,
         pixels: *c_void,
         offset: c_int,
         hwdata: *c_void,
         clip_rect: SDL_Rect,
         unused1: uint32_t,
         locked: uint32_t,
         map: *c_void,
         format_version: c_uint,
         refcount: c_int
    }

    pub struct SDL_Color {
         r: uint8_t,
         g: uint8_t,
         b: uint8_t,
         unused: uint8_t
    }

    pub struct SDL_Palette {
         ncolors: c_int,
         colors: *SDL_Color,
    }

    pub struct SDL_PixelFormat {
         palette: *SDL_Palette,
         BitsPerPixel: uint8_t,
         BytesPerPixel: uint8_t,
         Rloss: uint8_t,
         Gloss: uint8_t,
         Bloss: uint8_t,
         Aloss: uint8_t,
         Rshift: uint8_t,
         Gshift: uint8_t,
         Bshift: uint8_t,
         Ashift: uint8_t,
         Rmask: uint32_t,
         Gmask: uint32_t,
         Bmask: uint32_t,
         Amask: uint32_t,
         colorkey: uint32_t,
         alpha: uint8_t,
    }

    pub struct SDL_VideoInfo {
         flags: uint32_t,        // actually a set of packed fields
         video_mem: uint32_t,
         vfmt: *SDL_PixelFormat,
         current_w: c_int,
         current_h: c_int,
    }

    extern "C" {
        pub fn SDL_CreateRGBSurface(flags: uint32_t,
                                          width: c_int,
                                          height: c_int,
                                          depth: c_int,
                                          Rmask: uint32_t,
                                          Gmask: uint32_t,
                                          Bmask: uint32_t,
                                          Amask: uint32_t) -> *SDL_Surface;
        pub fn SDL_CreateRGBSurfaceFrom(pixels: *c_void,
                                              width: c_int,
                                              height: c_int,
                                              depth: c_int,
                                              pitch: c_int,
                                              Rmask: uint32_t,
                                              Gmask: uint32_t,
                                              Bmask: uint32_t,
                                              Amask: uint32_t) -> *SDL_Surface;
        pub fn SDL_FreeSurface(surface: *SDL_Surface);
        pub fn SDL_MapRGB(format: *SDL_PixelFormat,
                                r: uint8_t,
                                g: uint8_t,
                                b: uint8_t) -> uint32_t;
        pub fn SDL_MapRGBA(format: *SDL_PixelFormat,
                                 r: uint8_t,
                                 g: uint8_t,
                                 b: uint8_t,
                                 a: uint8_t) -> uint32_t;
        pub fn SDL_GetRGB(pixel: uint32_t,
                                fmt: *SDL_PixelFormat,
                                r: *uint8_t,
                                g: *uint8_t,
                                b: *uint8_t);
        pub fn SDL_GetRGBA(pixel: uint32_t,
                                 fmt: *SDL_PixelFormat,
                                 r: *uint8_t,
                                 g: *uint8_t,
                                 b: *uint8_t,
                                 a: *uint8_t);
        pub fn SDL_SetVideoMode(width: c_int, height: c_int, bpp: c_int, flags: uint32_t)
                        -> *SDL_Surface;
        pub fn SDL_VideoModeOK(width: c_int, height: c_int, bpp: c_int, flags: uint32_t) -> c_int;
        pub fn SDL_GetVideoInfo() -> *SDL_VideoInfo;
        pub fn SDL_GetVideoSurface() -> *SDL_Surface;
        pub fn SDL_UpdateRect(screen: *SDL_Surface,
                                    x: int32_t,
                                    y: int32_t,
                                    w: uint32_t,
                                    h: uint32_t);
        pub fn SDL_UpdateRects(screen: *SDL_Surface, numrects: c_int, rects: *SDL_Rect);
        pub fn SDL_SetColors(surface: *SDL_Surface,
                                   colors: *SDL_Color,
                                   firstcolor: c_int,
                                   ncolors: c_int) -> c_int;
        pub fn SDL_SetPalette(surface: *SDL_Surface,
                                    flags: c_int,
                                    colors: *SDL_Color,
                                    firstcolor: c_int,
                                    ncolors: c_int) -> c_int;
        pub fn SDL_LockSurface(surface: *SDL_Surface) -> c_int;
        pub fn SDL_UnlockSurface(surface: *SDL_Surface);
        pub fn SDL_Flip(screen: *SDL_Surface) -> c_int;
        pub fn SDL_ConvertSurface(src: *SDL_Surface, fmt: *SDL_PixelFormat, flags: uint32_t)
                        -> *SDL_Surface;
        pub fn SDL_DisplayFormat(surface: *SDL_Surface) -> *SDL_Surface;
        pub fn SDL_DisplayFormatAlpha(surface: *SDL_Surface) -> *SDL_Surface;
        pub fn SDL_SetColorKey(surface: *SDL_Surface, flag: uint32_t, key: uint32_t) -> c_int;
        pub fn SDL_SetAlpha(surface: *SDL_Surface, flag: uint32_t, alpha: uint8_t) -> c_int;
        pub fn SDL_SetClipRect(surface: *SDL_Surface, rect: *SDL_Rect);
        pub fn SDL_UpperBlit(src: *SDL_Surface,
                                   srcrect: *SDL_Rect,
                                   dst: *SDL_Surface,
                                   dstrect: *SDL_Rect) -> c_int;
        pub fn SDL_FillRect(dst: *SDL_Surface, dstrect: *SDL_Rect, color: uint32_t) -> c_int;
        pub fn SDL_SetGamma(r: c_float, g: c_float, b: c_float) -> c_int;
        pub fn SDL_SetGammaRamp(r: *uint16_t, g: *uint16_t, b: *uint16_t) -> c_int;
        pub fn SDL_GetGammaRamp(r: *uint16_t, g: *uint16_t, b: *uint16_t) -> c_int;
        pub fn SDL_RWFromFile(file: *c_schar, mode: *c_schar) -> *SDL_RWops;
        pub fn SDL_LoadBMP_RW(src: *SDL_RWops, freesrc: c_int) -> *SDL_Surface;
        pub fn SDL_SaveBMP_RW(surface: *SDL_Surface, dst: *SDL_RWops, freedst: c_int) -> c_int;
        pub fn SDL_GL_SwapBuffers();
    }
}

#[deriving(Eq)]
pub struct Surface {
    raw: *ll::SDL_Surface,
    owned: bool
}

fn wrap_surface(raw: *ll::SDL_Surface, owned: bool) -> ~Surface {
    ~Surface{ raw: raw, owned: owned }
}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe {
            if self.owned {
                ll::SDL_FreeSurface(self.raw);
            }
        }
    }
}

#[deriving(Eq)]
pub struct Palette {
    colors: ~[Color]
}

fn wrap_palette(palette: *ll::SDL_Palette) -> Option<Palette> {
    match palette.is_null() {
        true => None,
        _ => Some(Palette {
            colors: unsafe {
                vec::from_buf((*palette).colors, (*palette).ncolors as uint).map(|color| {
                    Color::from_struct(color)
                })
            }
        })
    }
}

fn unwrap_palette(palette: &Palette) -> ll::SDL_Palette {
    ll::SDL_Palette {
        ncolors: palette.colors.len() as c_int,
        colors: vec::raw::to_ptr(palette.colors.map(|color| {
            color.to_struct()
        }))
    }
}

#[deriving(Eq)]
pub struct PixelFormat {
     palette: Option<Palette>,
     bpp: u8,
     r_loss: u8,
     g_loss: u8,
     b_loss: u8,
     a_loss: u8,
     r_shift: u8,
     g_shift: u8,
     b_shift: u8,
     a_shift: u8,
     r_mask: u32,
     g_mask: u32,
     b_mask: u32,
     a_mask: u32,
     color_key: u32,
     alpha: u8
}

fn wrap_pixel_format(raw: *ll::SDL_PixelFormat) -> PixelFormat {
    let fmt = & unsafe { *raw };
    PixelFormat {
        palette: wrap_palette(fmt.palette),
        bpp: fmt.BitsPerPixel,
        r_loss: fmt.Rloss,
        g_loss: fmt.Gloss,
        b_loss: fmt.Bloss,
        a_loss: fmt.Aloss,
        r_shift: fmt.Rshift,
        g_shift: fmt.Gshift,
        b_shift: fmt.Bshift,
        a_shift: fmt.Ashift,
        r_mask: fmt.Rmask,
        g_mask: fmt.Gmask,
        b_mask: fmt.Bmask,
        a_mask: fmt.Amask,
        color_key: fmt.colorkey,
        alpha: fmt.alpha,
    }
}

fn unwrap_pixel_format(fmt: &PixelFormat) -> ll::SDL_PixelFormat {
    ll::SDL_PixelFormat {
        // FIXME: this will be freed at the end of this scope?
        palette: match fmt.palette {
            None => ptr::null(),
            Some(_) => {
                let workaround : *ll::SDL_Palette = &unwrap_palette(fmt.palette.get_ref());
                workaround
            }
        },
        BitsPerPixel: fmt.bpp,
        BytesPerPixel: fmt.bpp / 8,
        Rloss: fmt.r_loss,
        Gloss: fmt.g_loss,
        Bloss: fmt.b_loss,
        Aloss: fmt.a_loss,
        Rshift: fmt.r_shift,
        Gshift: fmt.g_shift,
        Bshift: fmt.b_shift,
        Ashift: fmt.a_shift,
        Rmask: fmt.r_mask,
        Gmask: fmt.g_mask,
        Bmask: fmt.b_mask,
        Amask: fmt.a_mask,
        colorkey: fmt.color_key,
        alpha: fmt.alpha
    }
}

#[deriving(Eq)]
pub enum Color {
    RGB(u8, u8, u8),
    RGBA(u8, u8, u8, u8)
}

impl rand::Rand for Color {
    fn rand<R: rand::Rng>(rng: &mut R) -> Color {
        if rng.gen() { RGBA(rng.gen(), rng.gen(), rng.gen(), rng.gen()) }
        else { RGB(rng.gen(), rng.gen(), rng.gen()) }
    }
}

impl Color {
    pub fn from_mapped(bit: u32, fmt: *ll::SDL_PixelFormat) -> Color {
        let r = 0;
        let g = 0;
        let b = 0;
        let a = 0;

        unsafe { ll::SDL_GetRGBA(bit, fmt,
                                 &r, &g,
                                 &b, &a) }

        RGBA(r, g, b, a)
    }

    pub fn to_mapped(&self, fmt: *ll::SDL_PixelFormat) -> u32 {
        match *self {
            RGB(r, g, b) => unsafe { ll::SDL_MapRGB(fmt, r, g, b) },
            RGBA(r, g, b, a) => unsafe { ll::SDL_MapRGBA(fmt, r, g, b, a) }
        }
    }

    pub fn from_struct(c: &ll::SDL_Color) -> Color {
        RGB(c.r, c.g, c.b)
    }

    pub fn to_struct(&self) -> ll::SDL_Color {
        match *self {
            RGB(r, g, b) => ll::SDL_Color {
                r: r,
                g: g,
                b: b,
                unused: 0,
            },
            RGBA(r, g, b, _) => ll::SDL_Color {
                r: r,
                g: g,
                b: b,
                unused: 0,
            }
        }
    }
}

#[deriving(Eq)]
pub enum SurfaceFlag {
    SWSurface = 0x00000000,
    HWSurface = 0x00000001,
    AsyncBlit = 0x00000004,
    SrcColorKey = 0x00001000,
    SrcAlpha = 0x00010000,
    RLEAccel = 0x00004000
}

#[deriving(Eq)]
pub enum VideoFlag {
    AnyFormat = 0x10000000,
    HWPalette = 0x20000000,
    DoubleBuf = 0x40000000,
    Fullscreen = 0x80000000,
    OpenGL = 0x00000002,
    OpenGLBlit = 0x0000000A,
    Resizable = 0x00000010,
    NoFrame = 0x00000020
}

pub fn set_video_mode(w: int, h: int, bpp: int,
                      surface_flags: &[SurfaceFlag],
                      video_flags: &[VideoFlag]) -> Result<~Surface, ~str> {
    let flags = surface_flags.iter().fold(0u32, |flags, &flag| {
        flags | flag as u32
    });
    let flags = video_flags.iter().fold(flags, |flags, &flag| {
        flags | flag as u32
    });

    unsafe {
        let raw = ll::SDL_SetVideoMode(w as c_int, h as c_int,
                                       bpp as c_int, flags);

        if raw == ptr::null() { Err(get_error()) }
        else { Ok(wrap_surface(raw, false)) }
    }
}

pub fn is_video_mode_ok(w: int, h: int, bpp: int,
                        surface_flags: &[SurfaceFlag],
                        video_flags: &[VideoFlag]) -> Option<int> {
    let flags = surface_flags.iter().fold(0u32, |flags, &flag| {
        flags | flag as u32
    });
    let flags = video_flags.iter().fold(flags, |flags, &flag| {
        flags | flag as u32
    });

    unsafe {
        let bpp = ll::SDL_VideoModeOK(w as c_int, h as c_int,
                                      bpp as c_int, flags);

        if bpp == 0 { None }
        else { Some(bpp as int) }
    }
}

#[deriving(Eq)]
pub enum VideoInfoFlag {
    HWAvailable    = 0x00000001,
    WMAvailable    = 0x00000002,
    BlitHW         = 0x00000200,
    BlitHWColorkey = 0x00000400,
    BlitHWAlpha    = 0x00000800,
    BlitSW         = 0x00001000,
    BlitSWColorkey = 0x00002000,
    BlitSWAlpha    = 0x00004000,
    BlitFill       = 0x00008000,
}

pub struct VideoInfo {
     flags: ~[VideoInfoFlag],
     width: int,
     height: int,
     format: PixelFormat,
}

fn wrap_video_info_flags(bitflags: u32) -> ~[VideoInfoFlag] {
    let flags = [HWAvailable,
        WMAvailable,
        BlitHW,
        BlitHWColorkey,
        BlitHWAlpha,
        BlitSW,
        BlitSWColorkey,
        BlitSWAlpha,
        BlitFill];

    flags.iter().filter_map(|&flag| {
        if bitflags & (flag as u32) != 0 { Some(flag) }
        else { None }
    }).collect()
}

pub fn get_video_info() -> ~VideoInfo {
    let raw = unsafe { ll::SDL_GetVideoInfo() };
    ~VideoInfo {
        flags:  wrap_video_info_flags(unsafe { (*raw).flags } as u32),
        width:  unsafe { (*raw).current_w } as int,
        height: unsafe { (*raw).current_h } as int,
        format: wrap_pixel_format(unsafe { (*raw).vfmt }),
    }
}

pub enum PaletteType {
    LogicalPaletteType = 1,
    PhysicalPaletteType
}

pub fn get_video_surface() -> Result<~Surface, ~str> {
    let raw = unsafe { ll::SDL_GetVideoSurface() };

    if raw.is_null() { Err(get_error()) }
    else { Ok(wrap_surface(raw, false)) }
}

// TODO: get_video_modes, get_video_driver_name

impl Surface {
    pub fn new(surface_flags: &[SurfaceFlag], width: int, height: int, bpp: int,
               rmask: u32, gmask: u32, bmask: u32, amask: u32) -> Result<~Surface, ~str> {
        let flags = surface_flags.iter().fold(0u32, |flags, flag| { flags | *flag as u32 });

        unsafe {
            let raw = ll::SDL_CreateRGBSurface(flags, width as c_int, height as c_int, bpp as c_int,
                                               rmask, gmask, bmask, amask);

            if raw == ptr::null() {
                Err(get_error())
            } else {
                Ok(~Surface { raw: raw, owned: true })
            }
        }
    }

    pub fn from_bmp(path: &Path) -> Result<~Surface, ~str> {
        let raw = path.to_c_str().with_ref(|path| {
                "rb".to_c_str().with_ref(|mode| {
                        unsafe {
                            ll::SDL_LoadBMP_RW(ll::SDL_RWFromFile(path, mode), 1)
                        }
                    })
            });

        if raw.is_null() { Err(get_error()) }
        else { Ok(wrap_surface(raw, true)) }
    }

    // TODO: from_data (hard because the pixel data has to stay alive)

    pub fn get_width(&self) -> u16 {
        unsafe { (*self.raw).w as u16 }
    }

    pub fn get_height(&self) -> u16 {
        unsafe { (*self.raw).h as u16 }
    }

    pub fn get_size(&self) -> (u16, u16) {
        (self.get_width(), self.get_height())
    }

    pub fn get_rect(&self) -> Rect {
        Rect {
            x: 0,
            y: 0,
            w: self.get_width(),
            h: self.get_height()
        }
    }

    pub fn update_rect(&self, rect: &Rect) {
        unsafe {
            ll::SDL_UpdateRect(self.raw, rect.x as i32, rect.y as i32,
                               rect.w as u32, rect.h as u32);
        }
    }

    pub fn update_rects(&self, rects: &[Rect]) {
        unsafe {
            ll::SDL_UpdateRects(self.raw, rects.len() as c_int,
                                cast::transmute(vec::raw::to_ptr(rects)));
        }
    }

    pub fn set_colors(&self, colors: ~[Color]) -> bool {
        let colors = colors.map(|color| {
            color.to_struct()
        });

        unsafe { ll::SDL_SetColors(self.raw, vec::raw::to_ptr(colors), 0,
                                   colors.len() as c_int) == 1 }
    }

    pub fn set_palette(&self, palettes: &[PaletteType],
                   colors: ~[Color]) -> bool {
        let colors = colors.map(|color| {
            color.to_struct()
        });
        let flags = palettes.iter().fold(0 as c_int, |flags, &flag| {
            flags | flag as c_int
        });

        unsafe { ll::SDL_SetPalette(self.raw, flags,
                                    vec::raw::to_ptr(colors), 0,
                                    colors.len() as c_int) == 1 }
    }

    pub fn lock(&self) -> bool {
        unsafe { ll::SDL_LockSurface(self.raw) == 0 }
    }

    /// Locks a surface so that the pixels can be directly accessed safely.
    pub fn with_lock<R>(&self, f: |&mut [u8]| -> R) -> R {
        unsafe {
            if ll::SDL_LockSurface(self.raw) != 0 { fail!(~"could not lock surface"); }
            let len = (*self.raw).pitch as uint * ((*self.raw).h as uint);
            let pixels: &mut [u8] = cast::transmute(((*self.raw).pixels, len));
            let rv = f(pixels);
            ll::SDL_UnlockSurface(self.raw);
            rv
        }
    }

    pub fn unlock(&self) {
        unsafe { ll::SDL_UnlockSurface(self.raw); }
    }

    pub fn flip(&self) -> bool {
        unsafe { ll::SDL_Flip(self.raw) == 0 }
    }

    pub fn convert(&self, fmt: &PixelFormat, flags: &[SurfaceFlag]) -> Result<~Surface, ~str> {
        let flags = flags.iter().fold(0u32, |flags, &flag| {
            flags | flag as u32
        });

        let rawfmt = unwrap_pixel_format(fmt);

        let new = unsafe { ll::SDL_ConvertSurface(self.raw, &rawfmt, flags) };
        match new.is_null() {
            true  => Err(get_error()),
            false => Ok(wrap_surface(new, true)),
        }
    }

    pub fn display_format(&self) -> Result<~Surface, ~str> {
        let raw = unsafe { ll::SDL_DisplayFormat(self.raw) };

        if raw.is_null() { Err(get_error()) }
        else { Ok(wrap_surface(raw, true)) }
    }

    pub fn display_format_alpha(&self) -> Result<~Surface, ~str> {
        let raw = unsafe { ll::SDL_DisplayFormatAlpha(self.raw) };

        if raw.is_null() { Err(get_error()) }
        else { Ok(wrap_surface(raw, true)) }
    }

    pub fn save_bmp(&self, path: &Path) -> bool {
        path.to_c_str().with_ref(|path| {
            "wb".to_c_str().with_ref(|mode| {
                unsafe {
                    ll::SDL_SaveBMP_RW(self.raw, ll::SDL_RWFromFile(path, mode), 1) == 0
                }
            })
        })
    }

    pub fn set_alpha(&self, flags: &[SurfaceFlag], alpha: u8) -> bool {
        let flags = flags.iter().fold(0u32, |flags, &flag| {
            flags | flag as u32
        });

        unsafe {
            ll::SDL_SetAlpha(self.raw, flags, alpha) == 0
        }
    }

    pub fn set_color_key(&self, flags: &[SurfaceFlag], color: Color) -> bool {
        let flags = flags.iter().fold(0u32, |flags, &flag| {
            flags | flag as u32
        });

        unsafe {
            ll::SDL_SetColorKey(self.raw, flags,
                                color.to_mapped((*self.raw).format)) == 0
        }
    }

    pub fn set_clip_rect(&self, rect: &Rect) {
        unsafe {
            ll::SDL_SetClipRect(self.raw, rect);
        }
    }

    pub fn get_clip_rect(&self) -> Rect {
        let rect = Rect {
            x: 0,
            y: 0,
            w: 0,
            h: 0
        };

        unsafe {
            ll::SDL_SetClipRect(self.raw,
                                cast::transmute(&rect));
        }

        rect
    }

    pub fn blit_rect(&self, src: &Surface, src_rect: Option<Rect>,
                     dest_rect: Option<Rect>) -> bool {
        unsafe {
            ll::SDL_UpperBlit(src.raw, match src_rect {
                Some(ref rect) => cast::transmute(rect),
                None => ptr::null()
            }, self.raw, match dest_rect {
                Some(ref rect) => cast::transmute(rect),
                None => ptr::null()
            }) == 0
        }
    }

    pub fn blit(&self, src: &Surface) -> bool {
        self.blit_rect(src, None, None)
    }

    pub fn blit_at(&self, src: &Surface, x: i16, y: i16) -> bool {
        let (w, h) = src.get_size();

        self.blit_rect(src, None, Some(Rect {
            x: x,
            y: y,
            w: w,
            h: h
        }))
    }

    pub fn fill_rect(&self, rect: Option<Rect>,
                     color: Color) -> bool {
        unsafe { ll::SDL_FillRect(self.raw, match rect {
            Some(ref rect) => cast::transmute(rect),
            None => ptr::null()
        }, color.to_mapped((*self.raw).format)) == 0 }
    }

    pub fn fill(&self, color: Color) -> bool {
        self.fill_rect(None, color)
    }

    pub fn clear(&self) -> bool {
        self.fill(RGB(0, 0, 0))
    }
}

pub fn set_gamma(r: f32, g: f32, b: f32) -> bool {
    unsafe { ll::SDL_SetGamma(r as c_float, g as c_float,
                              b as c_float) != -1 }
}

pub fn set_gamma_ramp(r: Option<[u16, ..256]>, g: Option<[u16, ..256]>,
                      b: Option<[u16, ..256]>) -> bool {
    unsafe { ll::SDL_SetGammaRamp(match r {
        Some(r) => vec::raw::to_ptr(r),
        None => ptr::null()
    }, match g {
        Some(g) => vec::raw::to_ptr(g),
        None => ptr::null()
    }, match b {
        Some(b) => vec::raw::to_ptr(b),
        None => ptr::null()
    }) != -1 }
}

pub fn get_gamma_ramp() -> ([u16, ..256], [u16, ..256], [u16, ..256]) {
    let r = [0u16, .. 256];
    let g = [0u16, .. 256];
    let b = [0u16, .. 256];

    unsafe { ll::SDL_GetGammaRamp(vec::raw::to_ptr(r),
                                  vec::raw::to_ptr(g),
                                  vec::raw::to_ptr(b)); }

    (r, g, b)
}

pub fn swap_buffers() {
    unsafe {
        ll::SDL_GL_SwapBuffers();
    }
}


// TODO: YUV
