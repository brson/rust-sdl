/*!

Low-level bindings

*/

// This "extern" block is just for setting up linking.
#[cfg(target_os="macos")]
mod mac {
    #[cfg(mac_framework)]
    #[link_args="-framework SDL"]
    extern {}

    #[cfg(mac_dylib)]
    #[link_args="-lSDL"]
    extern {}
}

#[cfg(target_os="win32")]
#[cfg(target_os="linux")]
#[cfg(target_os="freebsd")]
mod others {
    #[link_args="-lSDL"]
    extern {}
}

pub mod error {

    use core::libc::{c_char, c_int};

    type c_enum = c_int;

    pub type SDL_errorcode = c_enum;
    
    pub const SDL_ENOMEM: SDL_errorcode         = 0;
    pub const SDL_EFREAD: SDL_errorcode         = 1;
    pub const SDL_EFWRITE: SDL_errorcode        = 2;
    pub const SDL_EFSEEK: SDL_errorcode         = 3;
    pub const SDL_UNSUPPORTED: SDL_errorcode    = 4;

    extern {
        fn SDL_Error(code: SDL_errorcode);
        fn SDL_SetError(fmt: *c_char); // FIXME: This is actually a varargs call
        fn SDL_GetError() -> *c_char;
        fn SDL_ClearError();
    }
}

pub mod sdl {

    use core::libc::c_int;
    use core::libc::types::common::c99::uint32_t;

    pub type SDL_InitFlag = uint32_t;

    pub const SDL_INIT_TIMER: SDL_InitFlag         = 0x00000001;
    pub const SDL_INIT_AUDIO: SDL_InitFlag         = 0x00000010;
    pub const SDL_INIT_VIDEO: SDL_InitFlag         = 0x00000020;
    pub const SDL_INIT_CDROM: SDL_InitFlag         = 0x00000100;
    pub const SDL_INIT_JOYSTICK: SDL_InitFlag      = 0x00000200;
    pub const SDL_INIT_NO_PARACHUTE: SDL_InitFlag  = 0x00100000;
    pub const SDL_INIT_EVENTTHREAD: SDL_InitFlag   = 0x01000000;
    pub const SDL_INIT_EVERYTHING: SDL_InitFlag    = 0x0000FFFF;

    extern {
        fn SDL_Init(flags: SDL_InitFlag) -> c_int;
        fn SDL_InitSubSystem(flags: SDL_InitFlag) -> c_int;
        fn SDL_QuitSubSystem(flags: SDL_InitFlag);
        fn SDL_Quit();
        fn SDL_WasInit(flags: SDL_InitFlag) -> SDL_InitFlag;
    }
}

pub mod event {

    use core::libc::c_int;
    use ll;

    pub type SDL_ButtonState = u8;

    pub const SDL_RELEASED: SDL_ButtonState = 0u8;
    pub const SDL_PRESSED: SDL_ButtonState = 1u8;

    pub type SDL_EventType = u8;

    pub const SDL_NOEVENT:          SDL_EventType = 0u8;
    pub const SDL_ACTIVEEVENT:      SDL_EventType = 1u8;
    pub const SDL_KEYDOWN:          SDL_EventType = 2u8;
    pub const SDL_KEYUP:            SDL_EventType = 3u8;
    pub const SDL_MOUSEMOTION:      SDL_EventType = 4u8;
    pub const SDL_MOUSEBUTTONDOWN:  SDL_EventType = 5u8;
    pub const SDL_MOUSEBUTTONUP:    SDL_EventType = 6u8;
    pub const SDL_JOYAXISMOTION:    SDL_EventType = 7u8;
    pub const SDL_JOYBALLMOTION:    SDL_EventType = 8u8;
    pub const SDL_JOYHATMOTION:     SDL_EventType = 9u8;
    pub const SDL_JOYBUTTONDOWN:    SDL_EventType = 10u8;
    pub const SDL_JOYBUTTONUP:      SDL_EventType = 11u8;
    pub const SDL_QUIT:             SDL_EventType = 12u8;
    pub const SDL_SYSWMEVENT:       SDL_EventType = 13u8;
    pub const SDL_EVENT_RESERVEDA:  SDL_EventType = 14u8;
    pub const SDL_EVENT_RESERVEDB:  SDL_EventType = 15u8;
    pub const SDL_VIDEORESIZE:      SDL_EventType = 16u8;
    pub const SDL_VIDEOEXPOSE:      SDL_EventType = 17u8;
    pub const SDL_EVENT_RESERVED2:  SDL_EventType = 18u8;
    pub const SDL_EVENT_RESERVED3:  SDL_EventType = 19u8;
    pub const SDL_EVENT_RESERVED4:  SDL_EventType = 20u8;
    pub const SDL_EVENT_RESERVED5:  SDL_EventType = 21u8;
    pub const SDL_EVENT_RESERVED6:  SDL_EventType = 22u8;
    pub const SDL_EVENT_RESERVED7:  SDL_EventType = 23u8;
    pub const SDL_EVENT_USEREVENT:  SDL_EventType = 24u8;

