fn on_osmain(f: fn~()) {
    let builder = task::builder();
    task::set_opts(builder, {
        sched: some({
            mode: task::osmain,
            native_stack_size: none
        })
        with task::get_opts(builder)
    });
    let po = comm::port();
    let ch = comm::chan(po);
    task::run(builder) {||
        f();
        comm::send(ch, ());
    }
    comm::recv(po);
}

// FIXME: Needs additional cocoa setup on OS X. rust-cocoa should probably just
// be a dependency
#[test]
#[ignore(cfg(target_os = "macos"))]
fn test_everything() {

    on_osmain() {||
        init([init_video, init_timer]);
        run_tests([
            general::test_was_init,
            general::test_set_error,
            general::test_error,
            general::test_clear_error,
            video::test_set_video_mode,
            // FIXME: Doesn't work when called from a directory that
            // doesn't contain the test image file
            //video::test_blit,
            test_event::test_poll_event_none
            // FIXME: This test is interactive
            //test_event::test_keyboard
        ]);
        quit();
    }
}

fn run_tests(tests: [fn()]) {
    vec::iter(tests) {|test| test()}
}

mod general {
    fn test_was_init() {
        assert vec::contains(was_init([init_timer]), init_timer);
    }

    fn test_set_error() {
        set_error("test");
        assert get_error() == "test";
    }

    fn test_error() {
        clear_error();
        assert str::is_empty(get_error());
        error(enomem);
        assert str::is_not_empty(get_error());
    }

    fn test_clear_error() {
        set_error("test");
        clear_error();
        assert str::is_empty(get_error());
    }
}

mod test_event {
    fn test_poll_event_none() {
        ::event::poll_event {|event|
            assert event == ::event::no_event;
        }
    }

    fn test_keyboard() {
        io::println("press a key in the window");
        let surface = ::video::set_video_mode(320, 200, 32,
            [::video::swsurface], [::video::doublebuf, ::video::resizable]);
        let mut keydown = false;
        let mut keyup = false;
        while !keydown || !keyup {
            ::event::poll_event {|event|
                alt event {
                  event::keyup_event(_) { keyup = true; }
                  event::keydown_event(_) { keydown = true; }
                  _ { }
                }
            }
        }
        ::video::free_surface(surface);
    }
}

mod video {

    fn test_set_video_mode() {
        let surface = ::video::set_video_mode(320, 200, 32,
            [::video::hwsurface], [::video::doublebuf]);
        assert surface != ptr::null();
        ::video::free_surface(surface);
    }

    fn test_blit() {
        let screen = ::video::set_video_mode(320, 200, 32,
            [::video::swsurface], [::video::doublebuf]);
        assert screen != ptr::null();

        let image = {
            // FIXME: We need to load this from the crate instead of
            // off the filesystem
            let tmp = ::video::load_bmp("rust-logo-128x128-blk.bmp");
            assert tmp != ptr::null();
            let image = ::video::display_format(tmp);
            assert image != ptr::null();
            ::video::free_surface(tmp);
            image
        };

        iter::repeat(1u) {||
            ::video::blit_surface(image, ptr::null(),
                                  screen, ptr::null());
            ::video::flip(screen);
            ::event::poll_event {|_event|
            }
        }

        ::video::free_surface(image);
        ::video::free_surface(screen);
    }
}
