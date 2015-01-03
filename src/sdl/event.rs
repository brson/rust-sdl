use std::mem;
use libc::c_int;
use std::slice;
use std::num::FromPrimitive;

pub mod ll {
    #![allow(non_camel_case_types)]

    use std::mem;
    use libc::{c_void, c_int, c_uint, c_uchar, uint8_t, uint16_t, int16_t};
    use libc::types::os::arch::c95::c_schar;

    pub type SDLKey = c_uint;
    pub type SDLMod = c_uint;
    pub type SDL_SysWMmsg = c_void;

    pub const SDL_DISABLE: c_int = 0;
    pub const SDL_ENABLE: c_int = 1;
    pub const SDL_QUERY: c_int = -1;

    pub const SDL_APPMOUSEFOCUS: c_int = 0x01;
    pub const SDL_APPINPUTFOCUS: c_int = 0x02;
    pub const SDL_APPACTIVE: c_int = 0x04;

    #[repr(C)]
    pub struct SDL_keysym {
        pub scancode: uint8_t,
        pub sym: SDLKey,
        pub _mod: SDLMod,
        pub unicode: uint16_t,
    }

    impl Copy for SDL_keysym {}

    #[repr(C)]
    pub struct SDL_Event {
        pub data: [c_uchar; 24],
    }

    impl Copy for SDL_Event {}

    #[repr(C)]
    pub struct SDL_ActiveEvent {
        pub _type: uint8_t,
        pub gain: uint8_t,
        pub state: uint8_t,
    }

    impl Copy for SDL_ActiveEvent {}

    #[repr(C)]
    pub struct SDL_KeyboardEvent {
        pub _type: uint8_t,
        pub which: uint8_t,
        pub state: uint8_t,
        pub keysym: SDL_keysym,
    }

    impl Copy for SDL_KeyboardEvent {}

    #[repr(C)]
    pub struct SDL_MouseMotionEvent {
        pub _type: uint8_t,
        pub which: uint8_t,
        pub state: uint8_t,
        pub x: uint16_t,
        pub y: uint16_t,
        pub xrel: int16_t,
        pub yrel: int16_t,
    }

    impl Copy for SDL_MouseMotionEvent {}

    #[repr(C)]
    pub struct SDL_MouseButtonEvent {
        pub _type: uint8_t,
        pub which: uint8_t,
        pub button: uint8_t,
        pub state: uint8_t,
        pub x: uint16_t,
        pub y: uint16_t,
    }

    impl Copy for SDL_MouseButtonEvent {}

    #[repr(C)]
    pub struct SDL_JoyAxisEvent {
        pub _type: uint8_t,
        pub which: uint8_t,
        pub axis: uint8_t,
        pub value: int16_t,
    }

    impl Copy for SDL_JoyAxisEvent {}

    #[repr(C)]
    pub struct SDL_JoyBallEvent {
        pub _type: uint8_t,
        pub which: uint8_t,
        pub ball: uint8_t,
        pub xrel: int16_t,
        pub yrel: int16_t,
    }

    impl Copy for SDL_JoyBallEvent {}

    #[repr(C)]
    pub struct SDL_JoyHatEvent {
        pub _type: uint8_t,
        pub which: uint8_t,
        pub hat: uint8_t,
        pub value: uint8_t,
    }

    impl Copy for SDL_JoyHatEvent {}

    #[repr(C)]
    pub struct SDL_JoyButtonEvent {
        pub _type: uint8_t,
        pub which: uint8_t,
        pub button: uint8_t,
        pub state: uint8_t,
    }

    impl Copy for SDL_JoyButtonEvent {}

    #[repr(C)]
    pub struct SDL_ResizeEvent {
        pub _type: uint8_t,
        pub w: c_int,
        pub h: c_int,
    }

    impl Copy for SDL_ResizeEvent {}

    #[repr(C)]
    pub struct SDL_ExposeEvent {
        pub _type: uint8_t,
    }

    impl Copy for SDL_ExposeEvent {}

    #[repr(C)]
    pub struct SDL_QuitEvent {
        pub _type: uint8_t,
    }

    impl Copy for SDL_QuitEvent {}

    #[repr(C)]
    pub struct SDL_UserEvent {
        pub _type: uint8_t,
        pub code: c_int,
        pub data1: *mut c_void,
        pub data2: *mut c_void,
    }

    impl Copy for SDL_UserEvent {}

    #[repr(C)]
    pub struct SDL_SysWMEvent {
        pub _type: uint8_t,
        pub msg: *mut SDL_SysWMmsg,
    }

