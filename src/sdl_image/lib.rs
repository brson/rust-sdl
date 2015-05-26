extern crate libc;
extern crate sdl;

use std::path::Path;
use std::ffi::CString;
use libc::c_int;

use sdl::get_error;
use sdl::video::Surface;

// Setup linking for all targets.
#[cfg(any(not(target_os = "macos"), not(mac_framework)))]
#[link(name = "SDL_image")]
extern {}

#[cfg(all(target_os = "macos", mac_framework))]
#[link(name = "SDL_image", kind = "framework")]
extern {}

pub mod ll {
    #![allow(non_camel_case_types)]

    use sdl::video::ll::SDL_Surface;

    use libc::{c_int, c_uint};
    use libc::types::os::arch::c95::c_schar;

    pub type IMG_InitFlags = c_uint;

    pub const IMG_INIT_JPG: IMG_InitFlags = 1;
    pub const IMG_INIT_PNG: IMG_InitFlags = 2;
    pub const IMG_INIT_TIF: IMG_InitFlags = 4;
    pub const IMG_INIT_WEBP: IMG_InitFlags = 8;

    extern "C" {
        pub fn IMG_Init(flags: c_int) -> c_int;
        pub fn IMG_Quit();
        pub fn IMG_Load(file: *const c_schar) -> *mut SDL_Surface;
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum InitFlag {
    JPG = ll::IMG_INIT_JPG as isize,
    PNG = ll::IMG_INIT_PNG as isize,
    TIF = ll::IMG_INIT_TIF as isize
}

pub fn init(flags: &[InitFlag]) -> Vec<InitFlag> {
    let bitflags = unsafe {
        ll::IMG_Init(flags.iter().fold(0i32, |flags, &flag| {
            flags | flag as c_int
        }))
    };

    let flags = [InitFlag::JPG,
        InitFlag::PNG,
        InitFlag::TIF];

    flags.iter().filter_map(|&flag| {
        if bitflags & (flag as c_int) != 0 { Some(flag) }
        else { None }
    }).collect()
}

pub fn load(file: &Path) -> Result<Surface, String> {
    let cfile = CString::new(file.to_str().unwrap()).unwrap();
    unsafe {
        let raw = ll::IMG_Load(cfile.as_ptr());

        if raw.is_null() {
            Err(get_error())
        } else {
            Ok(Surface { raw: raw, owned: true })
        }
    }
}

pub fn quit() {
    unsafe { ll::IMG_Quit(); }
}
