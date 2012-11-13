/*!

Low-level bindings

*/

use libc::{c_char};
use core::libc::types::common::c99::{uint32_t};

type c_enum = uint32_t;
type rust_enum = uint;

pub mod error {
    pub type sdl_error_flag = c_enum;
    
    pub const SDL_ENOMEM: sdl_error_flag      = 0;
    pub const SDL_EFREAD: sdl_error_flag      = 1;
    pub const SDL_EFWRITE: sdl_error_flag     = 2;
    pub const SDL_EFSEEK: sdl_error_flag      = 3;
    pub const SDL_UNSUPPORTED: sdl_error_flag = 4;

    extern {
        fn SDL_Error(code: sdl_error_flag);
        // FIXME: This is actually a varargs call
        fn SDL_SetError(fmt: *c_char);
        fn SDL_GetError() -> *c_char;
        fn SDL_ClearError();
    }
}

pub mod sdl {
    pub type sdl_init_flag = uint32_t;
    
    pub const SDL_INIT_TIMER: sdl_init_flag         = 0x00000001;
    pub const SDL_INIT_AUDIO: sdl_init_flag         = 0x00000010;
    pub const SDL_INIT_VIDEO: sdl_init_flag         = 0x00000020;
    pub const SDL_INIT_CDROM: sdl_init_flag         = 0x00000100;
    pub const SDL_INIT_JOYSTICK: sdl_init_flag      = 0x00000200;
    pub const SDL_INIT_HAPTIC: sdl_init_flag        = 0x00001000;
    pub const SDL_INIT_NO_PARACHUTE: sdl_init_flag  = 0x00100000;
    pub const SDL_INIT_EVENT_THREAD: sdl_init_flag  = 0x01000000;
    pub const SDL_INIT_EVERYTHING: sdl_init_flag    = 0x0000FFFF;

    extern {
        fn SDL_Init(flags: uint32_t) -> c_int;
        fn SDL_InitSubSystem(flags: uint32_t) -> c_int;
        fn SDL_QuitSubSystem(flags: uint32_t);
        fn SDL_Quit();
        fn SDL_WasInit(flags: uint32_t) -> c_int;
    }
}
