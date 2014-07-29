use std::mem;
use libc::c_int;
use std::string;

use get_error;

pub mod ll {
	#![allow(non_camel_case_types)]

	use libc::{c_void, c_int, uint8_t, int16_t};
	use libc::types::os::arch::c95::c_schar;

	pub type SDL_Joystick = c_void;

    extern "C" {
        pub fn SDL_NumJoysticks() -> c_int;
        pub fn SDL_JoystickName(i: c_int) -> *const c_schar;
        pub fn SDL_JoystickOpen(i: c_int) -> *mut SDL_Joystick;
        pub fn SDL_JoystickOpened(i: c_int) -> c_int;
        pub fn SDL_JoystickIndex(joystick: *mut SDL_Joystick) -> c_int;
        pub fn SDL_JoystickNumAxes(joystick: *mut SDL_Joystick) -> c_int;
        pub fn SDL_JoystickNumBalls(joystick: *mut SDL_Joystick) -> c_int;
        pub fn SDL_JoystickNumHats(joystick: *mut SDL_Joystick) -> c_int;
        pub fn SDL_JoystickNumButtons(joystick: *mut SDL_Joystick) -> c_int;
        pub fn SDL_JoystickUpdate();
        pub fn SDL_JoystickEventState(state: c_int) -> c_int;
        pub fn SDL_JoystickGetAxis(joystick: *mut SDL_Joystick, axis: c_int) -> int16_t;
        pub fn SDL_JoystickGetHat(joystick: *mut SDL_Joystick, hat: c_int) -> uint8_t;
        pub fn SDL_JoystickGetBall(joystick: *mut SDL_Joystick, ball: c_int, dx: *mut c_int, dy: *mut c_int)
                        -> c_int;
        pub fn SDL_JoystickGetButton(joystick: *mut SDL_Joystick, button: c_int) -> uint8_t;
        pub fn SDL_JoystickClose(joystick: *mut SDL_Joystick);
    }
}

pub fn get_num_joysticks() -> int {
	unsafe { ll::SDL_NumJoysticks() as int }
}

pub fn get_joystick_name(index: int) -> String {
	unsafe {
		let cstr = ll::SDL_JoystickName(index as c_int);

		string::raw::from_buf(mem::transmute_copy(&cstr))
	}
}

pub fn is_joystick_open(index: int) -> bool {
	unsafe { ll::SDL_JoystickOpened(index as c_int) == 1 }
}

pub fn update_joysticks() {
	unsafe { ll::SDL_JoystickUpdate(); }
}

#[deriving(PartialEq)]
pub struct Joystick {
	pub raw: *mut ll::SDL_Joystick
}

fn wrap_joystick(raw: *mut ll::SDL_Joystick) -> Joystick {
	Joystick { raw: raw }
}

impl Joystick {
	pub fn open(index: int) -> Result<Joystick, String> {
		unsafe {
			let raw = ll::SDL_JoystickOpen(index as c_int);

			if raw.is_null() { Err(get_error()) }
			else { Ok(wrap_joystick(raw)) }
		}
	}

	pub fn get_index(&self) -> int {
		unsafe { ll::SDL_JoystickIndex(self.raw) as int }
	}

	pub fn get_num_axes(&self) -> int {
		unsafe { ll::SDL_JoystickNumAxes(self.raw) as int }
	}

	pub fn get_num_balls(&self) -> int {
		unsafe { ll::SDL_JoystickNumBalls(self.raw) as int }
	}

	pub fn get_num_hats(&self) -> int {
		unsafe { ll::SDL_JoystickNumHats(self.raw) as int }
	}

	pub fn get_num_buttons(&self) -> int {
		unsafe { ll::SDL_JoystickNumButtons(self.raw) as int }
	}

	pub fn get_axis(&self, axis: int) -> i16 {
		unsafe { ll::SDL_JoystickGetAxis(self.raw, axis as c_int) as i16 }
	}

	pub fn get_hat(&self, hat: int) -> u8 {
		unsafe { ll::SDL_JoystickGetAxis(self.raw, hat as c_int) as u8 }
	}

	pub fn get_button(&self, button: int) -> u8 {
		unsafe { ll::SDL_JoystickGetButton(self.raw, button as c_int) as u8 }
	}

	pub fn get_ball(&self, ball: int) -> (int, int) {
		let mut dx = 0;
		let mut dy = 0;

		unsafe { ll::SDL_JoystickGetBall(self.raw, ball as c_int,
			                             &mut dx,
			                             &mut dy); }

		(dx as int, dy as int)
	}
}

impl Drop for Joystick {
    fn drop(&mut self) {
        unsafe { ll::SDL_JoystickClose(self.raw); }
    }
}
