use std::mem;
use std::ptr;
use std::str;
use std::ffi::{CStr, CString};

use video;

pub mod ll {
    #![allow(non_camel_case_types)]

    use video::ll::SDL_Surface;

    use libc::{uint8_t, c_int};
    use libc::types::os::arch::c95::c_schar;

    pub type SDL_GrabMode = c_int;

    pub const SDL_GRAB_QUERY: SDL_GrabMode = -1;
    pub const SDL_GRAB_OFF: SDL_GrabMode = 0;
    pub const SDL_GRAB_ON: SDL_GrabMode = 1;
    pub const SDL_GRAB_FULLSCREEN: SDL_GrabMode = 2;

    extern "C" {
        pub fn SDL_WM_SetCaption(title: *const c_schar, icon: *const c_schar);
        pub fn SDL_WM_GetCaption(title: *mut *mut c_schar, icon: *mut *mut c_schar);
        pub fn SDL_WM_SetIcon(icon: *mut SDL_Surface, mask: *mut uint8_t);
        pub fn SDL_WM_IconifyWindow() -> c_int;
        pub fn SDL_WM_ToggleFullScreen(surface: *mut SDL_Surface) -> c_int;
        pub fn SDL_WM_GrabInput(mode: SDL_GrabMode) -> SDL_GrabMode;
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum GrabMode {
     Query = ll::SDL_GRAB_QUERY as isize,
     Off = ll::SDL_GRAB_OFF as isize,
     On = ll::SDL_GRAB_ON as isize
}

pub fn set_caption(title: &str, icon: &str) {
    unsafe {
        ll::SDL_WM_SetCaption(CString::new(title.as_bytes()).unwrap().as_ptr(),
                              CString::new(icon.as_bytes()).unwrap().as_ptr());
    }
}

pub fn get_caption() -> (String, String) {
    let mut title_buf = ptr::null_mut();
    let mut icon_buf = ptr::null_mut();
    let mut title = String::new();
    let mut icon = String::new();

    unsafe {
        ll::SDL_WM_GetCaption(&mut title_buf, &mut icon_buf);

        if !title_buf.is_null() {
            let slice = CStr::from_ptr(mem::transmute_copy(&mut &title_buf)).to_bytes();
            title = str::from_utf8(slice).unwrap().to_string();
        }

        if !icon_buf.is_null() {
            let slice = CStr::from_ptr(mem::transmute_copy(&mut &icon_buf)).to_bytes();
            icon = str::from_utf8(slice).unwrap().to_string();
        }

        (title, icon)
    }
}

pub fn set_icon(surface: video::Surface) {
    unsafe { ll::SDL_WM_SetIcon(surface.raw, ptr::null_mut()); }
}

pub fn iconify_window() {
    unsafe { ll::SDL_WM_IconifyWindow(); }
}

pub fn toggle_fullscreen(surface: video::Surface) {
    unsafe { ll::SDL_WM_ToggleFullScreen(surface.raw); }
}

pub fn grab_input(mode: GrabMode) {
    unsafe { ll::SDL_WM_GrabInput(mode as i32); }
}

pub fn toggle_grab_input() {
    unsafe {
        if ll::SDL_WM_GrabInput(GrabMode::Query as i32) == GrabMode::On as i32 {
            ll::SDL_WM_GrabInput(GrabMode::Off as i32);
        } else {
            ll::SDL_WM_GrabInput(GrabMode::On as i32);
        }
    }
}

pub fn is_grabbing_input() -> bool {
    unsafe { ll::SDL_WM_GrabInput(GrabMode::Query as i32) == GrabMode::On as i32 }
}

// TODO: get_wm_info
