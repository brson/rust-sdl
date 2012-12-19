// FIXME: Needs additional cocoa setup on OS X. rust-cocoa should probably just
// be a dependency

#[test]
#[ignore(cfg(target_os = "macos"))]
pub fn test_everything() {

    do task::spawn {
        assert sdl::init(~[sdl::InitVideo, sdl::InitTimer]) == true;
        run_tests(~[
            general::test_was_init,
            general::test_set_error,
            general::test_error,
            general::test_clear_error,
            video::test_set_video_mode,
            // FIXME: Doesn't work when called from a directory that
            // doesn't contain the test image file
            video::test_blit, //Comment out before merge
            test_event::test_poll_event_none,
            // FIXME: This test is interactive
            test_event::test_keyboard, //Comment out before merge
            
        ]);
        sdl::quit();
    }
}

fn run_tests(tests: &[fn()]) {
    for tests.each |test| {
        (*test)();
    }
}

mod general {
    pub fn test_was_init() {
        assert vec::contains(~[sdl::InitTimer], &sdl::InitTimer);
    }

    pub fn test_set_error() {
        sdl::set_error(~"test");
        assert sdl::get_error() == ~"test";
    }

    pub fn test_error() {
        sdl::clear_error();
        assert str::is_empty(sdl::get_error());
        sdl::error(sdl::ENoMem);
        assert str::is_not_empty(sdl::get_error());
    }

    pub fn test_clear_error() {
        sdl::set_error(~"test");
        sdl::clear_error();
        assert str::is_empty(sdl::get_error());
    }
}

mod test_event {
    pub fn test_poll_event_none() {
        ::event::poll_event(|event| assert event == ::event::NoEvent);
    }

    pub fn test_keyboard_event_conversion() {
        //TODO: Implement me!
    }

    pub fn test_keyboard() {
        io::println(~"press a key in the window");
        let maybe_surface = ::video::set_video_mode(320, 200, 32,
            ~[::video::SWSurface], ~[::video::DoubleBuf, ::video::Resizable]);

        match maybe_surface {
            result::Ok(_) => {
                let mut keydown = false;
                let mut keyup = false;
                while !keydown || !keyup {
                    ::event::poll_event(|event| {
                        match event {
                          event::KeyUpEvent(_) => keyup = true,
                          event::KeyDownEvent(event) => keydown = true,
                          _ => { }
                        }
                    })
                }
            }
            result::Err(_) => {
                fail;
            }
        }
    }
}

mod video {

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
                            ::event::poll_event(|_event| {})
                        };
                    },
                    result::Err(_) => ()
                };
            },
            result::Err(_) => ()
        };
    }
}
