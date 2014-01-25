#[crate_id="sdl#0.3.1"];
#[comment = "SDL bindings"];
#[license = "MIT"];
#[crate_type = "lib"];

#[feature(globs)];
#[feature(link_args)];

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

