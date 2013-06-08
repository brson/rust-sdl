use std::i16;
use std::rand::RngUtil;
use std::rand;

use sdl;

pub fn main() {
	do sdl::start {
		sdl::init([sdl::InitVideo]);
		sdl::wm::set_caption("rust-sdl demo - video", "rust-sdl");

		let mut rng = rand::rng();
		let screen = match sdl::video::set_video_mode(800, 600, 32, [sdl::video::HWSurface],
			                                                        [sdl::video::DoubleBuf]) {
			Ok(screen) => screen,
			Err(err) => fail!(fmt!("failed to set video mode: %s", err))
		};

		// Note: You'll want to put this and the flip call inside the main loop
		// but we don't as to not startle epileptics
		for i16::range(0, 10) |i| {
			for i16::range(0, 10) |j| {
				screen.fill_rect(Some(sdl::Rect {
					x: i * 800 / 10,
					y: j * 600 / 10,
					w: 800 / 10,
					h: 600 / 10
				}), rng.gen::<sdl::video::Color>());
			}
		}

		screen.flip();

		'main : loop {
			'event : loop {
				match sdl::event::poll_event() {
					sdl::event::QuitEvent => break 'main,
					sdl::event::NoEvent => break 'event,
					_ => {}
				}
			}
		}

		sdl::quit();
	}
}
