/*!

Low-level bindings

*/

use libc::{c_char, c_int};
use core::libc::types::common::c99::{uint32_t, int32_t};

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
        fn SDL_Init(flags: uint32_t) -> int32_t;
        fn SDL_InitSubSystem(flags: uint32_t) -> int32_t;
        fn SDL_QuitSubSystem(flags: uint32_t);
        fn SDL_Quit();
        fn SDL_WasInit(flags: uint32_t) -> int32_t;
    }
}

/*pub mod event {

    extern {
        fn SDL_PollEvent(event: *RawEvent) -> int32_t;
    }
}

pub mod keyboard {
}

pub mod video {

    pub type sdl_surface_flag = uint32_t;

    pub const SDL_SWSURFACE: sdl_surface_flag = 0x00000000;
    pub const SDL_HWSURFACE: sdl_surface_flag = 0x00000001;
    pub const SDL_ASYNCBLIT: sdl_surface_flag = 0x00000004;

    pub type sdl_video_mode_flag = uint32_t;

    pub const SDL_ANYFORMAT: sdl_video_flag  = 0x10000000;
    pub const SDL_HWPALETTE: sdl_video_flag  = 0x20000000;
    pub const SDL_DOUBLEBUF: sdl_video_flag  = 0x40000000;
    pub const SDL_FULLSCREEN: sdl_video_flag = 0x80000000;
    pub const SDL_OPENGL: sdl_video_flag     = 0x00000002;
    pub const SDL_OPENGLBLIT: sdl_video_flag = 0x0000000A;
    pub const SDL_RESIZABLE: sdl_video_flag  = 0x00000010;
    pub const SDL_NOFRAME: sdl_video_flag    = 0x00000020;

    extern {
        fn SDL_SetVideoMode(width: c_int, height: c_int, bitsperpixel: c_int, flags: u32) -> *Surface;
        fn SDL_FreeSurface(surface: *Surface);
        fn SDL_LoadBMP_RW(src: *RWOps, freesrc: c_int) -> *Surface;
        fn SDL_RWFromFile(file: *c_char, mode: *c_char) -> *RWOps;
        fn SDL_DisplayFormat(surface: *Surface) -> *Surface;
        fn SDL_UpperBlit(src: *Surface, srcrect: *Rect,
                         dst: *Surface, dstrect: *Rect) -> c_int;
        fn SDL_Flip(screen: *Surface) -> c_int;
        fn SDL_CreateRGBSurface(flags: u32, width: c_int, height: c_int,
                                bitsPerPixel: c_int,
                                Rmask: u32, Gmask: u32, Bmask: u32, Amask: u32) -> *Surface;
        fn SDL_FillRect(dst: *Surface, dstrect: *Rect, color: u32);
        fn SDL_LockSurface(surface: *Surface);
        fn SDL_UnlockSurface(surface: *Surface);
    }
}*/
