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

    pub type sdl_button_state = u8;

    pub const SDL_RELEASED: sdl_button_state = 0u8;
    pub const SDL_PRESSED: sdl_button_state = 1u8;

    pub type sdl_event_type = u8;

    pub const SDL_NOEVENT: sdl_event_type = 0u8;
    pub const SDL_ACTIVEEVENT: sdl_event_type = 1u8;
    pub const SDL_KEYDOWN: sdl_event_type = 2u8;
    pub const SDL_KEYUP: sdl_event_type = 3u8;
    pub const SDL_MOUSEMOTION: sdl_event_type = 4u8;
    pub const SDL_MOUSEBUTTONDOWN: sdl_event_type = 5u8;
    pub const SDL_MOUSEBUTTONUP: sdl_event_type = 6u8;
    pub const SDL_JOYAXISMOTION: sdl_event_type = 7u8;
    pub const SDL_JOYBALLMOTION: sdl_event_type = 8u8;
    pub const SDL_JOYHATMOTION: sdl_event_type = 9u8;
    pub const SDL_JOYBUTTONDOWN: sdl_event_type = 10u8;
    pub const SDL_JOYBUTTONUP: sdl_event_type = 11u8;
    pub const SDL_QUIT: sdl_event_type = 12u8;
    pub const SDL_SYSWMEVENT: sdl_event_type = 13u8;
    pub const SDL_EVENT_RESERVEDA: sdl_event_type = 14u8;
    pub const SDL_EVENT_RESERVEDB: sdl_event_type = 15u8;
    pub const SDL_VIDEORESIZE: sdl_event_type = 16u8;
    pub const SDL_VIDEOEXPOSE: sdl_event_type = 17u8;
    pub const SDL_EVENT_RESERVED2: sdl_event_type = 18u8;
    pub const SDL_EVENT_RESERVED3: sdl_event_type = 19u8;
    pub const SDL_EVENT_RESERVED4: sdl_event_type = 20u8;
    pub const SDL_EVENT_RESERVED5: sdl_event_type = 21u8;
    pub const SDL_EVENT_RESERVED6: sdl_event_type = 22u8;
    pub const SDL_EVENT_RESERVED7: sdl_event_type = 23u8;
    pub const SDL_EVENT_USEREVENT: sdl_event_type = 24u8;

    pub type Event = {
        type_: sdl_event_type,
        // FIXME: Not sure exactly how big this needs to be
        event: (u64, u64, u64, u64, u64, u64, u64, u64,
                u64, u64, u64, u64, u64, u64, u64, u64,
                u64, u64, u64, u64, u64, u64, u64, u64)
    };

    extern {
        fn SDL_PollEvent(event: *Event) -> c_int;
    }
}

pub mod keyboard {
    // sym corresponds to the `Key` enum, mod_ to the `Mod` enum. We should
    // be using the correct type here but our enums don't have the right
    // size yet
    pub type KeySym = {
        scancode: u8,
        sym: c_int,
        mod_: c_int,
        unicode: u16
    };
}

pub mod keysym {
    
    /** What we really want is a mapping of every raw key on the keyboard.
     *  To support international keyboards, we use the range 0xA1 - 0xFF
     *  as international virtual keycodes.  We'll follow in the footsteps of X11...
     *  @brief The names of the keys
     */
    pub type sdl_key_type = c_int;
        /** @name ASCII mapped KeySyms
        *  The keyboard syms have been cleverly chosen to map to ASCII
        */
        /*@{*/
        pub SDLK_UNKNOWN         = 0,
        //pub SDLK_First         = 0,
        pub SDLK_BACKSPACE       = 8,
        pub SDLK_TAB             = 9,
        pub SDLK_CLEAR           = 12,
        pub SDLK_RETURN          = 13,
        pub SDLK_PAUSE           = 19,
        pub SDLK_ESCAPE          = 27,
        pub SDLK_SPACE           = 32,
        pub SDLK_EXCLAIM         = 33,
        pub SDLK_QUOTEDBL        = 34,
        pub SDLK_HASH            = 35,
        pub SDLK_DOLLAR          = 36,
        pub SDLK_AMPERSAND       = 38,
        pub SDLK_QUOTE           = 39,
        pub SDLK_LEFTPAREN       = 40,
        pub SDLK_RIGHTPAREN      = 41,
        pub SDLK_ASTERISK        = 42,
        pub SDLK_PLUS            = 43,
        pub SDLK_COMMA           = 44,
        pub SDLK_MINUS           = 45,
        pub SDLK_PERIOD          = 46,
        pub SDLK_SLASH           = 47,
        pub SDLK_0               = 48,
        pub SDLK_1               = 49,
        pub SDLK_2               = 50,
        pub SDLK_3               = 51,
        pub SDLK_4               = 52,
        pub SDLK_5               = 53,
        pub SDLK_6               = 54,
        pub SDLK_7               = 55,
        pub SDLK_8               = 56,
        pub SDLK_9               = 57,
        pub SDLK_COLON           = 58,
        pub SDLK_SEMICOLON       = 59,
        pub SDLK_LESS            = 60,
        pub SDLK_EQUALS          = 61,
        pub SDLK_GREATER         = 62,
        pub SDLK_QUESTION        = 63,
        pub SDLK_AT              = 64,
        /* 
        Skip uppercase letters
        */
        pub SDLK_LEFTBRACKET     = 91,
        pub SDLK_BACKSLASH       = 92,
        pub SDLK_RIGHTBRACKEt    = 93,
        pub SDLK_CARET           = 94,
        pub SDLK_UNDERSCORE      = 95,
        pub SDLK_BACKQUOTE       = 96,
        pub SDLK_A               = 97,
        pub SDLK_B               = 98,
        pub SDLK_C               = 99,
        pub SDLK_D               = 100,
        pub SDLK_E               = 101,
        pub SDLK_F               = 102,
        pub SDLK_G               = 103,
        pub SDLK_H               = 104,
        pub SDLK_I               = 105,
        pub SDLK_J               = 106,
        pub SDLK_K               = 107,
        pub SDLK_L               = 108,
        pub SDLK_M               = 109,
        pub SDLK_N               = 110,
        pub SDLK_O               = 111,
        pub SDLK_P               = 112,
        pub SDLK_Q               = 113,
        pub SDLK_R               = 114,
        pub SDLK_S               = 115,
        pub SDLK_T               = 116,
        pub SDLK_U               = 117,
        pub SDLK_V               = 118,
        pub SDLK_W               = 119,
        pub SDLK_X               = 120,
        pub SDLK_Y               = 121,
        pub SDLK_Z               = 122,
        pub SDLK_DELETE          = 127,
        /* End of ASCII mapped KeySyms */
        /*@}*/