    pub struct SDL_Event {
        type_: SDL_EventType,
        // FIXME: Not sure exactly how big this needs to be
        event: [ u64 * 24 ]
    }

    pub struct SDL_QuitEvent {
        type_: SDL_EventType
    }

    pub struct SDL_KeyboardEvent {
        type_: SDL_EventType,
        which: u8,
        state: u8,
        keysym: ll::keyboard::SDL_keysym
    }

    pub struct SDL_MouseMotionEvent {
        type_: SDL_EventType,
        which: u8,
        state: u8,
        x: u16,
        y: u16,
        xrel: i16,
        yrel: i16
    }

    extern {
        fn SDL_PollEvent(event: *SDL_Event) -> c_int;
    }
}

pub mod keyboard {

    use ll;
    // sym corresponds to the `Key` enum, mod_ to the `Mod` enum. We should
    // be using the correct type here but our enums don't have the right
    // size yet
    pub type SDL_keysym = {
        scancode: u8,
        sym: ll::keysym::SDLKey,
        mod_: ll::keysym::SDLMod,
        unicode: u16
    };
}

pub mod keysym {
    
    use core::libc::c_int;

    type c_enum = c_int;

    /** What we really want is a mapping of every raw key on the keyboard.
     *  To support international keyboards, we use the range 0xA1 - 0xFF
     *  as international virtual keycodes.  We'll follow in the footsteps of X11...
     *  @brief The names of the keys
     */
    pub type SDLKey = c_enum;
    /** @name ASCII mapped KeySyms
    *  The keyboard syms have been cleverly chosen to map to ASCII
    */
    /*@{*/
    pub const SDLK_UNKNOWN: SDLKey = 0;
    //pub const SDLK_First: SDLKey = 0;
    pub const SDLK_BACKSPACE: SDLKey = 8;
    pub const SDLK_TAB: SDLKey = 9;
    pub const SDLK_CLEAR: SDLKey = 12;
    pub const SDLK_RETURN: SDLKey = 13;
    pub const SDLK_PAUSE: SDLKey = 19;
    pub const SDLK_ESCAPE: SDLKey = 27;
    pub const SDLK_SPACE: SDLKey = 32;
    pub const SDLK_EXCLAIM: SDLKey = 33;
    pub const SDLK_QUOTEDBL: SDLKey = 34;
    pub const SDLK_HASH: SDLKey = 35;
    pub const SDLK_DOLLAR: SDLKey = 36;
    pub const SDLK_AMPERSAND: SDLKey = 38;
    pub const SDLK_QUOTE: SDLKey = 39;
    pub const SDLK_LEFTPAREN: SDLKey = 40;
    pub const SDLK_RIGHTPAREN: SDLKey = 41;
    pub const SDLK_ASTERISK: SDLKey = 42;
    pub const SDLK_PLUS: SDLKey = 43;
    pub const SDLK_COMMA: SDLKey = 44;
    pub const SDLK_MINUS: SDLKey = 45;
    pub const SDLK_PERIOD: SDLKey = 46;
    pub const SDLK_SLASH: SDLKey = 47;
    pub const SDLK_0: SDLKey = 48;
    pub const SDLK_1: SDLKey = 49;
    pub const SDLK_2: SDLKey = 50;
    pub const SDLK_3: SDLKey = 51;
    pub const SDLK_4: SDLKey = 52;
    pub const SDLK_5: SDLKey = 53;
    pub const SDLK_6: SDLKey = 54;
    pub const SDLK_7: SDLKey = 55;
    pub const SDLK_8: SDLKey = 56;
    pub const SDLK_9: SDLKey = 57;
    pub const SDLK_COLON: SDLKey = 58;
    pub const SDLK_SEMICOLON: SDLKey = 59;
    pub const SDLK_LESS: SDLKey = 60;
    pub const SDLK_EQUALS: SDLKey = 61;
    pub const SDLK_GREATER: SDLKey = 62;
    pub const SDLK_QUESTION: SDLKey = 63;
    pub const SDLK_AT: SDLKey = 64;
    /* 
    Skip uppercase letters
    */
    pub const SDLK_LEFTBRACKET: SDLKey = 91;
    pub const SDLK_BACKSLASH: SDLKey = 92;
    pub const SDLK_RIGHTBRACKEt: SDLKey = 93;
    pub const SDLK_CARET: SDLKey = 94;
    pub const SDLK_UNDERSCORE: SDLKey = 95;
    pub const SDLK_BACKQUOTE: SDLKey = 96;
    pub const SDLK_A: SDLKey = 97;
    pub const SDLK_B: SDLKey = 98;
    pub const SDLK_C: SDLKey = 99;
    pub const SDLK_D: SDLKey = 100;
    pub const SDLK_E: SDLKey = 101;
    pub const SDLK_F: SDLKey = 102;
    pub const SDLK_G: SDLKey = 103;
    pub const SDLK_H: SDLKey = 104;
    pub const SDLK_I: SDLKey = 105;
    pub const SDLK_J: SDLKey = 106;
    pub const SDLK_K: SDLKey = 107;
    pub const SDLK_L: SDLKey = 108;
    pub const SDLK_M: SDLKey = 109;
    pub const SDLK_N: SDLKey = 110;
    pub const SDLK_O: SDLKey = 111;
    pub const SDLK_P: SDLKey = 112;
    pub const SDLK_Q: SDLKey = 113;
    pub const SDLK_R: SDLKey = 114;
    pub const SDLK_S: SDLKey = 115;
    pub const SDLK_T: SDLKey = 116;
    pub const SDLK_U: SDLKey = 117;
    pub const SDLK_V: SDLKey = 118;
    pub const SDLK_W: SDLKey = 119;
    pub const SDLK_X: SDLKey = 120;
    pub const SDLK_Y: SDLKey = 121;
    pub const SDLK_Z: SDLKey = 122;
    pub const SDLK_DELETE: SDLKey = 127;
    /* End of ASCII mapped KeySyms */
    /*@}*/

