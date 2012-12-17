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
    pub type SDL_keysym = {
        scancode: u8,
        sym: ll::keysym::sdl_key_type,
        mod_: ll::keysym::sdl_key_modifier,
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
    pub const SDLK_UNKNOWN: sdl_key_type = 0;
    //pub const SDLK_First: sdl_key_type = 0;
    pub const SDLK_BACKSPACE: sdl_key_type = 8;
    pub const SDLK_TAB: sdl_key_type = 9;
    pub const SDLK_CLEAR: sdl_key_type = 12;
    pub const SDLK_RETURN: sdl_key_type = 13;
    pub const SDLK_PAUSE: sdl_key_type = 19;
    pub const SDLK_ESCAPE: sdl_key_type = 27;
    pub const SDLK_SPACE: sdl_key_type = 32;
    pub const SDLK_EXCLAIM: sdl_key_type = 33;
    pub const SDLK_QUOTEDBL: sdl_key_type = 34;
    pub const SDLK_HASH: sdl_key_type = 35;
    pub const SDLK_DOLLAR: sdl_key_type = 36;
    pub const SDLK_AMPERSAND: sdl_key_type = 38;
    pub const SDLK_QUOTE: sdl_key_type = 39;
    pub const SDLK_LEFTPAREN: sdl_key_type = 40;
    pub const SDLK_RIGHTPAREN: sdl_key_type = 41;
    pub const SDLK_ASTERISK: sdl_key_type = 42;
    pub const SDLK_PLUS: sdl_key_type = 43;
    pub const SDLK_COMMA: sdl_key_type = 44;
    pub const SDLK_MINUS: sdl_key_type = 45;
    pub const SDLK_PERIOD: sdl_key_type = 46;
    pub const SDLK_SLASH: sdl_key_type = 47;
    pub const SDLK_0: sdl_key_type = 48;
    pub const SDLK_1: sdl_key_type = 49;
    pub const SDLK_2: sdl_key_type = 50;
    pub const SDLK_3: sdl_key_type = 51;
    pub const SDLK_4: sdl_key_type = 52;
    pub const SDLK_5: sdl_key_type = 53;
    pub const SDLK_6: sdl_key_type = 54;
    pub const SDLK_7: sdl_key_type = 55;
    pub const SDLK_8: sdl_key_type = 56;
    pub const SDLK_9: sdl_key_type = 57;
    pub const SDLK_COLON: sdl_key_type = 58;
    pub const SDLK_SEMICOLON: sdl_key_type = 59;
    pub const SDLK_LESS: sdl_key_type = 60;
    pub const SDLK_EQUALS: sdl_key_type = 61;
    pub const SDLK_GREATER: sdl_key_type = 62;
    pub const SDLK_QUESTION: sdl_key_type = 63;
    pub const SDLK_AT: sdl_key_type = 64;
    /* 
    Skip uppercase letters
    */
    pub const SDLK_LEFTBRACKET: sdl_key_type = 91;
    pub const SDLK_BACKSLASH: sdl_key_type = 92;
    pub const SDLK_RIGHTBRACKEt: sdl_key_type = 93;
    pub const SDLK_CARET: sdl_key_type = 94;
    pub const SDLK_UNDERSCORE: sdl_key_type = 95;
    pub const SDLK_BACKQUOTE: sdl_key_type = 96;
    pub const SDLK_A: sdl_key_type = 97;
    pub const SDLK_B: sdl_key_type = 98;
    pub const SDLK_C: sdl_key_type = 99;
    pub const SDLK_D: sdl_key_type = 100;
    pub const SDLK_E: sdl_key_type = 101;
    pub const SDLK_F: sdl_key_type = 102;
    pub const SDLK_G: sdl_key_type = 103;
    pub const SDLK_H: sdl_key_type = 104;
    pub const SDLK_I: sdl_key_type = 105;
    pub const SDLK_J: sdl_key_type = 106;
    pub const SDLK_K: sdl_key_type = 107;
    pub const SDLK_L: sdl_key_type = 108;
    pub const SDLK_M: sdl_key_type = 109;
    pub const SDLK_N: sdl_key_type = 110;
    pub const SDLK_O: sdl_key_type = 111;
    pub const SDLK_P: sdl_key_type = 112;
    pub const SDLK_Q: sdl_key_type = 113;
    pub const SDLK_R: sdl_key_type = 114;
    pub const SDLK_S: sdl_key_type = 115;
    pub const SDLK_T: sdl_key_type = 116;
    pub const SDLK_U: sdl_key_type = 117;
    pub const SDLK_V: sdl_key_type = 118;
    pub const SDLK_W: sdl_key_type = 119;
    pub const SDLK_X: sdl_key_type = 120;
    pub const SDLK_Y: sdl_key_type = 121;
    pub const SDLK_Z: sdl_key_type = 122;
    pub const SDLK_DELETE: sdl_key_type = 127;
    /* End of ASCII mapped KeySyms */
    /*@}*/

    /** @name International keyboard syms */
    /*@{*/
    pub const SDLK_WORLD0: sdl_key_type = 160;      /* 0xA0 */
    pub const SDLK_WORLD1: sdl_key_type = 161;
    pub const SDLK_WORLD2: sdl_key_type = 162;
    pub const SDLK_WORLD3: sdl_key_type = 163;
    pub const SDLK_WORLD4: sdl_key_type = 164;
    pub const SDLK_WORLD5: sdl_key_type = 165;
    pub const SDLK_WORLD6: sdl_key_type = 166;
    pub const SDLK_WORLD7: sdl_key_type = 167;
    pub const SDLK_WORLD8: sdl_key_type = 168;
    pub const SDLK_WORLD9: sdl_key_type = 169;
    pub const SDLK_WORLD10: sdl_key_type = 170;
    pub const SDLK_WORLD11: sdl_key_type = 171;
    pub const SDLK_WORLD12: sdl_key_type = 172;
    pub const SDLK_WORLD13: sdl_key_type = 173;
    pub const SDLK_WORLD14: sdl_key_type = 174;
    pub const SDLK_WORLD15: sdl_key_type = 175;
    pub const SDLK_WORLD16: sdl_key_type = 176;
    pub const SDLK_WORLD17: sdl_key_type = 177;
    pub const SDLK_WORLD18: sdl_key_type = 178;
    pub const SDLK_WORLD19: sdl_key_type = 179;
    pub const SDLK_WORLD20: sdl_key_type = 180;
    pub const SDLK_WORLD21: sdl_key_type = 181;
    pub const SDLK_WORLD22: sdl_key_type = 182;
    pub const SDLK_WORLD23: sdl_key_type = 183;
    pub const SDLK_WORLD24: sdl_key_type = 184;
    pub const SDLK_WORLD25: sdl_key_type = 185;
    pub const SDLK_WORLD26: sdl_key_type = 186;
    pub const SDLK_WORLD27: sdl_key_type = 187;
    pub const SDLK_WORLD28: sdl_key_type = 188;
    pub const SDLK_WORLD29: sdl_key_type = 189;
    pub const SDLK_WORLD30: sdl_key_type = 190;
    pub const SDLK_WORLD31: sdl_key_type = 191;
    pub const SDLK_WORLD32: sdl_key_type = 192;
    pub const SDLK_WORLD33: sdl_key_type = 193;
    pub const SDLK_WORLD34: sdl_key_type = 194;
    pub const SDLK_WORLD35: sdl_key_type = 195;
    pub const SDLK_WORLD36: sdl_key_type = 196;
    pub const SDLK_WORLD37: sdl_key_type = 197;
    pub const SDLK_WORLD38: sdl_key_type = 198;
    pub const SDLK_WORLD39: sdl_key_type = 199;
    pub const SDLK_WORLD40: sdl_key_type = 200;
    pub const SDLK_WORLD41: sdl_key_type = 201;
    pub const SDLK_WORLD42: sdl_key_type = 202;
    pub const SDLK_WORLD43: sdl_key_type = 203;
    pub const SDLK_WORLD44: sdl_key_type = 204;
    pub const SDLK_WORLD45: sdl_key_type = 205;
    pub const SDLK_WORLD46: sdl_key_type = 206;
    pub const SDLK_WORLD47: sdl_key_type = 207;
    pub const SDLK_WORLD48: sdl_key_type = 208;
    pub const SDLK_WORLD49: sdl_key_type = 209;
    pub const SDLK_WORLD50: sdl_key_type = 210;
    pub const SDLK_WORLD51: sdl_key_type = 211;
    pub const SDLK_WORLD52: sdl_key_type = 212;
    pub const SDLK_WORLD53: sdl_key_type = 213;
    pub const SDLK_WORLD54: sdl_key_type = 214;
    pub const SDLK_WORLD55: sdl_key_type = 215;
    pub const SDLK_WORLD56: sdl_key_type = 216;
    pub const SDLK_WORLD57: sdl_key_type = 217;
    pub const SDLK_WORLD58: sdl_key_type = 218;
    pub const SDLK_WORLD59: sdl_key_type = 219;
    pub const SDLK_WORLD60: sdl_key_type = 220;
    pub const SDLK_WORLD61: sdl_key_type = 221;
    pub const SDLK_WORLD62: sdl_key_type = 222;
    pub const SDLK_WORLD63: sdl_key_type = 223;
    pub const SDLK_WORLD64: sdl_key_type = 224;
    pub const SDLK_WORLD65: sdl_key_type = 225;
    pub const SDLK_WORLD66: sdl_key_type = 226;
    pub const SDLK_WORLD67: sdl_key_type = 227;
    pub const SDLK_WORLD68: sdl_key_type = 228;
    pub const SDLK_WORLD69: sdl_key_type = 229;
    pub const SDLK_WORLD70: sdl_key_type = 230;
    pub const SDLK_WORLD71: sdl_key_type = 231;
    pub const SDLK_WORLD72: sdl_key_type = 232;
    pub const SDLK_WORLD73: sdl_key_type = 233;
    pub const SDLK_WORLD74: sdl_key_type = 234;
    pub const SDLK_WORLD75: sdl_key_type = 235;
    pub const SDLK_WORLD76: sdl_key_type = 236;
    pub const SDLK_WORLD77: sdl_key_type = 237;
    pub const SDLK_WORLD78: sdl_key_type = 238;
    pub const SDLK_WORLD79: sdl_key_type = 239;
    pub const SDLK_WORLD80: sdl_key_type = 240;
    pub const SDLK_WORLD81: sdl_key_type = 241;
    pub const SDLK_WORLD82: sdl_key_type = 242;
    pub const SDLK_WORLD83: sdl_key_type = 243;
    pub const SDLK_WORLD84: sdl_key_type = 244;
    pub const SDLK_WORLD85: sdl_key_type = 245;
    pub const SDLK_WORLD86: sdl_key_type = 246;
    pub const SDLK_WORLD87: sdl_key_type = 247;
    pub const SDLK_WORLD88: sdl_key_type = 248;
    pub const SDLK_WORLD89: sdl_key_type = 249;
    pub const SDLK_WORLD90: sdl_key_type = 250;
    pub const SDLK_WORLD91: sdl_key_type = 251;
    pub const SDLK_WORLD92: sdl_key_type = 252;
    pub const SDLK_WORLD93: sdl_key_type = 253;
    pub const SDLK_WORLD94: sdl_key_type = 254;
    pub const SDLK_WORLD95: sdl_key_type = 255;      /* 0xFF */
    /*@}*/

    /** @name Numeric keypad */
    /*@{*/
    pub const SDLK_KP0: sdl_key_type = 256;
    pub const SDLK_KP1: sdl_key_type = 257;
    pub const SDLK_KP2: sdl_key_type = 258;
    pub const SDLK_KP3: sdl_key_type = 259;
    pub const SDLK_KP4: sdl_key_type = 260;
    pub const SDLK_KP5: sdl_key_type = 261;
    pub const SDLK_KP6: sdl_key_type = 262;
    pub const SDLK_KP7: sdl_key_type = 263;
    pub const SDLK_KP8: sdl_key_type = 264;
    pub const SDLK_KP9: sdl_key_type = 265;
    pub const SDLK_KPPERIOD: sdl_key_type = 266;
    pub const SDLK_KPDIVIDE: sdl_key_type = 267;
    pub const SDLK_KPMULTIPLY: sdl_key_type = 268;
    pub const SDLK_KPMINUS: sdl_key_type = 269;
    pub const SDLK_KPPLUS: sdl_key_type = 270;
    pub const SDLK_KPENTER: sdl_key_type = 271;
    pub const SDLK_KPEQUALS: sdl_key_type = 272;
    /*@}*/

    /** @name Arrows + Home/End pad */
    /*@{*/
    pub const SDLK_UP: sdl_key_type = 273;
    pub const SDLK_DOWN: sdl_key_type = 274;
    pub const SDLK_RIGHT: sdl_key_type = 275;
    pub const SDLK_LEFT: sdl_key_type = 276;
    pub const SDLK_INSERT: sdl_key_type = 277;
    pub const SDLK_HOME: sdl_key_type = 278;
    pub const SDLK_END: sdl_key_type = 279;
    pub const SDLK_PAGEUP: sdl_key_type = 280;
    pub const SDLK_PAGEDOWN: sdl_key_type = 281;
    /*@}*/

    /** @name Function keys */
    /*@{*/
    pub const SDLK_F1: sdl_key_type = 282;
    pub const SDLK_F2: sdl_key_type = 283;
    pub const SDLK_F3: sdl_key_type = 284;
    pub const SDLK_F4: sdl_key_type = 285;
    pub const SDLK_F5: sdl_key_type = 286;
    pub const SDLK_F6: sdl_key_type = 287;
    pub const SDLK_F7: sdl_key_type = 288;
    pub const SDLK_F8: sdl_key_type = 289;
    pub const SDLK_F9: sdl_key_type = 290;
    pub const SDLK_F10: sdl_key_type = 291;
    pub const SDLK_F11: sdl_key_type = 292;
    pub const SDLK_F12: sdl_key_type = 293;
    pub const SDLK_F13: sdl_key_type = 294;
    pub const SDLK_F14: sdl_key_type = 295;
    pub const SDLK_F15: sdl_key_type = 296;
    /*@}*/

    /** @name Key state modifier keys */
    /*@{*/
    pub const SDLK_NUMLOCK: sdl_key_type = 300;
    pub const SDLK_CAPSLOCK: sdl_key_type = 301;
    pub const SDLK_SCROLLOCK: sdl_key_type = 302;
    pub const SDLK_RSHIFT: sdl_key_type = 303;
    pub const SDLK_LSHIFT: sdl_key_type = 304;
    pub const SDLK_RCTRL: sdl_key_type = 305;
    pub const SDLK_LCTRL: sdl_key_type = 306;
    pub const SDLK_RALT: sdl_key_type = 307;
    pub const SDLK_LALT: sdl_key_type = 308;
    pub const SDLK_RMETA: sdl_key_type = 309;
    pub const SDLK_LMETA: sdl_key_type = 310;
    pub const SDLK_LSUPER: sdl_key_type = 311;      /**< Left "Windows" key */
    pub const SDLK_RSUPER: sdl_key_type = 312;      /**< Right "Windows" key */
    pub const SDLK_MODE: sdl_key_type = 313;      /**< "Alt Gr" key */
    pub const SDLK_COMPOSE: sdl_key_type = 314;      /**< Multi-key compose key */
    /*@}*/

    /** @name Miscellaneous function keys */
    /*@{*/
    pub const SDLK_HELP: sdl_key_type = 315;
    pub const SDLK_PRINT: sdl_key_type = 316;
    pub const SDLK_SYSREQ: sdl_key_type = 317;
    pub const SDLK_BREAK: sdl_key_type = 318;
    pub const SDLK_MENU: sdl_key_type = 319;
    pub const SDLK_POWER: sdl_key_type = 320;      /**< Power Macintosh power key */
    pub const SDLK_EURO: sdl_key_type = 321;      /**< Some european keyboards */
    pub const SDLK_UNDO: sdl_key_type = 322;      /**< Atari keyboard has Undo */
    /*@}*/

    /* Add any other keys here */

    pub const SDLK_LAST: sdl_key_type = 323;

    pub type sdl_key_modifier = c_int;

    pub const KMOD_None: sdl_key_modifier = 0x0000;
    pub const KMOD_LShift: sdl_key_modifier = 0x0001;
    pub const KMOD_RShift: sdl_key_modifier = 0x0002;
    pub const KMOD_LCtrl: sdl_key_modifier = 0x0040;
    pub const KMOD_RCtrl: sdl_key_modifier = 0x0080;
    pub const KMOD_LAlt: sdl_key_modifier = 0x0100;
    pub const KMOD_RAlt: sdl_key_modifier = 0x0200;
    pub const KMOD_LMeta: sdl_key_modifier = 0x0400;
    pub const KMOD_RMeta: sdl_key_modifier = 0x0800;
    pub const KMOD_Num: sdl_key_modifier = 0x1000;
    pub const KMOD_Caps: sdl_key_modifier = 0x2000;
    pub const KMOD_Mode: sdl_key_modifier = 0x4000;
    pub const KMOD_Reserved: sdl_key_modifier = 0x8000;
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