        /** @name International keyboard syms */
        /*@{*/
        pub SDLK_WORLD0          = 160,      /* 0xA0 */
        pub SDLK_WORLD1          = 161,
        pub SDLK_WORLD2          = 162,
        pub SDLK_WORLD3          = 163,
        pub SDLK_WORLD4          = 164,
        pub SDLK_WORLD5          = 165,
        pub SDLK_WORLD6          = 166,
        pub SDLK_WORLD7          = 167,
        pub SDLK_WORLD8          = 168,
        pub SDLK_WORLD9          = 169,
        pub SDLK_WORLD10         = 170,
        pub SDLK_WORLD11         = 171,
        pub SDLK_WORLD12         = 172,
        pub SDLK_WORLD13         = 173,
        pub SDLK_WORLD14         = 174,
        pub SDLK_WORLD15         = 175,
        pub SDLK_WORLD16         = 176,
        pub SDLK_WORLD17         = 177,
        pub SDLK_WORLD18         = 178,
        pub SDLK_WORLD19         = 179,
        pub SDLK_WORLD20         = 180,
        pub SDLK_WORLD21         = 181,
        pub SDLK_WORLD22         = 182,
        pub SDLK_WORLD23         = 183,
        pub SDLK_WORLD24         = 184,
        pub SDLK_WORLD25         = 185,
        pub SDLK_WORLD26         = 186,
        pub SDLK_WORLD27         = 187,
        pub SDLK_WORLD28         = 188,
        pub SDLK_WORLD29         = 189,
        pub SDLK_WORLD30         = 190,
        pub SDLK_WORLD31         = 191,
        pub SDLK_WORLD32         = 192,
        pub SDLK_WORLD33         = 193,
        pub SDLK_WORLD34         = 194,
        pub SDLK_WORLD35         = 195,
        pub SDLK_WORLD36         = 196,
        pub SDLK_WORLD37         = 197,
        pub SDLK_WORLD38         = 198,
        pub SDLK_WORLD39         = 199,
        pub SDLK_WORLD40         = 200,
        pub SDLK_WORLD41         = 201,
        pub SDLK_WORLD42         = 202,
        pub SDLK_WORLD43         = 203,
        pub SDLK_WORLD44         = 204,
        pub SDLK_WORLD45         = 205,
        pub SDLK_WORLD46         = 206,
        pub SDLK_WORLD47         = 207,
        pub SDLK_WORLD48         = 208,
        pub SDLK_WORLD49         = 209,
        pub SDLK_WORLD50         = 210,
        pub SDLK_WORLD51         = 211,
        pub SDLK_WORLD52         = 212,
        pub SDLK_WORLD53         = 213,
        pub SDLK_WORLD54         = 214,
        pub SDLK_WORLD55         = 215,
        pub SDLK_WORLD56         = 216,
        pub SDLK_WORLD57         = 217,
        pub SDLK_WORLD58         = 218,
        pub SDLK_WORLD59         = 219,
        pub SDLK_WORLD60         = 220,
        pub SDLK_WORLD61         = 221,
        pub SDLK_WORLD62         = 222,
        pub SDLK_WORLD63         = 223,
        pub SDLK_WORLD64         = 224,
        pub SDLK_WORLD65         = 225,
        pub SDLK_WORLD66         = 226,
        pub SDLK_WORLD67         = 227,
        pub SDLK_WORLD68         = 228,
        pub SDLK_WORLD69         = 229,
        pub SDLK_WORLD70         = 230,
        pub SDLK_WORLD71         = 231,
        pub SDLK_WORLD72         = 232,
        pub SDLK_WORLD73         = 233,
        pub SDLK_WORLD74         = 234,
        pub SDLK_WORLD75         = 235,
        pub SDLK_WORLD76         = 236,
        pub SDLK_WORLD77         = 237,
        pub SDLK_WORLD78         = 238,
        pub SDLK_WORLD79         = 239,
        pub SDLK_WORLD80         = 240,
        pub SDLK_WORLD81         = 241,
        pub SDLK_WORLD82         = 242,
        pub SDLK_WORLD83         = 243,
        pub SDLK_WORLD84         = 244,
        pub SDLK_WORLD85         = 245,
        pub SDLK_WORLD86         = 246,
        pub SDLK_WORLD87         = 247,
        pub SDLK_WORLD88         = 248,
        pub SDLK_WORLD89         = 249,
        pub SDLK_WORLD90         = 250,
        pub SDLK_WORLD91         = 251,
        pub SDLK_WORLD92         = 252,
        pub SDLK_WORLD93         = 253,
        pub SDLK_WORLD94         = 254,
        pub SDLK_WORLD95         = 255,      /* 0xFF */
        /*@}*/

