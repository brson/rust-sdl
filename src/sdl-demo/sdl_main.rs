//! Specialized demo of SDL for use on platforms that really want to
//! initialize and own the main routine and thread. This is meant to be
//! linked with object code compiled from one of the SDL_main variants
//! that one can find beneath <SDL-distribution>/src/main/.
//!
//! For example on Mac OS X one can build a runnable program from this
//! with something along the lines of:
//!
//!   rustc src/sdl-demo/sdl_main.rs -L. \
//!       -C link-args="-lSDLmain -lSDL -Wl,-framework,Cocoa"

#![no_main]

extern crate sdl;
extern crate rand;

use rand::Rng;

use sdl::video::{SurfaceFlag, VideoFlag};
use sdl::event::{Event, Key};

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn SDL_main() {
    real_main()
}

pub fn real_main() {
    sdl::init(&[sdl::InitFlag::Video]);
    sdl::wm::set_caption("rust-sdl demo - video", "rust-sdl");

    let mut rng = rand::thread_rng();
    let screen = match sdl::video::set_video_mode(800, 600, 32,
                                                  &[SurfaceFlag::HWSurface],
                                                  &[VideoFlag::DoubleBuf]) {
        Ok(screen) => screen,
        Err(err) => panic!("failed to set video mode: {}", err)
    };

    // Note: You'll want to put this and the flip call inside the main loop
    // but we don't as to not startle epileptics
    for i in 0usize..10 {
        for j in 0usize..10 {
            screen.fill_rect(Some(sdl::Rect {
                x: (i as i16) * 800 / 10,
                y: (j as i16) * 600 / 10,
                w: 800 / 10,
                h: 600 / 10
            }), rng.gen::<sdl::video::Color>());
        }
    }

    screen.flip();

    'main : loop {
        'event : loop {
            match sdl::event::poll_event() {
                Event::Quit => break 'main,
                Event::None => break 'event,
                Event::Key(k, _, _, _)
                    if k == Key::Escape
                        => break 'main,
                _ => {}
            }
        }
    }

    sdl::quit();
}
