use get_error;
use video::Surface;

use core::libc::c_int;

pub mod ll {
    use video::ll::SDL_Surface;

    use core::libc::{c_int, c_uint, c_schar};

    pub type IMG_InitFlags = c_uint;

    pub static IMG_INIT_JPG: IMG_InitFlags = 1;
    pub static IMG_INIT_PNG: IMG_InitFlags = 2;
    pub static IMG_INIT_TIF: IMG_InitFlags = 4;
    pub static IMG_INIT_WEBP: IMG_InitFlags = 8;

    #[link_args = "-lSDL_image"]
    pub extern {
        fn IMG_Init(flags: c_int) -> c_int;
        fn IMG_Quit();
        fn IMG_Load(file: *c_schar) -> *SDL_Surface;
    }
}

#[deriving(Eq)]
pub enum InitFlag {
    InitJPG = ll::IMG_INIT_JPG as int,
    InitPNG = ll::IMG_INIT_PNG as int,
    InitTIF = ll::IMG_INIT_TIF as int
}

pub fn init(flags: &[InitFlag]) -> ~[InitFlag] {
    let bitflags = unsafe {
        ll::IMG_Init(do vec::foldl(0, flags) |flags, &flag| {
            flags | flag as c_int
        })
    };

    do [InitJPG,
        InitPNG,
        InitTIF].filter_mapped |&flag| {
        if bitflags & (flag as c_int) != 0 { Some(flag) }
        else { None }
    }
}

pub fn load(file: &Path) -> Result<~Surface, ~str> {
    str::as_buf(file.to_str(), |buf, _len| {
        let file = unsafe {
            cast::reinterpret_cast(&buf)
        };

        unsafe {
            let raw = ll::IMG_Load(file);

            if raw == ptr::null() {
                Err(get_error())
            } else {
                Ok(~Surface { raw: raw, owned: true })
            }
        }
    })
}

pub fn quit() {
    unsafe { ll::IMG_Quit(); }
}
