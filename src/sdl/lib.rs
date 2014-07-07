#![crate_name = "sdl"]
#![comment = "SDL bindings"]
#![license = "MIT"]
#![crate_type = "lib"]

#![feature(globs)]
#![allow(raw_pointer_deriving)]

extern crate libc;
extern crate rand;

pub use sdl::*;

pub mod audio;
pub mod cd;
pub mod event;
pub mod joy;
pub mod mouse;
pub mod video;
pub mod gl;
pub mod wm;

pub mod sdl;

