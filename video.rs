use libc::{c_void, c_int, c_char};
use core::result::{Result, Err, Ok};

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
}

impl Surface {

    fn display_format() -> Result<~Surface, ~str> {
        let raw_surface = ll::video::SDL_DisplayFormat(self.raw_surface);
        if raw_surface == ptr::null() {
            Err(sdl::get_error())
        } else {
            Ok(~Surface{ raw_surface: raw_surface })
        }
    }

    fn flip() -> bool {
        ll::video::SDL_Flip(self.raw_surface) == 0 as c_int
    }

    fn lock() -> bool {
        return ll::video::SDL_LockSurface(self.raw_surface) == 0 as c_int;
    }

    fn unlock() -> bool {
        return ll::video::SDL_UnlockSurface(self.raw_surface) == 0 as c_int;
    }

    fn blit_surface_rect(src: &Surface, srcrect: &util::Rect, dstrect: &util::Rect) -> bool {
        let res = ll::video::SDL_UpperBlit(src.raw_surface, srcrect, self.raw_surface, dstrect);
        return res == 0 as c_int;
    }

    fn blit_surface(src: &Surface) -> bool {
        let res = ll::video::SDL_UpperBlit(src.raw_surface, ptr::null(), self.raw_surface, ptr::null());
        return res == 0 as c_int;
    }

    fn fill_rect(rect: &util::Rect, color: u32) -> bool {
        return ll::video::SDL_FillRect(self.raw_surface, rect, color) == 0 as c_int;
    }

    fn fill(color: u32) -> bool {
        return ll::video::SDL_FillRect(self.raw_surface, ptr::null(), color) == 0 as c_int;
    }
}

impl Surface : Drop {

    fn finalize(&self) {
        ll::video::SDL_FreeSurface(self.raw_surface)
    }
}


//FIXME: This needs to be called multiple times on window resize, so Drop is going to do bad things, possibly. Test it out.
//Consider making videomode surfaces their own type, with a reset method?
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

    let raw_surface = ll::video::SDL_SetVideoMode(width as c_int, height as c_int, bitsperpixel as c_int, flags);

    if raw_surface == ptr::null() {
        Err(sdl::get_error())
    } else {
        Ok(~Surface{ raw_surface: raw_surface })
    }
}

pub fn load_bmp(file: &str) -> Result<~Surface, ~str> unsafe {
    str::as_buf(file, |buf, _len| {
        let buf = cast::reinterpret_cast(&buf);
        str::as_buf(~"rb", |rbbuf, _len| {
            let rbbuf = cast::reinterpret_cast(&rbbuf);
            let raw_surface = ll::video::SDL_LoadBMP_RW(ll::video::SDL_RWFromFile(buf, rbbuf), 1 as c_int);
            if raw_surface == ptr::null() {
                Err(sdl::get_error())
            } else {
                Ok(~Surface{ raw_surface: raw_surface })
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

    let raw_surface = ll::video::SDL_CreateRGBSurface(
        flags, width as c_int, height as c_int, bits_per_pixel as c_int,
        rmask, gmask, bmask, amask);
    if raw_surface == ptr::null() {
        Err(sdl::get_error())
    } else {
        Ok(~Surface{ raw_surface: raw_surface })
    }
}
