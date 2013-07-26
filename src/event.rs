use std::cast;
use std::libc::c_int;
use std::str;
use std::vec;

pub mod ll {
    use std::cast;
    use std::libc::{c_void, c_int, c_uint, c_uchar, c_schar, uint8_t, uint16_t, int16_t};
    use std::ptr;

    pub type SDLKey = c_uint;
    pub type SDLMod = c_uint;
    pub type SDL_SysWMmsg = c_void;

    pub static SDL_DISABLE: c_int = 0;
    pub static SDL_ENABLE: c_int = 1;
    pub static SDL_QUERY: c_int = -1;

    pub static SDL_APPMOUSEFOCUS: c_int = 0x01;
    pub static SDL_APPINPUTFOCUS: c_int = 0x02;
    pub static SDL_APPACTIVE: c_int = 0x04;

    pub struct SDL_keysym {
        pub scancode: uint8_t,
        pub sym: SDLKey,
        pub _mod: SDLMod,
        pub unicode: uint16_t,
    }

    pub struct SDL_Event {
        pub data: [c_uchar, ..24],
    }

    pub struct SDL_ActiveEvent {
        pub _type: uint8_t,
        pub gain: uint8_t,
        pub state: uint8_t,
    }

    pub struct SDL_KeyboardEvent {
        pub _type: uint8_t,
        pub which: uint8_t,
        pub state: uint8_t,
        pub keysym: SDL_keysym,
    }

    pub struct SDL_MouseMotionEvent {
        pub _type: uint8_t,
        pub which: uint8_t,
        pub state: uint8_t,
        pub x: uint16_t,
        pub y: uint16_t,
        pub xrel: int16_t,
        pub yrel: int16_t,
    }

    pub struct SDL_MouseButtonEvent {
        pub _type: uint8_t,
        pub which: uint8_t,
        pub button: uint8_t,
        pub state: uint8_t,
        pub x: uint16_t,
        pub y: uint16_t,
    }

    pub struct SDL_JoyAxisEvent {
        pub _type: uint8_t,
        pub which: uint8_t,
        pub axis: uint8_t,
        pub value: int16_t,
    }

    pub struct SDL_JoyBallEvent {
        pub _type: uint8_t,
        pub which: uint8_t,
        pub ball: uint8_t,
        pub xrel: int16_t,
        pub yrel: int16_t,
    }

    pub struct SDL_JoyHatEvent {
        pub _type: uint8_t,
        pub which: uint8_t,
        pub hat: uint8_t,
        pub value: uint8_t,
    }

    pub struct SDL_JoyButtonEvent {
        pub _type: uint8_t,
        pub which: uint8_t,
        pub button: uint8_t,
        pub state: uint8_t,
    }

    pub struct SDL_ResizeEvent {
        pub _type: uint8_t,
        pub w: c_int,
        pub h: c_int,
    }

    pub struct SDL_ExposeEvent {
        pub _type: uint8_t,
    }

    pub struct SDL_QuitEvent {
        pub _type: uint8_t,
    }

    pub struct SDL_UserEvent {
        pub _type: uint8_t,
        pub code: c_int,
        pub data1: *c_void,
        pub data2: *c_void,
    }

    pub struct SDL_SysWMEvent {
        pub _type: uint8_t,
        pub msg: *SDL_SysWMmsg,
    }

    impl SDL_Event {
        pub fn _type(&self) -> *uint8_t {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn active(&self) -> *SDL_ActiveEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn key(&self) -> *SDL_KeyboardEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn motion(&self) -> *SDL_MouseMotionEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn button(&self) -> *SDL_MouseButtonEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn jaxis(&self) -> *SDL_JoyAxisEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn jball(&self) -> *SDL_JoyBallEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn jhat(&self) -> *SDL_JoyHatEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn jbutton(&self) -> *SDL_JoyButtonEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn resize(&self) -> *SDL_ResizeEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn expose(&self) -> *SDL_ExposeEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn quit(&self) -> *SDL_QuitEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn user(&self) -> *SDL_UserEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }

