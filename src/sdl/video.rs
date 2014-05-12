use std::mem;
use libc::{c_int, c_float};
use std::ptr;
use std::iter;
use rand::Rng;
use std::slice;

use Rect;
use get_error;

pub mod ll {
    #![allow(non_camel_case_types)]

    use Rect;

    use libc::{c_void, c_uint, c_int, c_float, c_uchar, uint8_t, uint16_t};
    use libc::{uint32_t, int32_t};
    use libc::types::os::arch::c95::c_schar;

    pub type SDL_Rect = Rect;

    pub struct SDL_RWops {
        pub seek: *uint8_t,
        pub read: *uint8_t,
        pub write: *uint8_t,
        pub close: *uint8_t,
        pub _type: uint32_t,
        hidden: [c_uchar, ..24]
    }

    pub struct SDL_Surface {
        pub flags: uint32_t,
        pub format: *SDL_PixelFormat,
        pub w: c_int,
        pub h: c_int,
        pub pitch: uint16_t,
        pub pixels: *c_void,
        pub offset: c_int,
        pub hwdata: *c_void,
        pub clip_rect: SDL_Rect,
        pub unused1: uint32_t,
        pub locked: uint32_t,
        pub map: *c_void,
        pub format_version: c_uint,
        pub refcount: c_int
    }

    pub struct SDL_Color {
        pub r: uint8_t,
        pub g: uint8_t,
        pub b: uint8_t,
        pub unused: uint8_t
    }

    pub struct SDL_Palette {
        pub ncolors: c_int,
        pub colors: *SDL_Color,
    }

    #[allow(uppercase_variables)]
    pub struct SDL_PixelFormat {
        pub palette: *SDL_Palette,
        pub BitsPerPixel: uint8_t,
        pub BytesPerPixel: uint8_t,
        pub Rloss: uint8_t,
        pub Gloss: uint8_t,
        pub Bloss: uint8_t,
        pub Aloss: uint8_t,
        pub Rshift: uint8_t,
        pub Gshift: uint8_t,
        pub Bshift: uint8_t,
        pub Ashift: uint8_t,
        pub Rmask: uint32_t,
        pub Gmask: uint32_t,
        pub Bmask: uint32_t,
        pub Amask: uint32_t,
        pub colorkey: uint32_t,
        pub alpha: uint8_t,
    }

    pub struct SDL_VideoInfo {
        pub flags: uint32_t,        // actually a set of packed fields
        pub video_mem: uint32_t,
        pub vfmt: *SDL_PixelFormat,
        pub current_w: c_int,
        pub current_h: c_int,
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
    pub raw: *ll::SDL_Surface,
    pub owned: bool
}

fn wrap_surface(raw: *ll::SDL_Surface, owned: bool) -> Surface {
    Surface{ raw: raw, owned: owned }
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
#[allow(raw_pointer_deriving)]
pub struct Palette {
    pub raw: *ll::SDL_Palette
}

fn wrap_palette(palette: *ll::SDL_Palette) -> Option<Palette> {
    if palette.is_null() {
        None
    } else {
        Some(Palette { raw: palette })
    }
}

pub type PaletteColors<'a> =
    iter::Map<'a, &'a ll::SDL_Color, Color, slice::Items<'a, ll::SDL_Color>>;

impl Palette {
    pub fn colors<'a>(&'a self) -> PaletteColors<'a> {
        use std::{raw, mem};
        let colors: &'a [ll::SDL_Color] = unsafe {
            mem::transmute(raw::Slice { data: (*self.raw).colors,
                                         len: (*self.raw).ncolors as uint })
        };
        colors.iter().map(|color| Color::from_struct(color))
    }
}