    /** @name International keyboard syms */
    /*@{*/
    pub const SDLK_WORLD0: SDLKey = 160;      /* 0xA0 */
    pub const SDLK_WORLD1: SDLKey = 161;
    pub const SDLK_WORLD2: SDLKey = 162;
    pub const SDLK_WORLD3: SDLKey = 163;
    pub const SDLK_WORLD4: SDLKey = 164;
    pub const SDLK_WORLD5: SDLKey = 165;
    pub const SDLK_WORLD6: SDLKey = 166;
    pub const SDLK_WORLD7: SDLKey = 167;
    pub const SDLK_WORLD8: SDLKey = 168;
    pub const SDLK_WORLD9: SDLKey = 169;
    pub const SDLK_WORLD10: SDLKey = 170;
    pub const SDLK_WORLD11: SDLKey = 171;
    pub const SDLK_WORLD12: SDLKey = 172;
    pub const SDLK_WORLD13: SDLKey = 173;
    pub const SDLK_WORLD14: SDLKey = 174;
    pub const SDLK_WORLD15: SDLKey = 175;
    pub const SDLK_WORLD16: SDLKey = 176;
    pub const SDLK_WORLD17: SDLKey = 177;
    pub const SDLK_WORLD18: SDLKey = 178;
    pub const SDLK_WORLD19: SDLKey = 179;
    pub const SDLK_WORLD20: SDLKey = 180;
    pub const SDLK_WORLD21: SDLKey = 181;
    pub const SDLK_WORLD22: SDLKey = 182;
    pub const SDLK_WORLD23: SDLKey = 183;
    pub const SDLK_WORLD24: SDLKey = 184;
    pub const SDLK_WORLD25: SDLKey = 185;
    pub const SDLK_WORLD26: SDLKey = 186;
    pub const SDLK_WORLD27: SDLKey = 187;
    pub const SDLK_WORLD28: SDLKey = 188;
    pub const SDLK_WORLD29: SDLKey = 189;
    pub const SDLK_WORLD30: SDLKey = 190;
    pub const SDLK_WORLD31: SDLKey = 191;
    pub const SDLK_WORLD32: SDLKey = 192;
    pub const SDLK_WORLD33: SDLKey = 193;
    pub const SDLK_WORLD34: SDLKey = 194;
    pub const SDLK_WORLD35: SDLKey = 195;
    pub const SDLK_WORLD36: SDLKey = 196;
    pub const SDLK_WORLD37: SDLKey = 197;
    pub const SDLK_WORLD38: SDLKey = 198;
    pub const SDLK_WORLD39: SDLKey = 199;
    pub const SDLK_WORLD40: SDLKey = 200;
    pub const SDLK_WORLD41: SDLKey = 201;
    pub const SDLK_WORLD42: SDLKey = 202;
    pub const SDLK_WORLD43: SDLKey = 203;
    pub const SDLK_WORLD44: SDLKey = 204;
    pub const SDLK_WORLD45: SDLKey = 205;
    pub const SDLK_WORLD46: SDLKey = 206;
    pub const SDLK_WORLD47: SDLKey = 207;
    pub const SDLK_WORLD48: SDLKey = 208;
    pub const SDLK_WORLD49: SDLKey = 209;
    pub const SDLK_WORLD50: SDLKey = 210;
    pub const SDLK_WORLD51: SDLKey = 211;
    pub const SDLK_WORLD52: SDLKey = 212;
    pub const SDLK_WORLD53: SDLKey = 213;
    pub const SDLK_WORLD54: SDLKey = 214;
    pub const SDLK_WORLD55: SDLKey = 215;
    pub const SDLK_WORLD56: SDLKey = 216;
    pub const SDLK_WORLD57: SDLKey = 217;
    pub const SDLK_WORLD58: SDLKey = 218;
    pub const SDLK_WORLD59: SDLKey = 219;
    pub const SDLK_WORLD60: SDLKey = 220;
    pub const SDLK_WORLD61: SDLKey = 221;
    pub const SDLK_WORLD62: SDLKey = 222;
    pub const SDLK_WORLD63: SDLKey = 223;
    pub const SDLK_WORLD64: SDLKey = 224;
    pub const SDLK_WORLD65: SDLKey = 225;
    pub const SDLK_WORLD66: SDLKey = 226;
    pub const SDLK_WORLD67: SDLKey = 227;
    pub const SDLK_WORLD68: SDLKey = 228;
    pub const SDLK_WORLD69: SDLKey = 229;
    pub const SDLK_WORLD70: SDLKey = 230;
    pub const SDLK_WORLD71: SDLKey = 231;
    pub const SDLK_WORLD72: SDLKey = 232;
    pub const SDLK_WORLD73: SDLKey = 233;
    pub const SDLK_WORLD74: SDLKey = 234;
    pub const SDLK_WORLD75: SDLKey = 235;
    pub const SDLK_WORLD76: SDLKey = 236;
    pub const SDLK_WORLD77: SDLKey = 237;
    pub const SDLK_WORLD78: SDLKey = 238;
    pub const SDLK_WORLD79: SDLKey = 239;
    pub const SDLK_WORLD80: SDLKey = 240;
    pub const SDLK_WORLD81: SDLKey = 241;
    pub const SDLK_WORLD82: SDLKey = 242;
    pub const SDLK_WORLD83: SDLKey = 243;
    pub const SDLK_WORLD84: SDLKey = 244;
    pub const SDLK_WORLD85: SDLKey = 245;
    pub const SDLK_WORLD86: SDLKey = 246;
    pub const SDLK_WORLD87: SDLKey = 247;
    pub const SDLK_WORLD88: SDLKey = 248;
    pub const SDLK_WORLD89: SDLKey = 249;
    pub const SDLK_WORLD90: SDLKey = 250;
    pub const SDLK_WORLD91: SDLKey = 251;
    pub const SDLK_WORLD92: SDLKey = 252;
    pub const SDLK_WORLD93: SDLKey = 253;
    pub const SDLK_WORLD94: SDLKey = 254;
    pub const SDLK_WORLD95: SDLKey = 255;      /* 0xFF */
    /*@}*/