        pub fn syswm(&self) -> *SDL_SysWMEvent {
            unsafe { cast::transmute_copy(&ptr::to_unsafe_ptr(self)) }
        }
    }

    extern {
        pub fn SDL_PollEvent(event: *SDL_Event) -> c_int;
        pub fn SDL_WaitEvent(event: *SDL_Event) -> c_int;
        pub fn SDL_EventState(_type: uint8_t, state: c_int) -> uint8_t;
        pub fn SDL_GetKeyState(numkeys: *c_int) -> *uint8_t;
        pub fn SDL_GetModState() -> SDLMod;
        pub fn SDL_GetKeyName(key: SDLKey) -> *c_schar;
        pub fn SDL_JoystickEventState(state: c_int) -> c_int;
        pub fn SDL_GetAppState() -> uint8_t;
        pub fn SDL_EnableUNICODE(enable: c_int) -> c_int;
        pub fn SDL_EnableKeyRepeat(delay: c_int, interval: c_int) -> c_int;
        pub fn SDL_SetModState(modstate: SDLMod);
        pub fn SDL_PumpEvents();
    }
}

#[deriving(Eq)]
pub enum AppState {
    pub AppMouseFocusState = ll::SDL_APPMOUSEFOCUS as int,
    pub AppInputFocusState = ll::SDL_APPINPUTFOCUS as int,
    pub AppActiveState = ll::SDL_APPACTIVE as int
}

fn wrap_app_state(bitflags: u8) -> ~[AppState] {
    let flags = [AppMouseFocusState,
        AppInputFocusState,
        AppActiveState];

    do flags.iter().filter_map |&flag| {
        if bitflags & (flag as u8) != 0 { Some(flag) }
        else { None }
    }.collect()
}

#[deriving(Eq)]
pub enum RepeatDelay {
    pub DefaultRepeatDelay,
    pub CustomRepeatDelay(int)
}

#[deriving(Eq)]
pub enum RepeatInterval {
    pub DefaultRepeatInterval,
    pub CustomRepeatInterval(int)
}