    impl Copy for SDL_SysWMEvent {}

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

#[deriving(PartialEq, Eq)]
pub enum AppState {
    MouseFocus = ll::SDL_APPMOUSEFOCUS as int,
    InputFocus = ll::SDL_APPINPUTFOCUS as int,
    Active = ll::SDL_APPACTIVE as int
}

impl Copy for AppState {}

fn wrap_app_state(bitflags: u8) -> Vec<AppState> {
    let flags = [AppState::MouseFocus,
        AppState::InputFocus,
        AppState::Active];

    flags.iter().filter_map(|&flag| {
        if bitflags & (flag as u8) != 0 { Some(flag) }
        else { None }
    }).collect()
}

#[deriving(PartialEq)]
pub enum RepeatDelay {
    Default,
    Custom(int)
}

impl Copy for RepeatDelay {}

#[deriving(PartialEq)]
pub enum RepeatInterval {
    Default,
    Custom(int)
}

impl Copy for RepeatInterval {}

#[deriving(PartialEq, Eq, FromPrimitive, PartialOrd, Ord)]
pub enum Key {
    Unknown = 0,
    Backspace = 8,
    Tab = 9,
    Clear = 12,
    Return = 13,
    Pause = 19,
    Escape = 27,
    Space = 32,
    Exclaim = 33,
    Quotedbl = 34,
    Hash = 35,
    Dollar = 36,
    Ampersand = 38,
    Quote = 39,
    LeftParen = 40,
    RightParen = 41,
    Asterisk = 42,
    Plus = 43,
    Comma = 44,
    Minus = 45,
    Period = 46,
    Slash = 47,
    Num0 = 48,
    Num1 = 49,
    Num2 = 50,
    Num3 = 51,
    Num4 = 52,
    Num5 = 53,
    Num6 = 54,
    Num7 = 55,
    Num8 = 56,
    Num9 = 57,
    Colon = 58,
    Semicolon = 59,
    Less = 60,
    Equals = 61,
    Greater = 62,
    Question = 63,
    At = 64,
     LeftBracket = 91,
     Backslash = 92,
     RightBracket = 93,
     Caret = 94,
     Underscore = 95,
     Backquote = 96,
     A = 97,
     B = 98,
     C = 99,
     D = 100,
     E = 101,
     F = 102,
     G = 103,
     H = 104,
     I = 105,
     J = 106,
     K = 107,
     L = 108,
     M = 109,
     N = 110,
     O = 111,
     P = 112,
     Q = 113,
     R = 114,
     S = 115,
     T = 116,
     U = 117,
     V = 118,
     W = 119,
     X = 120,
     Y = 121,
     Z = 122,
     Delete = 127,
     World0 = 160,
     World1 = 161,
     World2 = 162,
     World3 = 163,
     World4 = 164,
     World5 = 165,
     World6 = 166,
     World7 = 167,
     World8 = 168,
     World9 = 169,
     World10 = 170,
     World11 = 171,
     World12 = 172,
     World13 = 173,
     World14 = 174,
     World15 = 175,
     World16 = 176,
     World17 = 177,
     World18 = 178,
     World19 = 179,
     World20 = 180,
     World21 = 181,
     World22 = 182,
     World23 = 183,
     World24 = 184,
     World25 = 185,
     World26 = 186,
     World27 = 187,
     World28 = 188,
     World29 = 189,
     World30 = 190,
     World31 = 191,
     World32 = 192,
     World33 = 193,
     World34 = 194,
     World35 = 195,
     World36 = 196,
     World37 = 197,
     World38 = 198,
     World39 = 199,
     World40 = 200,
     World41 = 201,
     World42 = 202,
     World43 = 203,
     World44 = 204,
     World45 = 205,
     World46 = 206,
     World47 = 207,
     World48 = 208,
     World49 = 209,
     World50 = 210,
     World51 = 211,
     World52 = 212,
     World53 = 213,
     World54 = 214,
     World55 = 215,
     World56 = 216,
     World57 = 217,
     World58 = 218,
     World59 = 219,
     World60 = 220,
     World61 = 221,
     World62 = 222,
     World63 = 223,
     World64 = 224,
     World65 = 225,
     World66 = 226,
     World67 = 227,
     World68 = 228,
     World69 = 229,
     World70 = 230,
     World71 = 231,
     World72 = 232,
     World73 = 233,
     World74 = 234,
     World75 = 235,
     World76 = 236,
     World77 = 237,
     World78 = 238,
     World79 = 239,
     World80 = 240,
     World81 = 241,
     World82 = 242,
     World83 = 243,
     World84 = 244,
     World85 = 245,
     World86 = 246,
     World87 = 247,
     World88 = 248,
     World89 = 249,
     World90 = 250,
     World91 = 251,
     World92 = 252,
     World93 = 253,
     World94 = 254,
     World95 = 255,
     Kp0 = 256,
     Kp1 = 257,
     Kp2 = 258,
     Kp3 = 259,
     Kp4 = 260,
     Kp5 = 261,
     Kp6 = 262,
     Kp7 = 263,
     Kp8 = 264,
     Kp9 = 265,
     KpPeriod = 266,
     KpDivide = 267,
     KpMultiply = 268,
     KpMinus = 269,
     KpPlus = 270,
     KpEnter = 271,
     KpEquals = 272,
     Up = 273,
     Down = 274,
     Right = 275,
     Left = 276,
     Insert = 277,
     Home = 278,
     End = 279,
     PageUp = 280,
     PageDown = 281,
     F1 = 282,
     F2 = 283,
     F3 = 284,
     F4 = 285,
     F5 = 286,
     F6 = 287,
     F7 = 288,
     F8 = 289,
     F9 = 290,
     F10 = 291,
     F11 = 292,
     F12 = 293,
     F13 = 294,
     F14 = 295,
     F15 = 296,
     NumLock = 300,
     CapsLock = 301,
     ScrolLock = 302,
     RShift = 303,
     LShift = 304,
     RCtrl = 305,
     LCtrl = 306,
     RAlt = 307,
     LAlt = 308,
     RMeta = 309,
     LMeta = 310,
     LSuper = 311,
     RSuper = 312,
     Mode = 313,
     Compose = 314,
     Help = 315,
     Print = 316,
     SysReq = 317,
     Break = 318,
     Menu = 319,
     Power = 320,
     Euro = 321,
     Undo = 322,
     Last
}

impl Copy for Key {}

fn wrap_key(i: ll::SDLKey) -> Option<Key> {
    FromPrimitive::from_uint(i as uint)
}

#[deriving(PartialEq, Eq)]
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

impl Copy for Mod {}

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

#[deriving(PartialEq, Eq)]
pub enum HatState {
    Centered,
    Up,
    Right,
    Down,
    Left
}

impl Copy for HatState {}

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

#[deriving(PartialEq, Eq, FromPrimitive)]
pub enum Mouse {
    Left = 1,
    Middle,
    Right,
    WheelUp,
    WheelDown
}

impl Copy for Mouse {}

fn wrap_mouse(bitflags: u8) -> Option<Mouse> {
    FromPrimitive::from_u8(bitflags)
}

#[deriving(PartialEq, Eq)]
pub enum MouseState {
    Left = 1,
    Middle,
    Right,
    WheelUp,
    WheelDown,
    X1,
    X2
}

impl Copy for MouseState {}

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

#[deriving(PartialEq)]
pub enum Event {
    // TODO: TextInput, TextEditing
     None,
     Active(bool, Vec<AppState>),
     Key(Key, bool, Vec<Mod>, u16), // RFC: do you need the scancode?
     MouseMotion(Vec<MouseState>, u16, u16, i16, i16),
     MouseButton(Mouse, bool, u16, u16),
     JoyAxis(int, int, i16),
     JoyBall(int, int, i16, i16),
     JoyHat(int, int, Vec<HatState>),
     JoyButton(int, int, bool),
     Quit,
    // TODO: SysWm
     Resize(int, int),
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

