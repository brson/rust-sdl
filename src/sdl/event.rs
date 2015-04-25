use std::mem;
use libc::c_int;
use std::slice;
use num::FromPrimitive;
use std::ffi::CStr;
use std::str;

pub mod ll {
    #![allow(non_camel_case_types)]

    use std::mem;
    use libc::{c_void, c_int, c_uint, c_uchar, uint8_t, uint16_t, int16_t};
    use libc::types::os::arch::c95::c_schar;
    pub use keysym::*;

    pub type SDL_EventType = c_uint;
    pub type SDLMod = c_uint;
    pub type SDL_SysWMmsg = c_void;

    pub const SDL_NOEVENT: SDL_EventType = 0;
    pub const SDL_ACTIVEEVENT: SDL_EventType = 1;
    pub const SDL_KEYDOWN: SDL_EventType = 2;
    pub const SDL_KEYUP: SDL_EventType = 3;
    pub const SDL_MOUSEMOTION: SDL_EventType = 4;
    pub const SDL_MOUSEBUTTONDOWN: SDL_EventType = 5;
    pub const SDL_MOUSEBUTTONUP: SDL_EventType = 6;
    pub const SDL_JOYAXISMOTION: SDL_EventType = 7;
    pub const SDL_JOYBALLMOTION: SDL_EventType = 8;
    pub const SDL_JOYHATMOTION: SDL_EventType = 9;
    pub const SDL_JOYBUTTONDOWN: SDL_EventType = 10;
    pub const SDL_JOYBUTTONUP: SDL_EventType = 11;
    pub const SDL_QUIT: SDL_EventType = 12;
    pub const SDL_SYSWMEVENT: SDL_EventType = 13;
    pub const SDL_VIDEORESIZE: SDL_EventType = 16;
    pub const SDL_VIDEOEXPOSE: SDL_EventType = 17;
    pub const SDL_USEREVENT: SDL_EventType = 24;

    pub const SDL_DISABLE: c_int = 0;
    pub const SDL_ENABLE: c_int = 1;
    pub const SDL_QUERY: c_int = -1;

