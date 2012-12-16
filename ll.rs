/*!

Low-level bindings

*/

use libc::{c_char, c_int, c_void};
use core::libc::types::common::c99::{uint32_t, int32_t, uint16_t};

type c_enum = c_int;
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
    pub const SDL_INIT_EVENTTHREAD: sdl_init_flag   = 0x01000000;
    pub const SDL_INIT_EVERYTHING: sdl_init_flag    = 0x0000FFFF;

    extern mod SDL{ // Note: Rust chokes if this is unspecified, whereas it's fine elsewhere. Go figure.
        fn SDL_Init(flags: sdl_init_flag) -> c_int;
        fn SDL_InitSubSystem(flags: sdl_init_flag) -> c_int;
        fn SDL_QuitSubSystem(flags: sdl_init_flag);
        fn SDL_Quit();
        fn SDL_WasInit(flags: sdl_init_flag) -> sdl_init_flag;
    }
}

/*pub mod event {

    extern {
        fn SDL_PollEvent(event: *RawEvent) -> int32_t;
    }
}

pub mod keyboard {
}
*/
pub mod video {

    type RWOps = c_void;

    pub type Surface = {
        flags: sdl_flag,
        format: *c_void,
        w: c_int,
        h: c_int,
        pitch: uint16_t,
        pixels: *c_void,
        offset: c_int 
        // FIXME: Remaining are unlisted
    };

    pub type sdl_flag = uint32_t;
    pub type sdl_surface_flag = sdl_flag;

    pub const SDL_SWSURFACE: sdl_surface_flag = 0x00000000;
    pub const SDL_HWSURFACE: sdl_surface_flag = 0x00000001;
    pub const SDL_ASYNCBLIT: sdl_surface_flag = 0x00000004;

    pub type sdl_video_mode_flag = sdl_flag;

    pub const SDL_ANYFORMAT: sdl_video_mode_flag  = 0x10000000;
    pub const SDL_HWPALETTE: sdl_video_mode_flag  = 0x20000000;
    pub const SDL_DOUBLEBUF: sdl_video_mode_flag  = 0x40000000;
    pub const SDL_FULLSCREEN: sdl_video_mode_flag = 0x80000000;
    pub const SDL_OPENGL: sdl_video_mode_flag     = 0x00000002;
    pub const SDL_OPENGLBLIT: sdl_video_mode_flag = 0x0000000A;
    pub const SDL_RESIZABLE: sdl_video_mode_flag  = 0x00000010;
    pub const SDL_NOFRAME: sdl_video_mode_flag    = 0x00000020;

    extern {
        fn SDL_SetVideoMode(width: c_int, height: c_int, bitsperpixel: c_int, flags: sdl_flag) -> *Surface;
        fn SDL_FreeSurface(surface: *Surface);
        fn SDL_LoadBMP_RW(src: *RWOps, freesrc: c_int) -> *Surface;
        fn SDL_RWFromFile(file: *c_char, mode: *c_char) -> *RWOps;
        fn SDL_DisplayFormat(surface: *Surface) -> *Surface;
        fn SDL_UpperBlit(src: *Surface, srcrect: *util::Rect,
                         dst: *Surface, dstrect: *util::Rect) -> c_int;
        fn SDL_Flip(screen: *Surface) -> c_int;
        fn SDL_CreateRGBSurface(flags: sdl_flag, width: c_int, height: c_int,
                                bitsPerPixel: c_int,
                                Rmask: uint32_t, Gmask: uint32_t, Bmask: uint32_t, Amask: uint32_t) -> *Surface;
        fn SDL_FillRect(dst: *Surface, dstrect: *util::Rect, color: uint32_t) -> c_int;
        fn SDL_LockSurface(surface: *Surface) -> c_int;
        fn SDL_UnlockSurface(surface: *Surface) -> c_int;
    }
}
