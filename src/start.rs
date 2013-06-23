use std::cell::Cell;
use std::task::PlatformThread;
use std::task;


pub type MainFunction = ~fn();

pub fn start(main: MainFunction) {
    let cell = Cell::new(main);
    let mut task = task::task();
    task.sched_mode(PlatformThread);
    task.spawn_with(cell, platform_specific::run_main)
}

#[cfg(target_os="win32")]
#[cfg(target_os="linux")]
#[cfg(target_os="freebsd")]
mod platform_specific {
    use std::cell::Cell;
    use super::MainFunction;

    pub fn run_main(cell: Cell<MainFunction>) {
        cell.take()()
    }
}

#[cfg(target_os="macos")]
mod platform_specific {
    use std::cell::Cell;
    use super::MainFunction;
    use std::cast::transmute;
    use std::libc::{c_int, c_char};
    use std::local_data;
    use std::os;

    fn key(_: @MainFunction) {}

    pub fn run_main(cell: Cell<MainFunction>) {
        let args = os::args();
        unsafe {
            local_data::local_data_set(key, @cell.take());

            // XXX: Use return value to set program return code.
            // XXX: This isn't really safe... args might not be null-
            // terminated.
            let c_args = args.map(|s| -> *c_char { transmute(&s[0]) });
            let _ = SDLX_main(args.len() as c_int, &c_args[0]);
        }
    }

    #[no_mangle]
    extern fn SDL_main(_: c_int, _: **c_char) {
        unsafe {
            (*local_data::local_data_get(key).get())();
        }
    }

    #[link_args="-L. -lSDLXmain -framework AppKit -framework Foundation"]
    extern {
        fn SDLX_main(argc: c_int, argv: **c_char) -> c_int;
    }
}