#[deriving(Eq)]
pub enum Key {
    pub UnknownKey = 0,
    pub BackspaceKey = 8,
    pub TabKey = 9,
    pub ClearKey = 12,
    pub ReturnKey = 13,
    pub PauseKey = 19,
    pub EscapeKey = 27,
    pub SpaceKey = 32,
    pub ExclaimKey = 33,
    pub QuotedblKey = 34,
    pub HashKey = 35,
    pub DollarKey = 36,
    pub AmpersandKey = 38,
    pub QuoteKey = 39,
    pub LeftParenKey = 40,
    pub RightParenKey = 41,
    pub AsteriskKey = 42,
    pub PlusKey = 43,
    pub CommaKey = 44,
    pub MinusKey = 45,
    pub PeriodKey = 46,
    pub SlashKey = 47,
    pub Num0Key = 48,
    pub Num1Key = 49,
    pub Num2Key = 50,
    pub Num3Key = 51,
    pub Num4Key = 52,
    pub Num5Key = 53,
    pub Num6Key = 54,
    pub Num7Key = 55,
    pub Num8Key = 56,
    pub Num9Key = 57,
    pub ColonKey = 58,
    pub SemicolonKey = 59,
    pub LessKey = 60,
    pub EqualsKey = 61,
    pub GreaterKey = 62,
    pub QuestionKey = 63,
    pub AtKey = 64,
    pub LeftBracketKey = 91,
    pub BackslashKey = 92,
    pub RightBracketKey = 93,
    pub CaretKey = 94,
    pub UnderscoreKey = 95,
    pub BackquoteKey = 96,
    pub AKey = 97,
    pub BKey = 98,
    pub CKey = 99,
    pub DKey = 100,
    pub EKey = 101,
    pub FKey = 102,
    pub GKey = 103,
    pub HKey = 104,
    pub IKey = 105,
    pub JKey = 106,
    pub KKey = 107,
    pub LKey = 108,
    pub MKey = 109,
    pub NKey = 110,
    pub OKey = 111,
    pub PKey = 112,
    pub QKey = 113,
    pub RKey = 114,
    pub SKey = 115,
    pub TKey = 116,
    pub UKey = 117,
    pub VKey = 118,
    pub WKey = 119,
    pub XKey = 120,
    pub YKey = 121,
    pub ZKey = 122,
    pub DeleteKey = 127,
    pub World0Key = 160,
    pub World1Key = 161,
    pub World2Key = 162,
    pub World3Key = 163,
    pub World4Key = 164,
    pub World5Key = 165,
    pub World6Key = 166,
    pub World7Key = 167,
    pub World8Key = 168,
    pub World9Key = 169,
    pub World10Key = 170,
    pub World11Key = 171,
    pub World12Key = 172,
    pub World13Key = 173,
    pub World14Key = 174,
    pub World15Key = 175,
    pub World16Key = 176,
    pub World17Key = 177,
    pub World18Key = 178,
    pub World19Key = 179,
    pub World20Key = 180,
    pub World21Key = 181,
    pub World22Key = 182,
    pub World23Key = 183,
    pub World24Key = 184,
    pub World25Key = 185,
    pub World26Key = 186,
    pub World27Key = 187,
    pub World28Key = 188,
    pub World29Key = 189,
    pub World30Key = 190,
    pub World31Key = 191,
    pub World32Key = 192,
    pub World33Key = 193,
    pub World34Key = 194,
    pub World35Key = 195,
    pub World36Key = 196,
    pub World37Key = 197,
    pub World38Key = 198,
    pub World39Key = 199,
    pub World40Key = 200,
    pub World41Key = 201,
    pub World42Key = 202,
    pub World43Key = 203,
    pub World44Key = 204,
    pub World45Key = 205,
    pub World46Key = 206,
    pub World47Key = 207,
    pub World48Key = 208,
    pub World49Key = 209,
    pub World50Key = 210,
    pub World51Key = 211,
    pub World52Key = 212,
    pub World53Key = 213,
    pub World54Key = 214,
    pub World55Key = 215,
    pub World56Key = 216,
    pub World57Key = 217,
    pub World58Key = 218,
    pub World59Key = 219,
    pub World60Key = 220,
    pub World61Key = 221,
    pub World62Key = 222,
    pub World63Key = 223,
    pub World64Key = 224,
    pub World65Key = 225,
    pub World66Key = 226,
    pub World67Key = 227,
    pub World68Key = 228,
    pub World69Key = 229,
    pub World70Key = 230,
    pub World71Key = 231,
    pub World72Key = 232,
    pub World73Key = 233,
    pub World74Key = 234,
    pub World75Key = 235,
    pub World76Key = 236,
    pub World77Key = 237,
    pub World78Key = 238,
    pub World79Key = 239,
    pub World80Key = 240,
    pub World81Key = 241,
    pub World82Key = 242,
    pub World83Key = 243,
    pub World84Key = 244,
    pub World85Key = 245,
    pub World86Key = 246,
    pub World87Key = 247,
    pub World88Key = 248,
    pub World89Key = 249,
    pub World90Key = 250,
    pub World91Key = 251,
    pub World92Key = 252,
    pub World93Key = 253,
    pub World94Key = 254,
    pub World95Key = 255,
    pub Kp0Key = 256,
    pub Kp1Key = 257,
    pub Kp2Key = 258,
    pub Kp3Key = 259,
    pub Kp4Key = 260,
    pub Kp5Key = 261,
    pub Kp6Key = 262,
    pub Kp7Key = 263,
    pub Kp8Key = 264,
    pub Kp9Key = 265,
    pub KpPeriodKey = 266,
    pub KpDivideKey = 267,
    pub KpMultiplyKey = 268,
    pub KpMinusKey = 269,
    pub KpPlusKey = 270,
    pub KpEnterKey = 271,
    pub KpEqualsKey = 272,
    pub UpKey = 273,
    pub DownKey = 274,
    pub RightKey = 275,
    pub LeftKey = 276,
    pub InsertKey = 277,
    pub HomeKey = 278,
    pub EndKey = 279,
    pub PageUpKey = 280,
    pub PageDownKey = 281,
    pub F1Key = 282,
    pub F2Key = 283,
    pub F3Key = 284,
    pub F4Key = 285,
    pub F5Key = 286,
    pub F6Key = 287,
    pub F7Key = 288,
    pub F8Key = 289,
    pub F9Key = 290,
    pub F10Key = 291,
    pub F11Key = 292,
    pub F12Key = 293,
    pub F13Key = 294,
    pub F14Key = 295,
    pub F15Key = 296,
    pub NumLockKey = 300,
    pub CapsLockKey = 301,
    pub ScrolLockKey = 302,
    pub RShiftKey = 303,
    pub LShiftKey = 304,
    pub RCtrlKey = 305,
    pub LCtrlKey = 306,
    pub RAltKey = 307,
    pub LAltKey = 308,
    pub RMetaKey = 309,
    pub LMetaKey = 310,
    pub LSuperKey = 311,
    pub RSuperKey = 312,
    pub ModeKey = 313,
    pub ComposeKey = 314,
    pub HelpKey = 315,
    pub PrintKey = 316,
    pub SysReqKey = 317,
    pub BreakKey = 318,
    pub MenuKey = 319,
    pub PowerKey = 320,
    pub EuroKey = 321,
    pub UndoKey = 322,
    pub LastKey
}

