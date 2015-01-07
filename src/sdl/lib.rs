#![allow(raw_pointer_derive)]

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

