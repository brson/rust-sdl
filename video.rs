use sdl;
use ll;
use util::Rect;

use core::result::{Result, Err, Ok};
use core::cast::transmute;
use core::cast;
use core::libc::{c_void, c_int, c_char};
use core::str;
use core::vec;

pub enum SurfaceFlag {
    SWSurface = 0x00000000,
    HWSurface = 0x00000001,
    AsyncBlit = 0x00000004,
}

pub enum VideoModeFlag {
    AnyFormat = 0x10000000,
    HWPalette = 0x20000000,
    DoubleBuf = 0x40000000,
    Fullscreen = 0x80000000,
    OpenGL = 0x00000002,
    OpenGLBlit = 0x0000000A,
    Resizable  = 0x00000010,
    NoFrame = 0x00000020,
}

pub struct Surface {
    priv raw_surface: *ll::video::SDL_Surface,
    priv needs_freeing: bool
}

pub impl Surface {
    fn display_format(&self) -> Result<~Surface, ~str> {
        unsafe {
            let raw_surface = ll::video::SDL_DisplayFormat(self.raw_surface);
            if raw_surface == ptr::null() {
                Err(sdl::get_error())
            } else {
                Ok(~Surface{ raw_surface: raw_surface, needs_freeing: true })
            }
        }
    }

    fn flip(&self) -> bool {
        unsafe {
            ll::video::SDL_Flip(self.raw_surface) == 0 as c_int
        }
    }

    /// Locks a surface so that the pixels can be directly accessed safely.
    fn with_lock<R>(&self, f: &fn(pixels: &mut [u8]) -> R) -> R {
        unsafe {
            if ll::video::SDL_LockSurface(self.raw_surface) != 0 { die!(~"Could not lock surface"); }
            let len = (*self.raw_surface).pitch as uint * ((*self.raw_surface).h as uint);
            let pixels: &mut [u8] = transmute(((*self.raw_surface).pixels, len));
            let rv = f(pixels);
            ll::video::SDL_UnlockSurface(self.raw_surface);
            rv
        }
    }

    fn lock(&self) -> bool {
        unsafe {
            return ll::video::SDL_LockSurface(self.raw_surface) == 0 as c_int;
        }
    }

    fn unlock(&self) {
        unsafe {
            ll::video::SDL_UnlockSurface(self.raw_surface);
        }
    }

    fn blit_surface_rect(&self, src: &Surface, srcrect: &Rect, dstrect: &Rect) -> bool {
        unsafe {
            let res = ll::video::SDL_UpperBlit(src.raw_surface,
                                               srcrect,
                                               self.raw_surface,
                                               dstrect);
            return res == 0 as c_int;
        }
    }

    fn blit_surface(&self, src: &Surface) -> bool {
        unsafe {
            let res = ll::video::SDL_UpperBlit(src.raw_surface,
                                               ptr::null(),
                                               self.raw_surface,
                                               ptr::null());
            return res == 0 as c_int;
        }
    }

    fn fill_rect(&self, rect: &Rect, color: u32) -> bool {
        unsafe {
            return ll::video::SDL_FillRect(self.raw_surface, rect, color) == 0 as c_int;
        }
    }

    fn fill(&self, color: u32) -> bool {
        unsafe {
            return ll::video::SDL_FillRect(self.raw_surface, ptr::null(), color) == 0 as c_int;
        }
    }
}

pub impl Surface : Drop {
    pub fn finalize(&self) {
        if self.needs_freeing {
            unsafe {
                ll::video::SDL_FreeSurface(self.raw_surface)
            }
        }
    }
}


pub fn set_video_mode(
    width: int,
    height: int,
    bitsperpixel: int,
    surface_flags: &[SurfaceFlag],
    video_mode_flags: &[VideoModeFlag]) -> Result<~Surface, ~str> {
    let flags = vec::foldl(0u32, surface_flags, |flags, flag| {
        flags | *flag as u32
    });
    let flags = vec::foldl(flags, video_mode_flags, |flags, flag| {
        flags | *flag as u32
    });

    unsafe {
        let raw_surface = ll::video::SDL_SetVideoMode(width as c_int,
                                                      height as c_int,
                                                      bitsperpixel as c_int,
                                                      flags);

        if raw_surface == ptr::null() {
            Err(sdl::get_error())
        } else {
            Ok(~Surface{ raw_surface: raw_surface, needs_freeing: false })
        }
    }
}

pub fn load_bmp(file: &str) -> Result<~Surface, ~str> {
    str::as_buf(file, |buf, _len| {
        let buf = unsafe {
            cast::reinterpret_cast(&buf)
        };
        str::as_buf(~"rb", |rbbuf, _len| {
            unsafe {
                let rbbuf = cast::reinterpret_cast(&rbbuf);
                let raw_bmp = ll::video::SDL_RWFromFile(buf, rbbuf);
                let raw_surface = ll::video::SDL_LoadBMP_RW(raw_bmp, 1 as c_int);
                if raw_surface == ptr::null() {
                    Err(sdl::get_error())
                } else {
                    Ok(~Surface{ raw_surface: raw_surface, needs_freeing: true })
                }
            }
        })
    })
}

pub fn create_rgb_surface(
    surface_flags: &[SurfaceFlag],
    width: int, height: int, bits_per_pixel: int,
    rmask: u32, gmask: u32, bmask: u32, amask: u32) -> Result<~Surface, ~str> {

    let flags = vec::foldl(0u32, surface_flags, |flags, flag| {
        flags | *flag as u32
    });

    unsafe {
        let raw_surface = ll::video::SDL_CreateRGBSurface(
            flags, width as c_int, height as c_int, bits_per_pixel as c_int,
            rmask, gmask, bmask, amask);
        if raw_surface == ptr::null() {
            Err(sdl::get_error())
        } else {
            Ok(~Surface{ raw_surface: raw_surface, needs_freeing: true })
        }
    }
}

pub fn swap_buffers() {
    unsafe {
        ll::video::SDL_GL_SwapBuffers();
    }
}
