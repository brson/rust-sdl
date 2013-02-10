// FIXME: Needs additional cocoa setup on OS X. rust-cocoa should probably just
// be a dependency
use sdl::*;
use start;

#[test]
pub fn test_everything() {
    do start::start {
        start_tests();
    }
}

fn start_tests() {
    assert init(~[InitVideo, InitTimer]) == true;
    run_tests(~[
        general::test_was_init,
        // FIXME: Busted, segfault
        //general::test_set_error,
        //general::test_error,
        //general::test_clear_error,
        test_video::test_set_video_mode,
        test_video::test_with_lock,
        // FIXME: Doesn't work when called from a directory that
        // doesn't contain the test image file
        //test_video::test_blit,
        test_event::test_poll_event_none,
        // FIXME: This test is interactive
        //test_event::test_keyboard,
        
    ]);

    quit();
}

fn run_tests(tests: &[extern fn()]) {
    for tests.each |test| {
        (*test)();
    }
}

mod general {

    use sdl::*;

    pub fn test_was_init() {
        assert vec::contains(~[InitTimer], &InitTimer);
    }

    pub fn test_set_error() {
        set_error(~"test");
        assert get_error() == ~"test";
    }

    pub fn test_error() {
        clear_error();
        assert str::is_empty(get_error());
        error(ENoMem);
        assert (str::is_empty(get_error()) == false);
    }

    pub fn test_clear_error() {
        set_error(~"test");
        clear_error();
        assert str::is_empty(get_error());
    }
}

mod test_event {

    use event;
    use video;
    use keyboard;

    pub fn test_poll_event_none() {
        assert event::poll_event() == event::NoEvent;
    }

    pub fn test_keyboard() {
        io::println(~"press a key in the window");
        let maybe_surface = video::set_video_mode(320, 200, 32,
            ~[video::SWSurface], ~[video::DoubleBuf, video::Resizable]);

        match maybe_surface {
            result::Ok(_) => {
                let mut keydown = false;
                let mut keyup = false;
                while !keydown || !keyup {
                    match event::poll_event() {
                      event::KeyUpEvent(keyboard) => {
                          keyup = true;
                          assert keyboard.keycode != keyboard::SDLKUnknown;
                      },
                      event::KeyDownEvent(keyboard) => {
                          keydown = true;
                          assert keyboard.keycode != keyboard::SDLKUnknown;
                      },
                      event::QuitEvent => die!(~"Explicit quit"),
                      _ => { }
                    }
                }
            }
            result::Err(_) => {
                die!(~"Failed to set video mode");
            }
        }
    }
}

mod test_video {

    pub fn test_set_video_mode() {
        let maybe_surface = ::video::set_video_mode(320, 200, 32,
            ~[::video::HWSurface], ~[::video::DoubleBuf]);

        assert result::is_ok(&maybe_surface);
    }

    pub fn test_blit() {
        let maybe_screen = ::video::set_video_mode(320, 200, 32,
            ~[::video::SWSurface], ~[::video::DoubleBuf]);

        assert result::is_ok(&maybe_screen);

        match maybe_screen {
            result::Ok(screen) => {
                // FIXME: We need to load this from the crate instead of
                // off the filesystem
                let maybe_image = match ::video::load_bmp(~"rust-logo-128x128-blk.bmp") {
                    result::Ok(raw_image) => {
                        raw_image.display_format()
                    },
                    result::Err(err) => result::Err(err)
                };

                assert result::is_ok(&maybe_image);

                match maybe_image {
                    result::Ok(image) => {
                        for iter::repeat(1u) || {
                            screen.blit_surface(image);
                            screen.flip();
                            ::event::poll_event();
                        };
                    },
                    result::Err(_) => ()
                };
            },
            result::Err(_) => ()
        };
    }

    pub fn test_with_lock() {
        let maybe_screen = ::video::set_video_mode(320, 200, 32,
            ~[::video::SWSurface], ~[::video::DoubleBuf]);

        assert result::is_ok(&maybe_screen);

        match maybe_screen {
            result::Ok(screen) => {
                //Set all the pixels to green
                do screen.with_lock |raw| {
                    for uint::range(0, raw.len() / 4) |idx| {
                        let true_idx = idx * 4;
                        raw[true_idx] = 0;
                        raw[true_idx + 1] = 0xFF;
                        raw[true_idx + 2] = 0;
                        raw[true_idx + 3] = 0xFF;
                    }
                }
                //Then check that it makes sense
                do screen.with_lock |raw| {
                    //Check the 23rd pixel
                    let start = 23 * 4;
                    assert raw[start] == 0;
                    assert raw[start + 1] == 0xFF;
                    assert raw[start + 2] == 0;
                    assert raw[start + 3] == 0xFF;
                }
            },
            result::Err(message) => die!(message)
        };
    }
}