fn wrap_key(i: ll::SDLKey) -> Option<Key> {
    unsafe {
        // Handle edge cases where there's no variant (they're ignored)
        if (i < 8) || (i > 9 && i < 12) || (i > 13 && i < 19) || (i > 19 && i < 27) ||
           (i > 27 && i < 32) || (i > 36 && i < 38) || (i > 64 && i < 91) || (i > 122 && i < 127) ||
           (i > 127 && i < 160) || (i > 296 && i < 300) { None }
        else { Some(cast::transmute(i as uint)) }
    }
}

#[deriving(Eq)]
pub enum Mod {
    pub NoMod = 0x0000,
    pub LShiftMod = 0x0001,
    pub RShiftMod = 0x0002,
    pub LCtrlMod = 0x0040,
    pub RCtrlMod = 0x0080,
    pub LAltMod = 0x0100,
    pub RAltMod = 0x0200,
    pub LMetaMod = 0x0400,
    pub RMetaMod = 0x0800,
    pub NumMod = 0x1000,
    pub CapsMod = 0x2000,
    pub ModeMod = 0x4000,
    pub ReservedMod = 0x8000
}

fn wrap_mod_state(bitflags: ll::SDLMod) -> ~[Mod] {
    let flags = [NoMod,
        LShiftMod,
        RShiftMod,
        LCtrlMod,
        RCtrlMod,
        LAltMod,
        RAltMod,
        LMetaMod,
        RMetaMod,
        NumMod,
        CapsMod,
        ModeMod,
        ReservedMod];

    do flags.iter().filter_map |&flag| {
        if bitflags & (flag as ll::SDLMod) != 0 { Some(flag) }
        else { None }
    }.collect()
}

#[deriving(Eq)]
pub enum HatState {
    CenteredHatState,
    UpHatState,
    RightHatState,
    DownHatState,
    LeftHatState
}

fn wrap_hat_state(bitflags: u8) -> ~[HatState] {
    let flags = [CenteredHatState,
        UpHatState,
        RightHatState,
        DownHatState,
        LeftHatState];

    do flags.iter().filter_map |&flag| {
        if bitflags & (flag as u8) != 0 { Some(flag) }
        else { None }
    }.collect()
}

#[deriving(Eq)]
pub enum Mouse {
    LeftMouse,
    MiddleMouse,
    RightMouse,
    WheelUpMouse,
    WheelDownMouse
}