        /** @name Numeric keypad */
        /*@{*/
        pub SDLK_KP0             = 256,
        pub SDLK_KP1             = 257,
        pub SDLK_KP2             = 258,
        pub SDLK_KP3             = 259,
        pub SDLK_KP4             = 260,
        pub SDLK_KP5             = 261,
        pub SDLK_KP6             = 262,
        pub SDLK_KP7             = 263,
        pub SDLK_KP8             = 264,
        pub SDLK_KP9             = 265,
        pub SDLK_KPPERIOD        = 266,
        pub SDLK_KPDIVIDE        = 267,
        pub SDLK_KPMULTIPLY      = 268,
        pub SDLK_KPMINUS         = 269,
        pub SDLK_KPPLUS          = 270,
        pub SDLK_KPENTER         = 271,
        pub SDLK_KPEQUALS        = 272,
        /*@}*/

        /** @name Arrows + Home/End pad */
        /*@{*/
        pub SDLK_UP              = 273,
        pub SDLK_DOWN            = 274,
        pub SDLK_RIGHT           = 275,
        pub SDLK_LEFT            = 276,
        pub SDLK_INSERT          = 277,
        pub SDLK_HOME            = 278,
        pub SDLK_END             = 279,
        pub SDLK_PAGEUP          = 280,
        pub SDLK_PAGEDOWN        = 281,
        /*@}*/

        /** @name Function keys */
        /*@{*/
        pub SDLK_F1              = 282,
        pub SDLK_F2              = 283,
        pub SDLK_F3              = 284,
        pub SDLK_F4              = 285,
        pub SDLK_F5              = 286,
        pub SDLK_F6              = 287,
        pub SDLK_F7              = 288,
        pub SDLK_F8              = 289,
        pub SDLK_F9              = 290,
        pub SDLK_F10             = 291,
        pub SDLK_F11             = 292,
        pub SDLK_F12             = 293,
        pub SDLK_F13             = 294,
        pub SDLK_F14             = 295,
        pub SDLK_F15             = 296,
        /*@}*/

        /** @name Key state modifier keys */
        /*@{*/
        pub SDLK_NUMLOCK         = 300,
        pub SDLK_CAPSLOCK        = 301,
        pub SDLK_SCROLLOCK       = 302,
        pub SDLK_RSHIFT          = 303,
        pub SDLK_LSHIFT          = 304,
        pub SDLK_RCTRL           = 305,
        pub SDLK_LCTRL           = 306,
        pub SDLK_RALT            = 307,
        pub SDLK_LALT            = 308,
        pub SDLK_RMETA           = 309,
        pub SDLK_LMETA           = 310,
        pub SDLK_LSUPER          = 311,      /**< Left "Windows" key */
        pub SDLK_RSUPER          = 312,      /**< Right "Windows" key */
        pub SDLK_MODE            = 313,      /**< "Alt Gr" key */
        pub SDLK_COMPOSE         = 314,      /**< Multi-key compose key */
        /*@}*/

        /** @name Miscellaneous function keys */
        /*@{*/
        pub SDLK_HELP            = 315,
        pub SDLK_PRINT           = 316,
        pub SDLK_SYSREQ          = 317,
        pub SDLK_BREAK           = 318,
        pub SDLK_MENU            = 319,
        pub SDLK_POWER           = 320,      /**< Power Macintosh power key */
        pub SDLK_EURO            = 321,      /**< Some european keyboards */
        pub SDLK_UNDO            = 322,      /**< Atari keyboard has Undo */
        /*@}*/

        /* Add any other keys here */

        SDLK_LAST
}

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