    /** @name Numeric keypad */
    /*@{*/
    pub const SDLK_KP0: SDLKey = 256;
    pub const SDLK_KP1: SDLKey = 257;
    pub const SDLK_KP2: SDLKey = 258;
    pub const SDLK_KP3: SDLKey = 259;
    pub const SDLK_KP4: SDLKey = 260;
    pub const SDLK_KP5: SDLKey = 261;
    pub const SDLK_KP6: SDLKey = 262;
    pub const SDLK_KP7: SDLKey = 263;
    pub const SDLK_KP8: SDLKey = 264;
    pub const SDLK_KP9: SDLKey = 265;
    pub const SDLK_KPPERIOD: SDLKey = 266;
    pub const SDLK_KPDIVIDE: SDLKey = 267;
    pub const SDLK_KPMULTIPLY: SDLKey = 268;
    pub const SDLK_KPMINUS: SDLKey = 269;
    pub const SDLK_KPPLUS: SDLKey = 270;
    pub const SDLK_KPENTER: SDLKey = 271;
    pub const SDLK_KPEQUALS: SDLKey = 272;
    /*@}*/

    /** @name Arrows + Home/End pad */
    /*@{*/
    pub const SDLK_UP: SDLKey = 273;
    pub const SDLK_DOWN: SDLKey = 274;
    pub const SDLK_RIGHT: SDLKey = 275;
    pub const SDLK_LEFT: SDLKey = 276;
    pub const SDLK_INSERT: SDLKey = 277;
    pub const SDLK_HOME: SDLKey = 278;
    pub const SDLK_END: SDLKey = 279;
    pub const SDLK_PAGEUP: SDLKey = 280;
    pub const SDLK_PAGEDOWN: SDLKey = 281;
    /*@}*/