fn wrap_mouse(bitflags: u8) -> Mouse {
    match bitflags {
        1 => LeftMouse,
        2 => MiddleMouse,
        3 => RightMouse,
        4 => WheelUpMouse,
        5 => WheelDownMouse,
        _ => fail!(~"unhandled mouse type")
    }
}

#[deriving(Eq)]
pub enum MouseState {
    LeftMouseState = 1,
    MiddleMouseState,
    RightMouseState,
    WheelUpMouseState,
    WheelDownMouseState,
    X1MouseState,
    X2MouseState
}

fn wrap_mouse_state(bitflags: u8) -> ~[MouseState] {
    let flags = [LeftMouseState,
        MiddleMouseState,
        RightMouseState,
        WheelUpMouseState,
        WheelDownMouseState,
        X1MouseState,
        X2MouseState];

    do flags.iter().filter_map |&flag| {
        if bitflags & (flag as u8) != 0 { Some(flag) }
        else { None }
    }.collect()
}

#[deriving(Eq)]
pub enum Event {
    // TODO: TextInputEvent, TextEditingEvent
    pub NoEvent,
    pub ActiveEvent(bool, ~[AppState]),
    pub KeyEvent(Key, bool, ~[Mod], u16), // RFC: do you need the scancode?
    pub MouseMotionEvent(~[MouseState], u16, u16, i16, i16),
    pub MouseButtonEvent(Mouse, bool, u16, u16),
    pub JoyAxisEvent(int, int, i16),
    pub JoyBallEvent(int, int, i16, i16),
    pub JoyHatEvent(int, int, ~[HatState]),
    pub JoyButtonEvent(int, int, bool),
    pub QuitEvent,
    // TODO: SysWmEvent
    pub ResizeEvent(int, int),
    pub ExposeEvent,
    // TODO: UserEvent
}

fn null_event() -> ll::SDL_Event {
    ll::SDL_Event { data: [0, .. 24] }
}

fn wrap_event(raw: ll::SDL_Event) -> Event {
    unsafe {
        let ty = raw._type();
        let ty = if ty.is_null() { return NoEvent; }
                 else { *ty };

        if ty < 0 || (ty > 13 && ty < 16) || (ty > 17 && ty < 24) || ty > 24 {
            return NoEvent;
        }

        // FIXME: This is incredibly hacky and will probably break on 32-bit
        let ty: EventType = cast::transmute(ty as uint);

        match ty {
            NoEventType => NoEvent,
            ActiveEventType => {
                let active = raw.active();
                let active = if active.is_null() { return NoEvent; }
                             else { *active };

                ActiveEvent(active.gain == 1, wrap_app_state(active.state))
            }
            KeyDownEventType | KeyUpEventType => {
                let key = raw.key();
                let (key, okey) = if key.is_null() { return NoEvent; }
                          else { ((*key).keysym, *key) };

                match wrap_key(key.sym) {
                    Some(sym) => {
                        KeyEvent(sym, okey.state == 1, wrap_mod_state(key._mod),
                                 key.unicode)
                    }
                    None => NoEvent
                }
            }
            MouseMotionEventType => {
                let motion = raw.motion();
                let motion = if motion.is_null() { return NoEvent; }
                             else { *motion };

                MouseMotionEvent(wrap_mouse_state(motion.state), motion.x,
                                 motion.y, motion.xrel, motion.yrel)
            }
            MouseButtonDownEventType | MouseButtonUpEventType => {
                let button = raw.button();
                let button = if button.is_null() { return NoEvent; }
                             else { *button };

                MouseButtonEvent(wrap_mouse(button.button), button.state == 1,
                                 button.x, button.y)
            }
            JoyAxisMotionEventType => {
                let jaxis = raw.jaxis();
                let jaxis = if jaxis.is_null() { return NoEvent; }
                            else { *jaxis };

                JoyAxisEvent(jaxis.which as int, jaxis.axis as int,
                             jaxis.value)
            }
            JoyBallMotionEventType => {
                let jball = raw.jball();
                let jball = if jball.is_null() { return NoEvent; }
                            else { *jball };

                JoyBallEvent(jball.which as int, jball.ball as int,
                             jball.xrel, jball.yrel)
            }
            JoyHatMotionEventType => {
                let jhat = raw.jhat();
                let jhat = if jhat.is_null() { return NoEvent; }
                           else { *jhat };

                JoyHatEvent(jhat.which as int, jhat.hat as int,
                            wrap_hat_state(jhat.value))
            }
            JoyButtonDownEventType | JoyButtonUpEventType => {
                let jbutton = raw.jbutton();
                let jbutton = if jbutton.is_null() { return NoEvent; }
                              else { *jbutton };

                JoyButtonEvent(jbutton.which as int, jbutton.button as int,
                               jbutton.state == 1u8)
            }
            QuitEventType => QuitEvent,
            ResizeEventType => {
                let resize = raw.resize();
                let resize = if resize.is_null() { return NoEvent; }
                             else { *resize };

                ResizeEvent(resize.w as int, resize.h as int)
            }
            ExposeEventType => ExposeEvent,
            _ => NoEvent
        }
    }
}

