use std::cell::Cell;
use std::task::PlatformThread;
use std::task;
use std::libc::{c_int, c_char};
use std::cast::transmute;

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
    let mut task = task::task();
    task.sched_mode(PlatformThread);
    do task.spawn {
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
    let mut task = task::task();
    task.sched_mode(PlatformThread);
    do task.spawn {
        cell.take()();
    }
}

#[cfg(target_os="macos")]
#[link_args="-L. -lSDLXmain -framework AppKit -framework Foundation"]
extern {
    fn SDLX_main(argc: c_int, argv: **c_char) -> c_int;
}