    /** @name Function keys */
    /*@{*/
    pub const SDLK_F1: SDLKey = 282;
    pub const SDLK_F2: SDLKey = 283;
    pub const SDLK_F3: SDLKey = 284;
    pub const SDLK_F4: SDLKey = 285;
    pub const SDLK_F5: SDLKey = 286;
    pub const SDLK_F6: SDLKey = 287;
    pub const SDLK_F7: SDLKey = 288;
    pub const SDLK_F8: SDLKey = 289;
    pub const SDLK_F9: SDLKey = 290;
    pub const SDLK_F10: SDLKey = 291;
    pub const SDLK_F11: SDLKey = 292;
    pub const SDLK_F12: SDLKey = 293;
    pub const SDLK_F13: SDLKey = 294;
    pub const SDLK_F14: SDLKey = 295;
    pub const SDLK_F15: SDLKey = 296;
    /*@}*/

    /** @name Key state modifier keys */
    /*@{*/
    pub const SDLK_NUMLOCK: SDLKey = 300;
    pub const SDLK_CAPSLOCK: SDLKey = 301;
    pub const SDLK_SCROLLOCK: SDLKey = 302;
    pub const SDLK_RSHIFT: SDLKey = 303;
    pub const SDLK_LSHIFT: SDLKey = 304;
    pub const SDLK_RCTRL: SDLKey = 305;
    pub const SDLK_LCTRL: SDLKey = 306;
    pub const SDLK_RALT: SDLKey = 307;
    pub const SDLK_LALT: SDLKey = 308;
    pub const SDLK_RMETA: SDLKey = 309;
    pub const SDLK_LMETA: SDLKey = 310;
    pub const SDLK_LSUPER: SDLKey = 311;      /**< Left "Windows" key */
    pub const SDLK_RSUPER: SDLKey = 312;      /**< Right "Windows" key */
    pub const SDLK_MODE: SDLKey = 313;      /**< "Alt Gr" key */
    pub const SDLK_COMPOSE: SDLKey = 314;      /**< Multi-key compose key */
    /*@}*/

    /** @name Miscellaneous function keys */
    /*@{*/
    pub const SDLK_HELP: SDLKey = 315;
    pub const SDLK_PRINT: SDLKey = 316;
    pub const SDLK_SYSREQ: SDLKey = 317;
    pub const SDLK_BREAK: SDLKey = 318;
    pub const SDLK_MENU: SDLKey = 319;
    pub const SDLK_POWER: SDLKey = 320;      /**< Power Macintosh power key */
    pub const SDLK_EURO: SDLKey = 321;      /**< Some european keyboards */
    pub const SDLK_UNDO: SDLKey = 322;      /**< Atari keyboard has Undo */
    /*@}*/

