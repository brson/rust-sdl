/* vim: set ts=4 sw=4 expandtab */

use core::libc::{c_char, c_int};
use video;
use video::Surface;
use ll;
use ll::img;
use sdl;

pub fn load_img(file: &str) -> Result<~video::Surface, ~str> {
    str::as_buf(file, |buf, _len| {
            let file = unsafe {
                cast::reinterpret_cast(&buf)
            };
            unsafe {
                let raw_surface = ll::img::SDL_image::IMG_Load(file);
                if raw_surface == ptr::null() {
                Err(sdl::get_error())
            } else {
                Ok(video::Surface::from_raw(raw_surface))
            }
        }
    })
}
