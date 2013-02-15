use core::cast::transmute;
use core::libc::{c_char, c_int};
use core::task::PlatformThread;
use core::task::local_data;
use core::task;
use std::cell::Cell;

type MainFunction = ~fn();

fn key(_: @MainFunction) {}

#[no_mangle]
#[cfg(target_os="macos")]
pub extern fn SDL_main(_: c_int, _: **c_char) {
    unsafe {
        (*local_data::local_data_get(key).get())();
    }
}

#[cfg(target_os="macos")]
pub fn start(main: MainFunction) {
    let cell = Cell(main);
    do task::task().sched_mode(PlatformThread).spawn {
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
}

#[cfg(target_os="win32")]
#[cfg(target_os="linux")]
#[cfg(target_os="freebsd")]
pub fn start(main: MainFunction) {
    let cell = Cell(main);
    do task::task().sched_mode(PlatformThread).spawn {
        cell.take()();
    }
}

#[cfg(target_os="macos")]
#[link_args="-L. -lSDLXmain -framework AppKit -framework Foundation"]
extern {
    fn SDLX_main(argc: c_int, argv: **c_char) -> c_int;
}