#[deriving(Eq)]
pub enum EventType {
    // TODO: TextInputEventType, TextEditingEventType
    pub NoEventType,
    pub ActiveEventType,
    pub KeyDownEventType,
    pub KeyUpEventType,
    pub MouseMotionEventType,
    pub MouseButtonDownEventType,
    pub MouseButtonUpEventType,
    pub JoyAxisMotionEventType,
    pub JoyBallMotionEventType,
    pub JoyHatMotionEventType,
    pub JoyButtonDownEventType,
    pub JoyButtonUpEventType,
    pub QuitEventType,
    pub SysWMEventType,
    pub ResizeEventType = 16,
    pub ExposeEventType,
    pub UserEventType = 24
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
    let raw = null_event();
    let success = unsafe { ll::SDL_WaitEvent(&raw)
                            == 1 as c_int };

    if success { wrap_event(raw) }
    else { NoEvent }
}

pub fn poll_event() -> Event {
    pump_events();

    let raw = null_event();
    let have = unsafe { ll::SDL_PollEvent(&raw) };

    if have != 1 {
        return NoEvent;
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

pub fn get_key_state() -> ~[(Key, bool)] {
    let num = 0;
    let data = unsafe { ll::SDL_GetKeyState(&num) };
    let mut i = -1;

    unsafe {
        let buf = vec::raw::from_buf_raw(data, num as uint);
        do buf.iter().filter_map |&state| {
            i += 1;

            match wrap_key(i as ll::SDLKey) {
                Some(key) => Some((key, state == 1)),
                None => None
            }
        }.collect()
    }
}

pub fn get_mod_state() -> ~[Mod] {
    unsafe { wrap_mod_state(ll::SDL_GetModState()) }
}

pub fn set_mod_state(states: &[Mod]) {
    unsafe {
        ll::SDL_SetModState(do states.iter().fold(0u32) |states, &state| {
            states | state as ll::SDLMod
        });
    }
}

pub fn get_key_name(key: Key) -> ~str {
    unsafe {
        let cstr = ll::SDL_GetKeyName(key as ll::SDLKey);

        str::raw::from_c_str(cast::transmute_copy(&cstr))
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

pub fn get_app_state() -> ~[AppState] {
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
        DefaultRepeatDelay => 500,
        CustomRepeatDelay(delay) => delay
    };
    let interval = match interval {
        DefaultRepeatInterval => 30,
        CustomRepeatInterval(interval) => interval
    };

    unsafe {
        ll::SDL_EnableKeyRepeat(delay as c_int, interval as c_int) == 0 as c_int
    }
}

// get_mouse_state, get_relative_mouse_state, start_text_input, stop_text_input, set_text_input_rect
