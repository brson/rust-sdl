# Rust-SDL
Bindings for SDL in Rust
# Overview

Rust-SDL is a library for talking to SDL from Rust. Low-level C components are wrapped in Rust code to make them more idiomatic and abstract away inappropriate manual memory management.

In addition, it provides optional APIs no a number of common SDL extension libraries.

Rust-SDL uses the MIT license.

# Requirements

* *Rust* - we currently compile against the *Incoming* branch, but the code often works against *Master*. The releases on http://www.rust-lang.org tend to not work.
* *SDL 1.2 development libraries* - install through your favourite package management tool, or via http://www.libsdl.org/

**Optional**
* *SDL_Mixer* and *SDL_Image* - also available through most package managers, or through http://www.libsdl.org/projects/SDL_mixer/ and http://www.libsdl.org/projects/SDL_image/

# Installation
Clone this repo, run `./configure`, and then `make`. To see an example of the code in use, *make demos*.
If you don't want SDL_Mixer (if you aren't using sound) or SDL_Image (if you're not using image formats other than BMP) then you can exclude them from make by setting `RUSTFLAGS`. For example, if you wish to use just SDL_Image, `RUSTFLAGS="--cfg image" make`

# When things go wrong
Rust, and Rust-SDL, are both still heavily in development, and you may run into teething issues when using this. Before panicking, check that you're using the latest Incoming branch of Rust, check that you've updated Rust-SDL to the latest version, and run `make clean` and `./configure`. If that fails, please let us know on the issue tracker.