    /* Add any other keys here */

    pub const SDLK_LAST: SDLKey = 323;

    pub type SDLMod = c_int;

    pub const KMOD_None: SDLMod = 0x0000;
    pub const KMOD_LShift: SDLMod = 0x0001;
    pub const KMOD_RShift: SDLMod = 0x0002;
    pub const KMOD_LCtrl: SDLMod = 0x0040;
    pub const KMOD_RCtrl: SDLMod = 0x0080;
    pub const KMOD_LAlt: SDLMod = 0x0100;
    pub const KMOD_RAlt: SDLMod = 0x0200;
    pub const KMOD_LMeta: SDLMod = 0x0400;
    pub const KMOD_RMeta: SDLMod = 0x0800;
    pub const KMOD_Num: SDLMod = 0x1000;
    pub const KMOD_Caps: SDLMod = 0x2000;
    pub const KMOD_Mode: SDLMod = 0x4000;
    pub const KMOD_Reserved: SDLMod = 0x8000;
}

pub mod video {

    use core::libc::{c_char, c_int, c_void};
    use core::libc::types::common::c99::{uint32_t, uint16_t};
    use util::Rect;

    type RWOps = c_void;

    pub type SDL_Surface = {
        flags: SDL_Flag,
        format: *c_void,
        w: c_int,
        h: c_int,
        pitch: uint16_t,
        pixels: *c_void,
        offset: c_int 
        // FIXME: Remaining are unlisted
    };

    pub type SDL_Flag = uint32_t;
    pub type SDL_SurfaceFlag = SDL_Flag;

    pub const SDL_SWSURFACE: SDL_SurfaceFlag = 0x00000000;
    pub const SDL_HWSURFACE: SDL_SurfaceFlag = 0x00000001;
    pub const SDL_ASYNCBLIT: SDL_SurfaceFlag = 0x00000004;

    pub type SDL_VideoModeFlag = SDL_Flag;

    pub const SDL_ANYFORMAT: SDL_VideoModeFlag  = 0x10000000;
    pub const SDL_HWPALETTE: SDL_VideoModeFlag  = 0x20000000;
    pub const SDL_DOUBLEBUF: SDL_VideoModeFlag  = 0x40000000;
    pub const SDL_FULLSCREEN: SDL_VideoModeFlag = 0x80000000;
    pub const SDL_OPENGL: SDL_VideoModeFlag     = 0x00000002;
    pub const SDL_OPENGLBLIT: SDL_VideoModeFlag = 0x0000000A;
    pub const SDL_RESIZABLE: SDL_VideoModeFlag  = 0x00000010;
    pub const SDL_NOFRAME: SDL_VideoModeFlag    = 0x00000020;

    extern {
        fn SDL_SetVideoMode(width: c_int, height: c_int, bitsperpixel: c_int, flags: SDL_Flag) -> *SDL_Surface;
        fn SDL_FreeSurface(surface: *SDL_Surface);
        fn SDL_LoadBMP_RW(src: *RWOps, freesrc: c_int) -> *SDL_Surface;
        fn SDL_RWFromFile(file: *c_char, mode: *c_char) -> *RWOps;
        fn SDL_DisplayFormat(surface: *SDL_Surface) -> *SDL_Surface;
        fn SDL_UpperBlit(src: *SDL_Surface, srcrect: *Rect,
                         dst: *SDL_Surface, dstrect: *Rect) -> c_int;
        fn SDL_Flip(screen: *SDL_Surface) -> c_int;
        fn SDL_CreateRGBSurface(flags: SDL_Flag, width: c_int, height: c_int,
                                bitsPerPixel: c_int,
                                Rmask: uint32_t, Gmask: uint32_t, Bmask: uint32_t, Amask: uint32_t) -> *SDL_Surface;
        fn SDL_FillRect(dst: *SDL_Surface, dstrect: *Rect, color: uint32_t) -> c_int;
        fn SDL_LockSurface(surface: *SDL_Surface) -> c_int;
        fn SDL_UnlockSurface(surface: *SDL_Surface);
        fn SDL_GL_SwapBuffers();
    }
}

pub mod img {
    use ll;
    use core::libc::{c_char, c_int, c_void};

    extern mod SDL_image {
        fn IMG_Load(file: *c_char) -> *ll::video::SDL_Surface;
    }
}
