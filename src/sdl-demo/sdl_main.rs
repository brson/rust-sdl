//! Specialized demo of SDL for use on platforms that really want to
//! initialize and own the main routine and thread.  This is mean to be
//! linked with object code compiled from one of the SDL_main variants
//! that one can find beneath <SDL-distribution>/src/main/.
//!
//! For example on Mac OS X on can build an runnable program from this
//! with something along the lines of:
//!
//!   rustc sdl_main.rs -L. -L<SDL-lib> -C link-args=" -I<SDL-include>/SDL   \
//!      -framework CoreFoundation -framework CoreGraphics -framework AppKit \
//!      <SDL-distribution>/src/main/macosx/SDLMain.m "
//!
//! where <SDL-lib> is the directory holding the libSDL.a,
//!       <SDL-include>/SDL is a directory holding headers like SDL.h
//!   and <SDL-distribution> is, as above, the root of SDL 1.2 source download.

#![no_main]

extern crate native;
extern crate rand;
extern crate sdl;

use rand::Rng;

#[no_mangle]
pub extern "C" fn SDL_main(argc: int, argv: **u8) {
    native::start(argc, argv, real_main);
}

pub fn real_main() {
    sdl::init([sdl::InitVideo]);
    sdl::wm::set_caption("rust-sdl demo - video", "rust-sdl");

    let mut rng = rand::task_rng();
    let screen = match sdl::video::set_video_mode(800, 600, 32, [sdl::video::HWSurface],
                                                                [sdl::video::DoubleBuf]) {
        Ok(screen) => screen,
        Err(err) => panic!("failed to set video mode: {}", err)
    };

    // Note: You'll want to put this and the flip call inside the main loop
    // but we don't as to not startle epileptics
    for i in range(0, 10) {
        for j in range(0, 10) {
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
                sdl::event::QuitEvent => break 'main,
                sdl::event::NoEvent => break 'event,
                sdl::event::KeyEvent(k, _, _, _)
                    if k == sdl::event::EscapeKey
                        => break 'main,
                _ => {}
            }
        }
    }

    sdl::quit();
}
