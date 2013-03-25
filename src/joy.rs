use get_error;

use core::libc::c_int;

pub mod ll {
	use core::libc::{c_void, c_int, c_schar, uint8_t, int16_t};

	pub type SDL_Joystick = c_void;

	pub extern {
		fn SDL_NumJoysticks() -> c_int;
		fn SDL_JoystickName(i: c_int) -> *c_schar;
	    fn SDL_JoystickOpen(i: c_int) -> *SDL_Joystick;
	    fn SDL_JoystickOpened(i: c_int) -> c_int;
	    fn SDL_JoystickIndex(joystick: *SDL_Joystick) -> c_int;
	    fn SDL_JoystickNumAxes(joystick: *SDL_Joystick) -> c_int;
	    fn SDL_JoystickNumBalls(joystick: *SDL_Joystick) -> c_int;
	    fn SDL_JoystickNumHats(joystick: *SDL_Joystick) -> c_int;
	    fn SDL_JoystickNumButtons(joystick: *SDL_Joystick) -> c_int;
	    fn SDL_JoystickUpdate();
	    fn SDL_JoystickEventState(state: c_int) -> c_int;
	    fn SDL_JoystickGetAxis(joystick: *SDL_Joystick, axis: c_int) -> int16_t;
	    fn SDL_JoystickGetHat(joystick: *SDL_Joystick, hat: c_int) -> uint8_t;
	    fn SDL_JoystickGetBall(joystick: *SDL_Joystick, ball: c_int,
	                           dx: *c_int, dy: *c_int) -> c_int;
	    fn SDL_JoystickGetButton(joystick: *SDL_Joystick, button: c_int) -> uint8_t;
	    fn SDL_JoystickClose(joystick: *SDL_Joystick);
	}
}

pub fn get_num_joysticks() -> int {
	unsafe { ll::SDL_NumJoysticks() as int }
}

pub fn get_joystick_name(index: int) -> ~str {
	unsafe {
		let cstr = ll::SDL_JoystickName(index as c_int);

		str::raw::from_c_str(cast::reinterpret_cast(&cstr))
	}
}

pub fn is_joystick_open(index: int) -> bool {
	unsafe { ll::SDL_JoystickOpened(index as c_int) == 1 }
}

pub fn update_joysticks() {
	unsafe { ll::SDL_JoystickUpdate(); }
}

#[deriving(Eq)]
pub struct Joystick {
	pub raw: *ll::SDL_Joystick
}

fn wrap_joystick(raw: *ll::SDL_Joystick) -> ~Joystick {
	~Joystick { raw: raw }
}

pub impl Joystick {
	fn open(index: int) -> Result<~Joystick, ~str> {
		unsafe {
			let raw = ll::SDL_JoystickOpen(index as c_int);

			if raw.is_null() { Err(get_error()) }
			else { Ok(wrap_joystick(raw)) }
		}
	}

	fn get_index(&self) -> int {
		unsafe { ll::SDL_JoystickIndex(self.raw) as int }
	}

	fn get_num_axes(&self) -> int {
		unsafe { ll::SDL_JoystickNumAxes(self.raw) as int }
	}

	fn get_num_balls(&self) -> int {
		unsafe { ll::SDL_JoystickNumBalls(self.raw) as int }
	}

	fn get_num_hats(&self) -> int {
		unsafe { ll::SDL_JoystickNumHats(self.raw) as int }
	}

	fn get_num_buttons(&self) -> int {
		unsafe { ll::SDL_JoystickNumButtons(self.raw) as int }
	}

	fn get_axis(&self, axis: int) -> i16 {
		unsafe { ll::SDL_JoystickGetAxis(self.raw, axis as c_int) as i16 }
	}

	fn get_hat(&self, hat: int) -> u8 {
		unsafe { ll::SDL_JoystickGetAxis(self.raw, hat as c_int) as u8 }
	}

	fn get_button(&self, button: int) -> u8 {
		unsafe { ll::SDL_JoystickGetButton(self.raw, button as c_int) as u8 }
	}

	fn get_ball(&self, ball: int) -> (int, int) {
		let dx = 0;
		let dy = 0;

		unsafe { ll::SDL_JoystickGetBall(self.raw, ball as c_int,
			                             ptr::addr_of(&dx),
			                             ptr::addr_of(&dy)); }

		(dx as int, dy as int)
	}
}

impl Drop for Joystick {
    pub fn finalize(&self) {
        unsafe { ll::SDL_JoystickClose(self.raw); }
    }
}
