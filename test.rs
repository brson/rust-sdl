fn on_osmain(f: fn~()) {
    let po = comm::Port();
    let ch = comm::Chan(po);
    do task::spawn_sched(task::PlatformThread) |copy f| {
        f();
        comm::send(ch, ());
    }
    comm::recv(po);
}

// FIXME: Needs additional cocoa setup on OS X. rust-cocoa should probably just
// be a dependency
#[test]
#[ignore(cfg(target_os = "macos"))]
pub fn test_everything() {

    on_osmain(|| {
        init(~[InitVideo, InitTimer]);
        run_tests(~[
            general::test_was_init,
            general::test_set_error,
            general::test_error,
            general::test_clear_error,
            video::test_set_video_mode,
            // FIXME: Doesn't work when called from a directory that
            // doesn't contain the test image file
            //video::test_blit,
            test_event::test_poll_event_none,
            // FIXME: This test is interactive
            //test_event::test_keyboard,
        ]);
        quit();
    })
}

fn run_tests(tests: ~[fn()]) {
    for tests.each |test| {
        (*test)();
    }
}

mod general {
    pub fn test_was_init() {
        assert vec::contains(was_init(~[InitTimer]), InitTimer);
    }

    pub fn test_set_error() {
        set_error(~"test");
        assert get_error() == ~"test";
    }

    pub fn test_error() {
        clear_error();
        assert str::is_empty(get_error());
        error(ENoMem);
        assert str::is_not_empty(get_error());
    }

    pub fn test_clear_error() {
        set_error(~"test");
        clear_error();
        assert str::is_empty(get_error());
    }
}

mod test_event {
    pub fn test_poll_event_none() {
        ::event::poll_event(|event| assert event == ::event::NoEvent);
    }

    pub fn test_keyboard() {
        io::println(~"press a key in the window");
        let surface = ::video::set_video_mode(320, 200, 32,
            ~[::video::SWSurface], ~[::video::DoubleBuf, ::video::Resizable]);
        let mut keydown = false;
        let mut keyup = false;
        while !keydown || !keyup {
            ::event::poll_event(|event| {
                match event {
                  event::KeyUpEvent(_) => keyup = true,
                  event::KeyDownEvent(_) => keydown = true,
                  _ => { }
                }
            })
        }
        ::video::free_surface(surface);
    }
}

mod video {

    pub fn test_set_video_mode() {
        let surface = ::video::set_video_mode(320, 200, 32,
            ~[::video::HWSurface], ~[::video::DoubleBuf]);
        assert surface != ptr::null();
        ::video::free_surface(surface);
    }

    pub fn test_blit() {
        let screen = ::video::set_video_mode(320, 200, 32,
            ~[::video::SWSurface], ~[::video::DoubleBuf]);
        assert screen != ptr::null();

        let image = {
            // FIXME: We need to load this from the crate instead of
            // off the filesystem
            let tmp = ::video::load_bmp(~"rust-logo-128x128-blk.bmp");
            assert tmp != ptr::null();
            let image = ::video::display_format(tmp);
            assert image != ptr::null();
            ::video::free_surface(tmp);
            image
        };

        for iter::repeat(1u) || {
            ::video::blit_surface(image, ptr::null(),
                                  screen, ptr::null());
            ::video::flip(screen);
            ::event::poll_event(|_event| {})
        };

        ::video::free_surface(image);
        ::video::free_surface(screen);
    }
}