        let ty : EventType = match FromPrimitive::from_uint(ty as uint) {
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

                Event::JoyAxis(jaxis.which as int, jaxis.axis as int,
                             jaxis.value)
            }
            EventType::JoyBallMotion => {
                let jball = raw.jball();
                let jball = if jball.is_null() { return Event::None; }
                            else { *jball };

                Event::JoyBall(jball.which as int, jball.ball as int,
                             jball.xrel, jball.yrel)
            }
            EventType::JoyHatMotion => {
                let jhat = raw.jhat();
                let jhat = if jhat.is_null() { return Event::None; }
                           else { *jhat };

                Event::JoyHat(jhat.which as int, jhat.hat as int,
                            wrap_hat_state(jhat.value))
            }
            EventType::JoyButtonDown | EventType::JoyButtonUp => {
                let jbutton = raw.jbutton();
                let jbutton = if jbutton.is_null() { return Event::None; }
                              else { *jbutton };

                Event::JoyButton(jbutton.which as int, jbutton.button as int,
                               jbutton.state == 1u8)
            }
            EventType::Quit => Event::Quit,
            EventType::Resize => {
                let resize = raw.resize();
                let resize = if resize.is_null() { return Event::None; }
                             else { *resize };

                Event::Resize(resize.w as int, resize.h as int)
            }
            EventType::Expose => Event::Expose,
            _ => Event::None
        }
    }
}

impl Copy for EventType {}

#[deriving(PartialEq, Eq, FromPrimitive)]
pub enum EventType {
    // TODO: TextInput, TextEditing
     None,
     Active,
     KeyDown,
     KeyUp,
     MouseMotion,
     MouseButtonDown,
     MouseButtonUp,
     JoyAxisMotion,
     JoyBallMotion,
     JoyHatMotion,
     JoyButtonDown,
     JoyButtonUp,
     Quit,
     SysWM,
     Resize = 16,
     Expose,
     User = 24
}

impl EventType {
    pub fn get_state(&self) -> bool { get_event_state(*self) }
    pub fn set_state(&self, state: bool) { set_event_state(*self, state) }
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
    let mut i = -1i;

    let buf = data as *const u8;
    let buf = unsafe { slice::from_raw_buf(&buf, num as uint) };
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

        String::from_raw_buf(mem::transmute_copy(&cstr))
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
