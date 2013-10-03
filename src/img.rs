use std::libc::c_int;
use std::ptr;

use get_error;
use video::Surface;

// Setup linking for all targets.
#[cfg(target_os="macos")]
mod mac {
    #[cfg(mac_framework)]
    #[link_args="-framework SDL_image"]
    extern {}

    #[cfg(mac_dylib)]
    #[link_args="-lSDL_image"]
    extern {}
}

#[cfg(not(target_os="macos"))]
mod others {
    #[link_args="-lSDL_image"]
    extern {}
}

pub mod ll {
    use video::ll::SDL_Surface;

    use std::libc::{c_int, c_uint, c_schar};

    pub type IMG_InitFlags = c_uint;

    pub static IMG_INIT_JPG: IMG_InitFlags = 1;
    pub static IMG_INIT_PNG: IMG_InitFlags = 2;
    pub static IMG_INIT_TIF: IMG_InitFlags = 4;
    pub static IMG_INIT_WEBP: IMG_InitFlags = 8;

    externfn!(fn IMG_Init(flags: c_int) -> c_int)
    externfn!(fn IMG_Quit())
    externfn!(fn IMG_Load(file: *c_schar) -> *SDL_Surface)
}

#[deriving(Eq)]
pub enum InitFlag {
    InitJPG = ll::IMG_INIT_JPG as int,
    InitPNG = ll::IMG_INIT_PNG as int,
    InitTIF = ll::IMG_INIT_TIF as int
}

pub fn init(flags: &[InitFlag]) -> ~[InitFlag] {
    let bitflags = unsafe {
        ll::IMG_Init(do flags.iter().fold(0i32) |flags, &flag| {
            flags | flag as c_int
        })
    };

    let flags = [InitJPG,
        InitPNG,
        InitTIF];

    do flags.iter().filter_map |&flag| {
        if bitflags & (flag as c_int) != 0 { Some(flag) }
        else { None }
    }.collect()
}

pub fn load(file: &Path) -> Result<~Surface, ~str> {
    do file.to_c_str().with_ref |file| {
        unsafe {
            let raw = ll::IMG_Load(file);

            if raw == ptr::null() {
                Err(get_error())
            } else {
                Ok(~Surface { raw: raw, owned: true })
            }
        }
    }
}

pub fn quit() {
    unsafe { ll::IMG_Quit(); }
}