    pub const SDL_APPMOUSEFOCUS: c_int = 0x01;
    pub const SDL_APPINPUTFOCUS: c_int = 0x02;
    pub const SDL_APPACTIVE: c_int = 0x04;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SDL_keysym {
        pub scancode: uint8_t,
        pub sym: SDLKey,
        pub _mod: SDLMod,
        pub unicode: uint16_t,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SDL_Event {
        pub data: [c_uchar; 24],
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SDL_ActiveEvent {
        pub _type: uint8_t,
        pub gain: uint8_t,
        pub state: uint8_t,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SDL_KeyboardEvent {
        pub _type: uint8_t,
        pub which: uint8_t,
        pub state: uint8_t,
        pub keysym: SDL_keysym,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SDL_MouseMotionEvent {
        pub _type: uint8_t,
        pub which: uint8_t,
        pub state: uint8_t,
        pub x: uint16_t,
        pub y: uint16_t,
        pub xrel: int16_t,
        pub yrel: int16_t,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SDL_MouseButtonEvent {
        pub _type: uint8_t,
        pub which: uint8_t,
        pub button: uint8_t,
        pub state: uint8_t,
        pub x: uint16_t,
        pub y: uint16_t,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SDL_JoyAxisEvent {
        pub _type: uint8_t,
        pub which: uint8_t,
        pub axis: uint8_t,
        pub value: int16_t,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SDL_JoyBallEvent {
        pub _type: uint8_t,
        pub which: uint8_t,
        pub ball: uint8_t,
        pub xrel: int16_t,
        pub yrel: int16_t,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SDL_JoyHatEvent {
        pub _type: uint8_t,
        pub which: uint8_t,
        pub hat: uint8_t,
        pub value: uint8_t,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SDL_JoyButtonEvent {
        pub _type: uint8_t,
        pub which: uint8_t,
        pub button: uint8_t,
        pub state: uint8_t,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SDL_ResizeEvent {
        pub _type: uint8_t,
        pub w: c_int,
        pub h: c_int,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SDL_ExposeEvent {
        pub _type: uint8_t,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SDL_QuitEvent {
        pub _type: uint8_t,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SDL_UserEvent {
        pub _type: uint8_t,
        pub code: c_int,
        pub data1: *mut c_void,
        pub data2: *mut c_void,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SDL_SysWMEvent {
        pub _type: uint8_t,
        pub msg: *mut SDL_SysWMmsg,
    }

    impl SDL_Event {
        pub fn _type(&self) -> *const uint8_t {
            unsafe { mem::transmute_copy(&self) }
        }

        pub fn active(&self) -> *const SDL_ActiveEvent {
            unsafe { mem::transmute_copy(&self) }
        }

        pub fn key(&self) -> *const SDL_KeyboardEvent {
            unsafe { mem::transmute_copy(&self) }
        }

        pub fn motion(&self) -> *const SDL_MouseMotionEvent {
            unsafe { mem::transmute_copy(&self) }
        }

        pub fn button(&self) -> *const SDL_MouseButtonEvent {
            unsafe { mem::transmute_copy(&self) }
        }

        pub fn jaxis(&self) -> *const SDL_JoyAxisEvent {
            unsafe { mem::transmute_copy(&self) }
        }

        pub fn jball(&self) -> *const SDL_JoyBallEvent {
            unsafe { mem::transmute_copy(&self) }
        }

        pub fn jhat(&self) -> *const SDL_JoyHatEvent {
            unsafe { mem::transmute_copy(&self) }
        }

        pub fn jbutton(&self) -> *const SDL_JoyButtonEvent {
            unsafe { mem::transmute_copy(&self) }
        }

        pub fn resize(&self) -> *const SDL_ResizeEvent {
            unsafe { mem::transmute_copy(&self) }
        }

        pub fn expose(&self) -> *const SDL_ExposeEvent {
            unsafe { mem::transmute_copy(&self) }
        }

        pub fn quit(&self) -> *const SDL_QuitEvent {
            unsafe { mem::transmute_copy(&self) }
        }

        pub fn user(&self) -> *const SDL_UserEvent {
            unsafe { mem::transmute_copy(&self) }
        }

        pub fn syswm(&self) -> *const SDL_SysWMEvent {
            unsafe { mem::transmute_copy(&self) }
        }
    }

    extern "C" {
        pub fn SDL_PollEvent(event: *mut SDL_Event) -> c_int;
        pub fn SDL_WaitEvent(event: *mut SDL_Event) -> c_int;
        pub fn SDL_EventState(_type: uint8_t, state: c_int) -> uint8_t;
        pub fn SDL_GetKeyState(numkeys: *mut c_int) -> *mut uint8_t;
        pub fn SDL_GetModState() -> SDLMod;
        pub fn SDL_GetKeyName(key: SDLKey) -> *mut c_schar;
        pub fn SDL_JoystickEventState(state: c_int) -> c_int;
        pub fn SDL_GetAppState() -> uint8_t;
        pub fn SDL_EnableUNICODE(enable: c_int) -> c_int;
        pub fn SDL_EnableKeyRepeat(delay: c_int, interval: c_int) -> c_int;
        pub fn SDL_SetModState(modstate: SDLMod);
        pub fn SDL_PumpEvents();
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum AppState {
    MouseFocus = ll::SDL_APPMOUSEFOCUS as isize,
    InputFocus = ll::SDL_APPINPUTFOCUS as isize,
    Active = ll::SDL_APPACTIVE as isize
}

fn wrap_app_state(bitflags: u8) -> Vec<AppState> {
    let flags = [AppState::MouseFocus,
        AppState::InputFocus,
        AppState::Active];

    flags.iter().filter_map(|&flag| {
        if bitflags & (flag as u8) != 0 { Some(flag) }
        else { None }
    }).collect()
}

#[derive(PartialEq, Copy, Clone)]
pub enum RepeatDelay {
    Default,
    Custom(isize)
}

#[derive(PartialEq, Copy, Clone)]
pub enum RepeatInterval {
    Default,
    Custom(isize)
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum Key {
    Unknown = ll::SDLK_UNKNOWN as isize,
    Backspace = ll::SDLK_BACKSPACE as isize,
    Tab = ll::SDLK_TAB as isize,
    Clear = ll::SDLK_CLEAR as isize,
    Return = ll::SDLK_RETURN as isize,
    Pause = ll::SDLK_PAUSE as isize,
    Escape = ll::SDLK_ESCAPE as isize,
    Space = ll::SDLK_SPACE as isize,
    Exclaim = ll::SDLK_EXCLAIM as isize,
    Quotedbl = ll::SDLK_QUOTEDBL as isize,
    Hash = ll::SDLK_HASH as isize,
    Dollar = ll::SDLK_DOLLAR as isize,
    Ampersand = ll::SDLK_AMPERSAND as isize,
    Quote = ll::SDLK_QUOTE as isize,
    LeftParen = ll::SDLK_LEFTPAREN as isize,
    RightParen = ll::SDLK_RIGHTPAREN as isize,
    Asterisk = ll::SDLK_ASTERISK as isize,
    Plus = ll::SDLK_PLUS as isize,
    Comma = ll::SDLK_COMMA as isize,
    Minus = ll::SDLK_MINUS as isize,
    Period = ll::SDLK_PERIOD as isize,
    Slash = ll::SDLK_SLASH as isize,
    Num0 = ll::SDLK_0 as isize,
    Num1 = ll::SDLK_1 as isize,
    Num2 = ll::SDLK_2 as isize,
    Num3 = ll::SDLK_3 as isize,
    Num4 = ll::SDLK_4 as isize,
    Num5 = ll::SDLK_5 as isize,
    Num6 = ll::SDLK_6 as isize,
    Num7 = ll::SDLK_7 as isize,
    Num8 = ll::SDLK_8 as isize,
    Num9 = ll::SDLK_9 as isize,
    Colon = ll::SDLK_COLON as isize,
    Semicolon = ll::SDLK_SEMICOLON as isize,
    Less = ll::SDLK_LESS as isize,
    Equals = ll::SDLK_EQUALS as isize,
    Greater = ll::SDLK_GREATER as isize,
    Question = ll::SDLK_QUESTION as isize,
    At = ll::SDLK_AT as isize,
    LeftBracket = ll::SDLK_LEFTBRACKET as isize,
    Backslash = ll::SDLK_BACKSLASH as isize,
    RightBracket = ll::SDLK_RIGHTBRACKET as isize,
    Caret = ll::SDLK_CARET as isize,
    Underscore = ll::SDLK_UNDERSCORE as isize,
    Backquote = ll::SDLK_BACKQUOTE as isize,
    A = ll::SDLK_a as isize,
    B = ll::SDLK_b as isize,
    C = ll::SDLK_c as isize,
    D = ll::SDLK_d as isize,
    E = ll::SDLK_e as isize,
    F = ll::SDLK_f as isize,
    G = ll::SDLK_g as isize,
    H = ll::SDLK_h as isize,
    I = ll::SDLK_i as isize,
    J = ll::SDLK_j as isize,
    K = ll::SDLK_k as isize,
    L = ll::SDLK_l as isize,
    M = ll::SDLK_m as isize,
    N = ll::SDLK_n as isize,
    O = ll::SDLK_o as isize,
    P = ll::SDLK_p as isize,
    Q = ll::SDLK_q as isize,
    R = ll::SDLK_r as isize,
    S = ll::SDLK_s as isize,
    T = ll::SDLK_t as isize,
    U = ll::SDLK_u as isize,
    V = ll::SDLK_v as isize,
    W = ll::SDLK_w as isize,
    X = ll::SDLK_x as isize,
    Y = ll::SDLK_y as isize,
    Z = ll::SDLK_z as isize,
    Delete = ll::SDLK_DELETE as isize,
    World0 = ll::SDLK_WORLD_0 as isize,
    World1 = ll::SDLK_WORLD_1 as isize,
    World2 = ll::SDLK_WORLD_2 as isize,
    World3 = ll::SDLK_WORLD_3 as isize,
    World4 = ll::SDLK_WORLD_4 as isize,
    World5 = ll::SDLK_WORLD_5 as isize,
    World6 = ll::SDLK_WORLD_6 as isize,
    World7 = ll::SDLK_WORLD_7 as isize,
    World8 = ll::SDLK_WORLD_8 as isize,
    World9 = ll::SDLK_WORLD_9 as isize,
    World10 = ll::SDLK_WORLD_10 as isize,
    World11 = ll::SDLK_WORLD_11 as isize,
    World12 = ll::SDLK_WORLD_12 as isize,
    World13 = ll::SDLK_WORLD_13 as isize,
    World14 = ll::SDLK_WORLD_14 as isize,
    World15 = ll::SDLK_WORLD_15 as isize,
    World16 = ll::SDLK_WORLD_16 as isize,
    World17 = ll::SDLK_WORLD_17 as isize,
    World18 = ll::SDLK_WORLD_18 as isize,
    World19 = ll::SDLK_WORLD_19 as isize,
    World20 = ll::SDLK_WORLD_20 as isize,
    World21 = ll::SDLK_WORLD_21 as isize,
    World22 = ll::SDLK_WORLD_22 as isize,
    World23 = ll::SDLK_WORLD_23 as isize,
    World24 = ll::SDLK_WORLD_24 as isize,
    World25 = ll::SDLK_WORLD_25 as isize,
    World26 = ll::SDLK_WORLD_26 as isize,
    World27 = ll::SDLK_WORLD_27 as isize,
    World28 = ll::SDLK_WORLD_28 as isize,
    World29 = ll::SDLK_WORLD_29 as isize,
    World30 = ll::SDLK_WORLD_30 as isize,
    World31 = ll::SDLK_WORLD_31 as isize,
    World32 = ll::SDLK_WORLD_32 as isize,
    World33 = ll::SDLK_WORLD_33 as isize,
    World34 = ll::SDLK_WORLD_34 as isize,
    World35 = ll::SDLK_WORLD_35 as isize,
    World36 = ll::SDLK_WORLD_36 as isize,
    World37 = ll::SDLK_WORLD_37 as isize,
    World38 = ll::SDLK_WORLD_38 as isize,
    World39 = ll::SDLK_WORLD_39 as isize,
    World40 = ll::SDLK_WORLD_40 as isize,
    World41 = ll::SDLK_WORLD_41 as isize,
    World42 = ll::SDLK_WORLD_42 as isize,
    World43 = ll::SDLK_WORLD_43 as isize,
    World44 = ll::SDLK_WORLD_44 as isize,
    World45 = ll::SDLK_WORLD_45 as isize,
    World46 = ll::SDLK_WORLD_46 as isize,
    World47 = ll::SDLK_WORLD_47 as isize,
    World48 = ll::SDLK_WORLD_48 as isize,
    World49 = ll::SDLK_WORLD_49 as isize,
    World50 = ll::SDLK_WORLD_50 as isize,
    World51 = ll::SDLK_WORLD_51 as isize,
    World52 = ll::SDLK_WORLD_52 as isize,
    World53 = ll::SDLK_WORLD_53 as isize,
    World54 = ll::SDLK_WORLD_54 as isize,
    World55 = ll::SDLK_WORLD_55 as isize,
    World56 = ll::SDLK_WORLD_56 as isize,
    World57 = ll::SDLK_WORLD_57 as isize,
    World58 = ll::SDLK_WORLD_58 as isize,
    World59 = ll::SDLK_WORLD_59 as isize,
    World60 = ll::SDLK_WORLD_60 as isize,
    World61 = ll::SDLK_WORLD_61 as isize,
    World62 = ll::SDLK_WORLD_62 as isize,
    World63 = ll::SDLK_WORLD_63 as isize,
    World64 = ll::SDLK_WORLD_64 as isize,
    World65 = ll::SDLK_WORLD_65 as isize,
    World66 = ll::SDLK_WORLD_66 as isize,
    World67 = ll::SDLK_WORLD_67 as isize,
    World68 = ll::SDLK_WORLD_68 as isize,
    World69 = ll::SDLK_WORLD_69 as isize,
    World70 = ll::SDLK_WORLD_70 as isize,
    World71 = ll::SDLK_WORLD_71 as isize,
    World72 = ll::SDLK_WORLD_72 as isize,
    World73 = ll::SDLK_WORLD_73 as isize,
    World74 = ll::SDLK_WORLD_74 as isize,
    World75 = ll::SDLK_WORLD_75 as isize,
    World76 = ll::SDLK_WORLD_76 as isize,
    World77 = ll::SDLK_WORLD_77 as isize,
    World78 = ll::SDLK_WORLD_78 as isize,
    World79 = ll::SDLK_WORLD_79 as isize,
    World80 = ll::SDLK_WORLD_80 as isize,
    World81 = ll::SDLK_WORLD_81 as isize,
    World82 = ll::SDLK_WORLD_82 as isize,
    World83 = ll::SDLK_WORLD_83 as isize,
    World84 = ll::SDLK_WORLD_84 as isize,
    World85 = ll::SDLK_WORLD_85 as isize,
    World86 = ll::SDLK_WORLD_86 as isize,
    World87 = ll::SDLK_WORLD_87 as isize,
    World88 = ll::SDLK_WORLD_88 as isize,
    World89 = ll::SDLK_WORLD_89 as isize,
    World90 = ll::SDLK_WORLD_90 as isize,
    World91 = ll::SDLK_WORLD_91 as isize,
    World92 = ll::SDLK_WORLD_92 as isize,
    World93 = ll::SDLK_WORLD_93 as isize,
    World94 = ll::SDLK_WORLD_94 as isize,
    World95 = ll::SDLK_WORLD_95 as isize,
    Kp0 = ll::SDLK_KP0 as isize,
    Kp1 = ll::SDLK_KP1 as isize,
    Kp2 = ll::SDLK_KP2 as isize,
    Kp3 = ll::SDLK_KP3 as isize,
    Kp4 = ll::SDLK_KP4 as isize,
    Kp5 = ll::SDLK_KP5 as isize,
    Kp6 = ll::SDLK_KP6 as isize,
    Kp7 = ll::SDLK_KP7 as isize,
    Kp8 = ll::SDLK_KP8 as isize,
    Kp9 = ll::SDLK_KP9 as isize,
    KpPeriod = ll::SDLK_KP_PERIOD as isize,
    KpDivide = ll::SDLK_KP_DIVIDE as isize,
    KpMultiply = ll::SDLK_KP_MULTIPLY as isize,
    KpMinus = ll::SDLK_KP_MINUS as isize,
    KpPlus = ll::SDLK_KP_PLUS as isize,
    KpEnter = ll::SDLK_KP_ENTER as isize,
    KpEquals = ll::SDLK_KP_EQUALS as isize,
    Up = ll::SDLK_UP as isize,
    Down = ll::SDLK_DOWN as isize,
    Right = ll::SDLK_RIGHT as isize,
    Left = ll::SDLK_LEFT as isize,
    Insert = ll::SDLK_INSERT as isize,
    Home = ll::SDLK_HOME as isize,
    End = ll::SDLK_END as isize,
    PageUp = ll::SDLK_PAGEUP as isize,
    PageDown = ll::SDLK_PAGEDOWN as isize,
    F1 = ll::SDLK_F1 as isize,
    F2 = ll::SDLK_F2 as isize,
    F3 = ll::SDLK_F3 as isize,
    F4 = ll::SDLK_F4 as isize,
    F5 = ll::SDLK_F5 as isize,
    F6 = ll::SDLK_F6 as isize,
    F7 = ll::SDLK_F7 as isize,
    F8 = ll::SDLK_F8 as isize,
    F9 = ll::SDLK_F9 as isize,
    F10 = ll::SDLK_F10 as isize,
    F11 = ll::SDLK_F11 as isize,
    F12 = ll::SDLK_F12 as isize,
    F13 = ll::SDLK_F13 as isize,
    F14 = ll::SDLK_F14 as isize,
    F15 = ll::SDLK_F15 as isize,
    NumLock = ll::SDLK_NUMLOCK as isize,
    CapsLock = ll::SDLK_CAPSLOCK as isize,
    ScrolLock = ll::SDLK_SCROLLOCK as isize,
    RShift = ll::SDLK_RSHIFT as isize,
    LShift = ll::SDLK_LSHIFT as isize,
    RCtrl = ll::SDLK_RCTRL as isize,
    LCtrl = ll::SDLK_LCTRL as isize,
    RAlt = ll::SDLK_RALT as isize,
    LAlt = ll::SDLK_LALT as isize,
    RMeta = ll::SDLK_RMETA as isize,
    LMeta = ll::SDLK_LMETA as isize,
    LSuper = ll::SDLK_LSUPER as isize,
    RSuper = ll::SDLK_RSUPER as isize,
    Mode = ll::SDLK_MODE as isize,
    Compose = ll::SDLK_COMPOSE as isize,
    Help = ll::SDLK_HELP as isize,
    Print = ll::SDLK_PRINT as isize,
    SysReq = ll::SDLK_SYSREQ as isize,
    Break = ll::SDLK_BREAK as isize,
    Menu = ll::SDLK_MENU as isize,
    Power = ll::SDLK_POWER as isize,
    Euro = ll::SDLK_EURO as isize,
    Undo = ll::SDLK_UNDO as isize,
    Last,
}

impl FromPrimitive for Key {
    fn from_i64(n: i64) -> Option<Key> {
        use self::Key::*;

        Some(match n as ll::SDLKey {
            ll::SDLK_UNKNOWN => Unknown,
            ll::SDLK_BACKSPACE => Backspace,
            ll::SDLK_TAB => Tab,
            ll::SDLK_CLEAR => Clear,
            ll::SDLK_RETURN => Return,
            ll::SDLK_PAUSE => Pause,
            ll::SDLK_ESCAPE => Escape,
            ll::SDLK_SPACE => Space,
            ll::SDLK_EXCLAIM => Exclaim,
            ll::SDLK_QUOTEDBL => Quotedbl,
            ll::SDLK_HASH => Hash,
            ll::SDLK_DOLLAR => Dollar,
            ll::SDLK_AMPERSAND => Ampersand,
            ll::SDLK_QUOTE => Quote,
            ll::SDLK_LEFTPAREN => LeftParen,
            ll::SDLK_RIGHTPAREN => RightParen,
            ll::SDLK_ASTERISK => Asterisk,
            ll::SDLK_PLUS => Plus,
            ll::SDLK_COMMA => Comma,
            ll::SDLK_MINUS => Minus,
            ll::SDLK_PERIOD => Period,
            ll::SDLK_SLASH => Slash,
            ll::SDLK_0 => Num0,
            ll::SDLK_1 => Num1,
            ll::SDLK_2 => Num2,
            ll::SDLK_3 => Num3,
            ll::SDLK_4 => Num4,
            ll::SDLK_5 => Num5,
            ll::SDLK_6 => Num6,
            ll::SDLK_7 => Num7,
            ll::SDLK_8 => Num8,
            ll::SDLK_9 => Num9,
            ll::SDLK_COLON => Colon,
            ll::SDLK_SEMICOLON => Semicolon,
            ll::SDLK_LESS => Less,
            ll::SDLK_EQUALS => Equals,
            ll::SDLK_GREATER => Greater,
            ll::SDLK_QUESTION => Question,
            ll::SDLK_AT => At,
            ll::SDLK_LEFTBRACKET => LeftBracket,
            ll::SDLK_BACKSLASH => Backslash,
            ll::SDLK_RIGHTBRACKET => RightBracket,
            ll::SDLK_CARET => Caret,
            ll::SDLK_UNDERSCORE => Underscore,
            ll::SDLK_BACKQUOTE => Backquote,
            ll::SDLK_a => A,
            ll::SDLK_b => B,
            ll::SDLK_c => C,
            ll::SDLK_d => D,
            ll::SDLK_e => E,
            ll::SDLK_f => F,
            ll::SDLK_g => G,
            ll::SDLK_h => H,
            ll::SDLK_i => I,
            ll::SDLK_j => J,
            ll::SDLK_k => K,
            ll::SDLK_l => L,
            ll::SDLK_m => M,
            ll::SDLK_n => N,
            ll::SDLK_o => O,
            ll::SDLK_p => P,
            ll::SDLK_q => Q,
            ll::SDLK_r => R,
            ll::SDLK_s => S,
            ll::SDLK_t => T,
            ll::SDLK_u => U,
            ll::SDLK_v => V,
            ll::SDLK_w => W,
            ll::SDLK_x => X,
            ll::SDLK_y => Y,
            ll::SDLK_z => Z,
            ll::SDLK_DELETE => Delete,
            ll::SDLK_WORLD_0 => World0,
            ll::SDLK_WORLD_1 => World1,
            ll::SDLK_WORLD_2 => World2,
            ll::SDLK_WORLD_3 => World3,
            ll::SDLK_WORLD_4 => World4,
            ll::SDLK_WORLD_5 => World5,
            ll::SDLK_WORLD_6 => World6,
            ll::SDLK_WORLD_7 => World7,
            ll::SDLK_WORLD_8 => World8,
            ll::SDLK_WORLD_9 => World9,
            ll::SDLK_WORLD_10 => World10,
            ll::SDLK_WORLD_11 => World11,
            ll::SDLK_WORLD_12 => World12,
            ll::SDLK_WORLD_13 => World13,
            ll::SDLK_WORLD_14 => World14,
            ll::SDLK_WORLD_15 => World15,
            ll::SDLK_WORLD_16 => World16,
            ll::SDLK_WORLD_17 => World17,
            ll::SDLK_WORLD_18 => World18,
            ll::SDLK_WORLD_19 => World19,
            ll::SDLK_WORLD_20 => World20,
            ll::SDLK_WORLD_21 => World21,
            ll::SDLK_WORLD_22 => World22,
            ll::SDLK_WORLD_23 => World23,
            ll::SDLK_WORLD_24 => World24,
            ll::SDLK_WORLD_25 => World25,
            ll::SDLK_WORLD_26 => World26,
            ll::SDLK_WORLD_27 => World27,
            ll::SDLK_WORLD_28 => World28,
            ll::SDLK_WORLD_29 => World29,
            ll::SDLK_WORLD_30 => World30,
            ll::SDLK_WORLD_31 => World31,
            ll::SDLK_WORLD_32 => World32,
            ll::SDLK_WORLD_33 => World33,
            ll::SDLK_WORLD_34 => World34,
            ll::SDLK_WORLD_35 => World35,
            ll::SDLK_WORLD_36 => World36,
            ll::SDLK_WORLD_37 => World37,
            ll::SDLK_WORLD_38 => World38,
            ll::SDLK_WORLD_39 => World39,
            ll::SDLK_WORLD_40 => World40,
            ll::SDLK_WORLD_41 => World41,
            ll::SDLK_WORLD_42 => World42,
            ll::SDLK_WORLD_43 => World43,
            ll::SDLK_WORLD_44 => World44,
            ll::SDLK_WORLD_45 => World45,
            ll::SDLK_WORLD_46 => World46,
            ll::SDLK_WORLD_47 => World47,
            ll::SDLK_WORLD_48 => World48,
            ll::SDLK_WORLD_49 => World49,
            ll::SDLK_WORLD_50 => World50,
            ll::SDLK_WORLD_51 => World51,
            ll::SDLK_WORLD_52 => World52,
            ll::SDLK_WORLD_53 => World53,
            ll::SDLK_WORLD_54 => World54,
            ll::SDLK_WORLD_55 => World55,
            ll::SDLK_WORLD_56 => World56,
            ll::SDLK_WORLD_57 => World57,
            ll::SDLK_WORLD_58 => World58,
            ll::SDLK_WORLD_59 => World59,
            ll::SDLK_WORLD_60 => World60,
            ll::SDLK_WORLD_61 => World61,
            ll::SDLK_WORLD_62 => World62,
            ll::SDLK_WORLD_63 => World63,
            ll::SDLK_WORLD_64 => World64,
            ll::SDLK_WORLD_65 => World65,
            ll::SDLK_WORLD_66 => World66,
            ll::SDLK_WORLD_67 => World67,
            ll::SDLK_WORLD_68 => World68,
            ll::SDLK_WORLD_69 => World69,
            ll::SDLK_WORLD_70 => World70,
            ll::SDLK_WORLD_71 => World71,
            ll::SDLK_WORLD_72 => World72,
            ll::SDLK_WORLD_73 => World73,
            ll::SDLK_WORLD_74 => World74,
            ll::SDLK_WORLD_75 => World75,
            ll::SDLK_WORLD_76 => World76,
            ll::SDLK_WORLD_77 => World77,
            ll::SDLK_WORLD_78 => World78,
            ll::SDLK_WORLD_79 => World79,
            ll::SDLK_WORLD_80 => World80,
            ll::SDLK_WORLD_81 => World81,
            ll::SDLK_WORLD_82 => World82,
            ll::SDLK_WORLD_83 => World83,
            ll::SDLK_WORLD_84 => World84,
            ll::SDLK_WORLD_85 => World85,
            ll::SDLK_WORLD_86 => World86,
            ll::SDLK_WORLD_87 => World87,
            ll::SDLK_WORLD_88 => World88,
            ll::SDLK_WORLD_89 => World89,
            ll::SDLK_WORLD_90 => World90,
            ll::SDLK_WORLD_91 => World91,
            ll::SDLK_WORLD_92 => World92,
            ll::SDLK_WORLD_93 => World93,
            ll::SDLK_WORLD_94 => World94,
            ll::SDLK_WORLD_95 => World95,
            ll::SDLK_KP0 => Kp0,
            ll::SDLK_KP1 => Kp1,
            ll::SDLK_KP2 => Kp2,
            ll::SDLK_KP3 => Kp3,
            ll::SDLK_KP4 => Kp4,
            ll::SDLK_KP5 => Kp5,
            ll::SDLK_KP6 => Kp6,
            ll::SDLK_KP7 => Kp7,
            ll::SDLK_KP8 => Kp8,
            ll::SDLK_KP9 => Kp9,
            ll::SDLK_KP_PERIOD => KpPeriod,
            ll::SDLK_KP_DIVIDE => KpDivide,
            ll::SDLK_KP_MULTIPLY => KpMultiply,
            ll::SDLK_KP_MINUS => KpMinus,
            ll::SDLK_KP_PLUS => KpPlus,
            ll::SDLK_KP_ENTER => KpEnter,
            ll::SDLK_KP_EQUALS => KpEquals,
            ll::SDLK_UP => Up,
            ll::SDLK_DOWN => Down,
            ll::SDLK_RIGHT => Right,
            ll::SDLK_LEFT => Left,
            ll::SDLK_INSERT => Insert,
            ll::SDLK_HOME => Home,
            ll::SDLK_END => End,
            ll::SDLK_PAGEUP => PageUp,
            ll::SDLK_PAGEDOWN => PageDown,
            ll::SDLK_F1 => F1,
            ll::SDLK_F2 => F2,
            ll::SDLK_F3 => F3,
            ll::SDLK_F4 => F4,
            ll::SDLK_F5 => F5,
            ll::SDLK_F6 => F6,
            ll::SDLK_F7 => F7,
            ll::SDLK_F8 => F8,
            ll::SDLK_F9 => F9,
            ll::SDLK_F10 => F10,
            ll::SDLK_F11 => F11,
            ll::SDLK_F12 => F12,
            ll::SDLK_F13 => F13,
            ll::SDLK_F14 => F14,
            ll::SDLK_F15 => F15,
            ll::SDLK_NUMLOCK => NumLock,
            ll::SDLK_CAPSLOCK => CapsLock,
            ll::SDLK_SCROLLOCK => ScrolLock,
            ll::SDLK_RSHIFT => RShift,
            ll::SDLK_LSHIFT => LShift,
            ll::SDLK_RCTRL => RCtrl,
            ll::SDLK_LCTRL => LCtrl,
            ll::SDLK_RALT => RAlt,
            ll::SDLK_LALT => LAlt,
            ll::SDLK_RMETA => RMeta,
            ll::SDLK_LMETA => LMeta,
            ll::SDLK_LSUPER => LSuper,
            ll::SDLK_RSUPER => RSuper,
            ll::SDLK_MODE => Mode,
            ll::SDLK_COMPOSE => Compose,
            ll::SDLK_HELP => Help,
            ll::SDLK_PRINT => Print,
            ll::SDLK_SYSREQ => SysReq,
            ll::SDLK_BREAK => Break,
            ll::SDLK_MENU => Menu,
            ll::SDLK_POWER => Power,
            ll::SDLK_EURO => Euro,
            ll::SDLK_UNDO => Undo,
            _ => return None,
        })
    }

    fn from_u64(n: u64) -> Option<Key> { FromPrimitive::from_i64(n as i64) }
}

fn wrap_key(i: ll::SDLKey) -> Option<Key> {
    FromPrimitive::from_usize(i as usize)
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Mod {
     None = 0x0000,
     LShift = 0x0001,
     RShift = 0x0002,
     LCtrl = 0x0040,
     RCtrl = 0x0080,
     LAlt = 0x0100,
     RAlt = 0x0200,
     LMeta = 0x0400,
     RMeta = 0x0800,
     Num = 0x1000,
     Caps = 0x2000,
     Mode = 0x4000,
     Reserved = 0x8000
}

fn wrap_mod_state(bitflags: ll::SDLMod) -> Vec<Mod> {
    let flags = [Mod::None,
        Mod::LShift,
        Mod::RShift,
        Mod::LCtrl,
        Mod::RCtrl,
        Mod::LAlt,
        Mod::RAlt,
        Mod::LMeta,
        Mod::RMeta,
        Mod::Num,
        Mod::Caps,
        Mod::Mode,
        Mod::Reserved];

    flags.iter().filter_map(|&flag| {
        if bitflags & (flag as ll::SDLMod) != 0 { Some(flag) }
        else { None }
    }).collect()
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum HatState {
    Centered,
    Up,
    Right,
    Down,
    Left
}

fn wrap_hat_state(bitflags: u8) -> Vec<HatState> {
    let flags = [HatState::Centered,
        HatState::Up,
        HatState::Right,
        HatState::Down,
        HatState::Left];

    flags.iter().filter_map(|&flag| {
        if bitflags & (flag as u8) != 0 { Some(flag) }
        else { None }
    }).collect()
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Mouse {
    Left = 1,
    Middle,
    Right,
    WheelUp,
    WheelDown
}

impl FromPrimitive for Mouse {
    fn from_i64(n: i64) -> Option<Mouse> {
        Some(match n {
            1 => Mouse::Left,
            2 => Mouse::Middle,
            3 => Mouse::Right,
            4 => Mouse::WheelUp,
            5 => Mouse::WheelDown,
            _ => return None,
        })
    }

    fn from_u64(n: u64) -> Option<Mouse> { FromPrimitive::from_i64(n as i64) }
}

fn wrap_mouse(bitflags: u8) -> Option<Mouse> {
    FromPrimitive::from_u8(bitflags)
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum MouseState {
    Left = 1,
    Middle,
    Right,
    WheelUp,
    WheelDown,
    X1,
    X2
}

fn wrap_mouse_state(bitflags: u8) -> Vec<MouseState> {
    let flags = [MouseState::Left,
        MouseState::Middle,
        MouseState::Right,
        MouseState::WheelUp,
        MouseState::WheelDown,
        MouseState::X1,
        MouseState::X2];

    flags.iter().filter_map(|&flag| {
        if bitflags & (flag as u8) != 0 { Some(flag) }
        else { None }
    }).collect()
}

#[derive(PartialEq)]
pub enum Event {
    // TODO: TextInput, TextEditing
     None,
     Active(bool, Vec<AppState>),
     Key(Key, bool, Vec<Mod>, u16), // RFC: do you need the scancode?
     MouseMotion(Vec<MouseState>, u16, u16, i16, i16),
     MouseButton(Mouse, bool, u16, u16),
     JoyAxis(isize, isize, i16),
     JoyBall(isize, isize, i16, i16),
     JoyHat(isize, isize, Vec<HatState>),
     JoyButton(isize, isize, bool),
     Quit,
    // TODO: SysWm
     Resize(isize, isize),
     Expose,
    // TODO: User
}

fn null_event() -> ll::SDL_Event {
    ll::SDL_Event { data: [0; 24] }
}

fn wrap_event(raw: ll::SDL_Event) -> Event {
    unsafe {
        let ty = raw._type();
        let ty = if ty.is_null() { return Event::None; }
                 else { *ty };

        let ty : EventType = match FromPrimitive::from_usize(ty as usize) {
            Some(ty) => ty,
            None => return Event::None
        };

        match ty {
            EventType::None => Event::None,
            EventType::Active => {
                let active = raw.active();
                let active = if active.is_null() { return Event::None; }
                             else { *active };

                Event::Active(active.gain == 1, wrap_app_state(active.state))
            }
            EventType::KeyDown | EventType::KeyUp => {
                let key = raw.key();
                let (key, okey) = if key.is_null() { return Event::None; }
                          else { ((*key).keysym, *key) };

                match wrap_key(key.sym) {
                    Some(sym) => {
                        Event::Key(sym, okey.state == 1, wrap_mod_state(key._mod),
                                 key.unicode)
                    }
                    None => Event::None
                }
            }
            EventType::MouseMotion => {
                let motion = raw.motion();
                let motion = if motion.is_null() { return Event::None; }
                             else { *motion };

                Event::MouseMotion(wrap_mouse_state(motion.state), motion.x,
                                 motion.y, motion.xrel, motion.yrel)
            }
            EventType::MouseButtonDown | EventType::MouseButtonUp => {
                let obutton = raw.button();
                let obutton = if obutton.is_null() { return Event::None; }
                             else { *obutton };

                match wrap_mouse(obutton.button) {
                    Some(button) => {
                        Event::MouseButton(button, obutton.state == 1,
                                 obutton.x, obutton.y)
                    }
                    None => Event::None
                }
            }
            EventType::JoyAxisMotion => {
                let jaxis = raw.jaxis();
                let jaxis = if jaxis.is_null() { return Event::None; }
                            else { *jaxis };

                Event::JoyAxis(jaxis.which as isize, jaxis.axis as isize,
                             jaxis.value)
            }
            EventType::JoyBallMotion => {
                let jball = raw.jball();
                let jball = if jball.is_null() { return Event::None; }
                            else { *jball };

                Event::JoyBall(jball.which as isize, jball.ball as isize,
                             jball.xrel, jball.yrel)
            }
            EventType::JoyHatMotion => {
                let jhat = raw.jhat();
                let jhat = if jhat.is_null() { return Event::None; }
                           else { *jhat };

                Event::JoyHat(jhat.which as isize, jhat.hat as isize,
                            wrap_hat_state(jhat.value))
            }
            EventType::JoyButtonDown | EventType::JoyButtonUp => {
                let jbutton = raw.jbutton();
                let jbutton = if jbutton.is_null() { return Event::None; }
                              else { *jbutton };

                Event::JoyButton(jbutton.which as isize, jbutton.button as isize,
                               jbutton.state == 1u8)
            }
            EventType::Quit => Event::Quit,
            EventType::Resize => {
                let resize = raw.resize();
                let resize = if resize.is_null() { return Event::None; }
                             else { *resize };

                Event::Resize(resize.w as isize, resize.h as isize)
            }
            EventType::Expose => Event::Expose,
            _ => Event::None
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum EventType {
    // TODO: TextInput, TextEditing
    None = ll::SDL_NOEVENT as isize,
    Active = ll::SDL_ACTIVEEVENT as isize,
    KeyDown = ll::SDL_KEYDOWN as isize,
    KeyUp = ll::SDL_KEYUP as isize,
    MouseMotion = ll::SDL_MOUSEMOTION as isize,
    MouseButtonDown = ll::SDL_MOUSEBUTTONDOWN as isize,
    MouseButtonUp = ll::SDL_MOUSEBUTTONUP as isize,
    JoyAxisMotion = ll::SDL_JOYAXISMOTION as isize,
    JoyBallMotion = ll::SDL_JOYBALLMOTION as isize,
    JoyHatMotion = ll::SDL_JOYHATMOTION as isize,
    JoyButtonDown = ll::SDL_JOYBUTTONDOWN as isize,
    JoyButtonUp = ll::SDL_JOYBUTTONUP as isize,
    Quit = ll::SDL_QUIT as isize,
    SysWM = ll::SDL_SYSWMEVENT as isize,
    Resize = ll::SDL_VIDEORESIZE as isize,
    Expose = ll::SDL_VIDEOEXPOSE as isize,
    User = ll::SDL_USEREVENT as isize,
}

impl EventType {
    pub fn get_state(&self) -> bool { get_event_state(*self) }
    pub fn set_state(&self, state: bool) { set_event_state(*self, state) }
}

impl FromPrimitive for EventType {
    fn from_i64(n: i64) -> Option<EventType> {
        Some(match n as ll::SDL_EventType {
            ll::SDL_NOEVENT => EventType::None,
            ll::SDL_ACTIVEEVENT => EventType::Active,
            ll::SDL_KEYDOWN => EventType::KeyDown,
            ll::SDL_KEYUP => EventType::KeyUp,
            ll::SDL_MOUSEMOTION => EventType::MouseMotion,
            ll::SDL_MOUSEBUTTONDOWN => EventType::MouseButtonDown,
            ll::SDL_MOUSEBUTTONUP => EventType::MouseButtonUp,
            ll::SDL_JOYAXISMOTION => EventType::JoyAxisMotion,
            ll::SDL_JOYBALLMOTION => EventType::JoyBallMotion,
            ll::SDL_JOYHATMOTION => EventType::JoyHatMotion,
            ll::SDL_JOYBUTTONDOWN => EventType::JoyButtonDown,
            ll::SDL_JOYBUTTONUP => EventType::JoyButtonUp,
            ll::SDL_QUIT => EventType::Quit,
            ll::SDL_SYSWMEVENT => EventType::SysWM,
            ll::SDL_VIDEORESIZE => EventType::Resize,
            ll::SDL_VIDEOEXPOSE => EventType::Expose,
            ll::SDL_USEREVENT => EventType::User,
            _ => return None,
        })
    }

    fn from_u64(n: u64) -> Option<EventType> { FromPrimitive::from_i64(n as i64) }
}

pub fn pump_events() {
    unsafe { ll::SDL_PumpEvents(); }
}

// TODO: peep_events (a tricky one but doable)

pub fn wait_event() -> Event {
    let mut raw = null_event();
    let success = unsafe { ll::SDL_WaitEvent(&mut raw)
                            == 1 as c_int };

    if success { wrap_event(raw) }
    else { Event::None }
}

pub fn poll_event() -> Event {
    pump_events();

    let mut raw = null_event();
    let have = unsafe { ll::SDL_PollEvent(&mut raw) };

    if have != 1 {
        return Event::None;
    }

    wrap_event(raw)
}

// TODO: set_event_filter, get_event_filter

pub fn set_event_state(ty: EventType, state: bool) {
    unsafe { ll::SDL_EventState(ty as u8, state as c_int); }
}

pub fn get_event_state(ty: EventType) -> bool {
    unsafe { ll::SDL_EventState(ty as u8, ll::SDL_QUERY as c_int)
             == ll::SDL_ENABLE as u8 }
}

pub fn get_key_state() -> Vec<(Key, bool)> {
    let mut num = 0;
    let data = unsafe { ll::SDL_GetKeyState(&mut num) };
    let mut i = -1isize;

    let buf = data as *const u8;
    let buf = unsafe { slice::from_raw_parts(buf, num as usize) };
    buf.iter().filter_map(|&state| {
        i += 1;

        match wrap_key(i as ll::SDLKey) {
            Some(key) => Some((key, state == 1)),
            None => None
        }
    }).collect()
}

pub fn get_mod_state() -> Vec<Mod> {
    unsafe { wrap_mod_state(ll::SDL_GetModState()) }
}

pub fn set_mod_state(states: &[Mod]) {
    unsafe {
        ll::SDL_SetModState(states.iter().fold(0u32, |states, &state| {
            states | state as ll::SDLMod
        }));
    }
}

pub fn get_key_name(key: Key) -> String {
    unsafe {
        let cstr = ll::SDL_GetKeyName(key as ll::SDLKey);

        str::from_utf8(CStr::from_ptr(mem::transmute_copy(&cstr)).to_bytes()).unwrap().to_string()
    }
}

pub fn set_joystick_event_state(state: bool) {
    unsafe {
        ll::SDL_JoystickEventState(state as c_int);
    }
}

pub fn get_joystick_event_state() -> bool {
    unsafe {
        ll::SDL_JoystickEventState(ll::SDL_QUERY as c_int) == ll::SDL_ENABLE as c_int
    }
}

pub fn toggle_joystick_event_state() {
    set_joystick_event_state(!get_joystick_event_state());
}

pub fn get_app_state() -> Vec<AppState> {
    let bitflags = unsafe { ll::SDL_GetAppState() };

    wrap_app_state(bitflags)
}

pub fn enable_unicode(enable: bool) {
    unsafe { ll::SDL_EnableUNICODE(enable as c_int); }
}

pub fn is_unicode_enabled() -> bool {
    unsafe { ll::SDL_EnableUNICODE(-1 as c_int) == 1 }
}

pub fn enable_key_repeat(delay: RepeatDelay, interval: RepeatInterval) -> bool {
    let delay = match delay {
        RepeatDelay::Default => 500,
        RepeatDelay::Custom(delay) => delay
    };
    let interval = match interval {
        RepeatInterval::Default => 30,
        RepeatInterval::Custom(interval) => interval
    };

    unsafe {
        ll::SDL_EnableKeyRepeat(delay as c_int, interval as c_int) == 0 as c_int
    }
}

// get_mouse_state, get_relative_mouse_state, start_text_input, stop_text_input, set_text_input_rect