#[deriving(Eq)]
pub struct PixelFormat {
    pub palette: Option<Palette>,
    pub bpp: u8,
    pub r_loss: u8,
    pub g_loss: u8,
    pub b_loss: u8,
    pub a_loss: u8,
    pub r_shift: u8,
    pub g_shift: u8,
    pub b_shift: u8,
    pub a_shift: u8,
    pub r_mask: u32,
    pub g_mask: u32,
    pub b_mask: u32,
    pub a_mask: u32,
    pub color_key: u32,
    pub alpha: u8
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
        palette: match fmt.palette {
            None => ptr::null(),
            Some(palette) => palette.raw
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

#[deriving(Eq, TotalEq)]
pub enum Color {
    RGB(u8, u8, u8),
    RGBA(u8, u8, u8, u8)
}

impl ::rand::Rand for Color {
    fn rand<R: ::rand::Rng>(rng: &mut R) -> Color {
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

#[deriving(Eq, TotalEq)]
pub enum SurfaceFlag {
    SWSurface = 0x00000000,
    HWSurface = 0x00000001,
    AsyncBlit = 0x00000004,
    SrcColorKey = 0x00001000,
    SrcAlpha = 0x00010000,
    RLEAccel = 0x00004000
}

#[deriving(Eq, TotalEq)]
pub enum VideoFlag {
    AnyFormat = 0x10000000u,
    HWPalette = 0x20000000u,
    DoubleBuf = 0x40000000u,
    Fullscreen = 0x80000000u,
    OpenGL = 0x00000002u,
    OpenGLBlit = 0x0000000Au,
    Resizable = 0x00000010u,
    NoFrame = 0x00000020u
}

pub fn set_video_mode(w: int, h: int, bpp: int,
                      surface_flags: &[SurfaceFlag],
                      video_flags: &[VideoFlag]) -> Result<Surface, ~str> {
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

#[deriving(Eq, TotalEq)]
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
    pub flags: Vec<VideoInfoFlag>,
    pub width: int,
    pub height: int,
    pub format: PixelFormat,
}

fn wrap_video_info_flags(bitflags: u32) -> Vec<VideoInfoFlag> {
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

pub fn get_video_info() -> VideoInfo {
    let raw = unsafe { ll::SDL_GetVideoInfo() };
    VideoInfo {
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

pub fn get_video_surface() -> Result<Surface, ~str> {
    let raw = unsafe { ll::SDL_GetVideoSurface() };

    if raw.is_null() { Err(get_error()) }
    else { Ok(wrap_surface(raw, false)) }
}

// TODO: get_video_modes, get_video_driver_name

impl Surface {
    pub fn new(surface_flags: &[SurfaceFlag], width: int, height: int, bpp: int,
               rmask: u32, gmask: u32, bmask: u32, amask: u32) -> Result<Surface, ~str> {
        let flags = surface_flags.iter().fold(0u32, |flags, flag| { flags | *flag as u32 });

        unsafe {
            let raw = ll::SDL_CreateRGBSurface(flags, width as c_int, height as c_int, bpp as c_int,
                                               rmask, gmask, bmask, amask);

            if raw == ptr::null() {
                Err(get_error())
            } else {
                Ok(Surface { raw: raw, owned: true })
            }
        }
    }

    pub fn from_bmp(path: &Path) -> Result<Surface, ~str> {
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
                                mem::transmute(rects.as_ptr()));
        }
    }

    #[allow(deprecated_owned_vector)]
    pub fn set_colors(&self, colors: &[Color]) -> bool {
        let colors: Vec<_> = colors.iter().map(|color| {
            color.to_struct()
        }).collect();

        unsafe { ll::SDL_SetColors(self.raw, colors.as_ptr(), 0,
                                   colors.len() as c_int) == 1 }
    }

    #[allow(deprecated_owned_vector)]
    pub fn set_palette(&self, palettes: &[PaletteType],
                   colors: &[Color]) -> bool {
        let colors: Vec<_> = colors.iter().map(|color| {
            color.to_struct()
        }).collect();
        let flags = palettes.iter().fold(0 as c_int, |flags, &flag| {
            flags | flag as c_int
        });

        unsafe { ll::SDL_SetPalette(self.raw, flags,
                                    colors.as_ptr(), 0,
                                    colors.len() as c_int) == 1 }
    }

    pub fn lock(&self) -> bool {
        unsafe { ll::SDL_LockSurface(self.raw) == 0 }
    }

    /// Locks a surface so that the pixels can be directly accessed safely.
    pub fn with_lock<R>(&self, f: |&mut [u8]| -> R) -> R {
        unsafe {
            if ll::SDL_LockSurface(self.raw) != 0 { fail!("could not lock surface"); }
            let len = (*self.raw).pitch as uint * ((*self.raw).h as uint);
            let pixels: &mut [u8] = mem::transmute(((*self.raw).pixels, len));
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

    pub fn convert(&self, fmt: &PixelFormat, flags: &[SurfaceFlag]) -> Result<Surface, ~str> {
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

    pub fn display_format(&self) -> Result<Surface, ~str> {
        let raw = unsafe { ll::SDL_DisplayFormat(self.raw) };

        if raw.is_null() { Err(get_error()) }
        else { Ok(wrap_surface(raw, true)) }
    }

    pub fn display_format_alpha(&self) -> Result<Surface, ~str> {
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
                                mem::transmute(&rect));
        }

        rect
    }

    pub fn blit_rect(&self, src: &Surface, src_rect: Option<Rect>,
                     dest_rect: Option<Rect>) -> bool {
        unsafe {
            ll::SDL_UpperBlit(src.raw, match src_rect {
                Some(ref rect) => mem::transmute(rect),
                None => ptr::null()
            }, self.raw, match dest_rect {
                Some(ref rect) => mem::transmute(rect),
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
            Some(ref rect) => mem::transmute(rect),
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
        Some(r) => r.as_ptr(),
        None => ptr::null()
    }, match g {
        Some(g) => g.as_ptr(),
        None => ptr::null()
    }, match b {
        Some(b) => b.as_ptr(),
        None => ptr::null()
    }) != -1 }
}

pub fn get_gamma_ramp() -> ([u16, ..256], [u16, ..256], [u16, ..256]) {
    let r = [0u16, .. 256];
    let g = [0u16, .. 256];
    let b = [0u16, .. 256];

    unsafe { ll::SDL_GetGammaRamp(r.as_ptr(),
                                  g.as_ptr(),
                                  b.as_ptr()); }

    (r, g, b)
}

pub fn swap_buffers() {
    unsafe {
        ll::SDL_GL_SwapBuffers();
    }
}


// TODO: YUV
