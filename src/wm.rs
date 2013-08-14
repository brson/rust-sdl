use std::cast;
use std::ptr;
use std::str;

use video;

pub mod ll {
	use video::ll::SDL_Surface;

	use std::libc::{c_schar, uint8_t, c_int};

	pub type SDL_GrabMode = c_int;

	pub static SDL_GRAB_QUERY: SDL_GrabMode = -1;
	pub static SDL_GRAB_OFF: SDL_GrabMode = 0;
	pub static SDL_GRAB_ON: SDL_GrabMode = 1;
	pub static SDL_GRAB_FULLSCREEN: SDL_GrabMode = 2;

	extern {
		pub fn SDL_WM_SetCaption(title: *c_schar, icon: *c_schar);
	    pub fn SDL_WM_GetCaption(title: **c_schar, icon: **c_schar);
	    pub fn SDL_WM_SetIcon(icon: *SDL_Surface, mask: *uint8_t);
	    pub fn SDL_WM_IconifyWindow() -> c_int;
	    pub fn SDL_WM_ToggleFullScreen(surface: *SDL_Surface) -> c_int;
	    pub fn SDL_WM_GrabInput(mode: SDL_GrabMode) -> SDL_GrabMode;
	}
}

#[deriving(Eq)]
pub enum GrabMode {
	 GrabQuery = ll::SDL_GRAB_QUERY as int,
	 GrabOff = ll::SDL_GRAB_OFF as int,
	 GrabOn = ll::SDL_GRAB_ON as int
}

pub fn set_caption(title: &str, icon: &str) {
	unsafe { ll::SDL_WM_SetCaption(title.to_c_str().unwrap(), icon.to_c_str().unwrap()); }
}

pub fn get_caption() -> (~str, ~str) {
	let title_buf = ptr::null();
	let icon_buf = ptr::null();

	unsafe {
		ll::SDL_WM_GetCaption(&title_buf,
			                  &icon_buf);

        (str::raw::from_c_str(cast::transmute_copy(&title_buf)),
         str::raw::from_c_str(cast::transmute_copy(&icon_buf)))
    }
}

pub fn set_icon(surface: ~video::Surface) {
	unsafe { ll::SDL_WM_SetIcon(surface.raw, ptr::null()); }
}

pub fn iconify_window() {
	unsafe { ll::SDL_WM_IconifyWindow(); }
}

pub fn toggle_fullscreen(surface: ~video::Surface) {
	unsafe { ll::SDL_WM_ToggleFullScreen(surface.raw); }
}

pub fn grab_input(mode: GrabMode) {
	unsafe { ll::SDL_WM_GrabInput(mode as i32); }
}

pub fn toggle_grab_input() {
	unsafe {
		if ll::SDL_WM_GrabInput(GrabQuery as i32) == GrabOn as i32 {
			ll::SDL_WM_GrabInput(GrabOff as i32);
		} else {
			ll::SDL_WM_GrabInput(GrabOn as i32);
		}
	}
}

pub fn is_grabbing_input() -> bool {
	unsafe { ll::SDL_WM_GrabInput(GrabQuery as i32) == GrabOn as i32 }
}

// TODO: get_wm_info
