use libc::{c_int};

pub mod ll {
    #![allow(non_camel_case_types)]

    use libc::{c_int};

    extern "C" {
        pub fn SDL_GL_SetAttribute(attr: c_int, value: c_int) -> c_int;
        pub fn SDL_GL_SwapBuffers();
    }
}

#[derive(Copy, Clone)]
pub enum GLAttribute {
    RedSize = 0,
    GreenSize,
    BlueSize,
    AlphaSize,
    BufferSize,
    DoubleBuffer,
    DepthSize,
    StencilSize,
    AccumRedSize,
    AccumGreenSize,
    AccumBlueSize,
    AccumAlphaSize,
    Stereo,
    MultiSampleBuffers,
    MultiSampleSamples,
    AcceleratedVisual,
    SwapControl
}

pub fn set_attribute(attr: GLAttribute, value: isize) -> isize {
    unsafe {
        ll::SDL_GL_SetAttribute(attr as c_int, value as c_int) as isize
    }
}

pub fn swap_buffers() {
    unsafe {
        ll::SDL_GL_SwapBuffers();
    }
}
