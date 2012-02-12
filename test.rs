#[test]
fn test_everything() {
    init([init_everything]);
    run_tests([
        general::test_was_init,
        general::test_set_error,
        general::test_error,
        general::test_clear_error,
        video::test_set_video_mode
    ]);
    quit();
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

mod video {

    fn test_set_video_mode() {
        let surface = ::video::set_video_mode(320, 200, 32,
            [::video::hwsurface], [::video::doublebuf]);
        assert surface != ptr::null();
        ::video::free_surface(surface);
    }
}
