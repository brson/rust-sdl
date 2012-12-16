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

pub mod event {

    type sdl_event_type = u8; //FIXME: Should this be c_enum?

    const SDL_NOEVENT: sdl_event_type = 0u8;
    const SDL_ACTIVEEVENT: sdl_event_type = 1u8;
    const SDL_KEYDOWN: sdl_event_type = 2u8;
    const SDL_KEYUP: sdl_event_type = 3u8;
    const SDL_MOUSEMOTION: sdl_event_type = 4u8;
    const SDL_MOUSEBUTTONDown: sdl_event_type = 5u8;
    const SDL_MOUSEBUTTONUP: sdl_event_type = 6u8;
    const SDL_JOYAXISMOTION: sdl_event_type = 7u8;
    const SDL_JOYBALLMOTION: sdl_event_type = 8u8;
    const SDL_JOYHATMOTION: sdl_event_type = 9u8;
    const SDL_JOYBUTTONDOWN: sdl_event_type = 10u8;
    const SDL_JOYBUTTONUP: sdl_event_type = 11u8;
    const SDL_QUIT: sdl_event_type = 12u8;
    const SDL_SYSWMEVENT: sdl_event_type = 13u8;
    const SDL_EVENT_RESERVEDA: sdl_event_type = 14u8;
    const SDL_EVENT_RESERVEDB: sdl_event_type = 15u8;
    const SDL_VIDEORESIZE: sdl_event_type = 16u8;
    const SDL_VIDEOEXPOSE: sdl_event_type = 17u8;
    const SDL_EVENT_RESERVED2: sdl_event_type = 18u8;
    const SDL_EVENT_RESERVED3: sdl_event_type = 19u8;
    const SDL_EVENT_RESERVED4: sdl_event_type = 20u8;
    const SDL_EVENT_RESERVED5: sdl_event_type = 21u8;
    const SDL_EVENT_RESERVED6: sdl_event_type = 22u8;
    const SDL_EVENT_RESERVED7: sdl_event_type = 23u8;
    const SDL_EVENT_USEREVENT: sdl_event_type = 24u8;

    extern {
        fn SDL_PollEvent(event: *RawEvent) -> c_int;
    }
}
/*
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
