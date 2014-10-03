use std::cell::Cell;
use std::task;


pub type MainFunction = Box<fn()>;

pub fn start(main: MainFunction) {
}

#[cfg(any(target_os="win32", target_os="linux", target_os="freebsd")]
mod platform_specific {
    use std::cell::Cell;
    use super::MainFunction;

    pub fn run_main(cell: Cell<MainFunction>) {
        cell.take()()
    }
}

#[cfg(target_os="macos")]
pub mod platform_specific {
    #[link_args="-L. -lSDLmain -framework AppKit -framework Foundation"]
    extern {
    }
}
