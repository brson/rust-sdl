extern crate sdl;

use std::rand::Rng;


#[main]
pub fn main() {
    sdl::init([sdl::InitVideo]);
    sdl::wm::set_caption("rust-sdl demo - video", "rust-sdl");

    let mut rng = std::rand::task_rng();
    let screen = match sdl::video::set_video_mode(800, 600, 32, [sdl::video::HWSurface],
                                                                [sdl::video::DoubleBuf]) {
        Ok(screen) => screen,
        Err(err) => fail!("failed to set video mode: {}", err)
    };

    // Note: You'll want to put this and the flip call inside the main loop
    // but we don't as to not startle epileptics
    for i in range(0u, 10) {
        for j in range(0u, 10) {
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
