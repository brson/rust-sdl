use std::cast;
use std::libc::c_int;
use std::str;
use std::slice;
use std::num::FromPrimitive;

pub mod ll {
    #[allow(non_camel_case_types)];

    use std::cast;
    use std::libc::{c_void, c_int, c_uint, c_uchar, c_schar, uint8_t, uint16_t, int16_t};

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
        scancode: uint8_t,
        sym: SDLKey,
        _mod: SDLMod,
        unicode: uint16_t,
    }

    pub struct SDL_Event {
        data: [c_uchar, ..24],
    }

    pub struct SDL_ActiveEvent {
        _type: uint8_t,
        gain: uint8_t,
        state: uint8_t,
    }

    pub struct SDL_KeyboardEvent {
        _type: uint8_t,
        which: uint8_t,
        state: uint8_t,
        keysym: SDL_keysym,
    }

    pub struct SDL_MouseMotionEvent {
        _type: uint8_t,
        which: uint8_t,
        state: uint8_t,
        x: uint16_t,
        y: uint16_t,
        xrel: int16_t,
        yrel: int16_t,
    }

    pub struct SDL_MouseButtonEvent {
        _type: uint8_t,
        which: uint8_t,
        button: uint8_t,
        state: uint8_t,
        x: uint16_t,
        y: uint16_t,
    }

    pub struct SDL_JoyAxisEvent {
        _type: uint8_t,
        which: uint8_t,
        axis: uint8_t,
        value: int16_t,
    }

    pub struct SDL_JoyBallEvent {
        _type: uint8_t,
        which: uint8_t,
        ball: uint8_t,
        xrel: int16_t,
        yrel: int16_t,
    }

    pub struct SDL_JoyHatEvent {
        _type: uint8_t,
        which: uint8_t,
        hat: uint8_t,
        value: uint8_t,
    }

    pub struct SDL_JoyButtonEvent {
        _type: uint8_t,
        which: uint8_t,
        button: uint8_t,
        state: uint8_t,
    }

    pub struct SDL_ResizeEvent {
        _type: uint8_t,
        w: c_int,
        h: c_int,
    }

    pub struct SDL_ExposeEvent {
        _type: uint8_t,
    }

    pub struct SDL_QuitEvent {
        _type: uint8_t,
    }

    pub struct SDL_UserEvent {
        _type: uint8_t,
        code: c_int,
        data1: *c_void,
        data2: *c_void,
    }

    pub struct SDL_SysWMEvent {
        _type: uint8_t,
        msg: *SDL_SysWMmsg,
    }

    impl SDL_Event {
        pub fn _type(&self) -> *uint8_t {
            unsafe { cast::transmute_copy(&self) }
        }

        pub fn active(&self) -> *SDL_ActiveEvent {
            unsafe { cast::transmute_copy(&self) }
        }

        pub fn key(&self) -> *SDL_KeyboardEvent {
            unsafe { cast::transmute_copy(&self) }
        }

        pub fn motion(&self) -> *SDL_MouseMotionEvent {
            unsafe { cast::transmute_copy(&self) }
        }

        pub fn button(&self) -> *SDL_MouseButtonEvent {
            unsafe { cast::transmute_copy(&self) }
        }

        pub fn jaxis(&self) -> *SDL_JoyAxisEvent {
            unsafe { cast::transmute_copy(&self) }
        }

        pub fn jball(&self) -> *SDL_JoyBallEvent {
            unsafe { cast::transmute_copy(&self) }
        }

        pub fn jhat(&self) -> *SDL_JoyHatEvent {
            unsafe { cast::transmute_copy(&self) }
        }

        pub fn jbutton(&self) -> *SDL_JoyButtonEvent {
            unsafe { cast::transmute_copy(&self) }
        }

        pub fn resize(&self) -> *SDL_ResizeEvent {
            unsafe { cast::transmute_copy(&self) }
        }

        pub fn expose(&self) -> *SDL_ExposeEvent {
            unsafe { cast::transmute_copy(&self) }
        }

        pub fn quit(&self) -> *SDL_QuitEvent {
            unsafe { cast::transmute_copy(&self) }
        }

        pub fn user(&self) -> *SDL_UserEvent {
            unsafe { cast::transmute_copy(&self) }
        }

        pub fn syswm(&self) -> *SDL_SysWMEvent {
            unsafe { cast::transmute_copy(&self) }
        }
    }

    extern "C" {
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
    AppMouseFocusState = ll::SDL_APPMOUSEFOCUS as int,
    AppInputFocusState = ll::SDL_APPINPUTFOCUS as int,
    AppActiveState = ll::SDL_APPACTIVE as int
}

fn wrap_app_state(bitflags: u8) -> ~[AppState] {
    let flags = [AppMouseFocusState,
        AppInputFocusState,
        AppActiveState];

    flags.iter().filter_map(|&flag| {
        if bitflags & (flag as u8) != 0 { Some(flag) }
        else { None }
    }).collect()
}

#[deriving(Eq)]
pub enum RepeatDelay {
    DefaultRepeatDelay,
    CustomRepeatDelay(int)
}

#[deriving(Eq)]
pub enum RepeatInterval {
    DefaultRepeatInterval,
    CustomRepeatInterval(int)
}

#[deriving(Eq, FromPrimitive)]
pub enum Key {
    UnknownKey = 0,
    BackspaceKey = 8,
    TabKey = 9,
    ClearKey = 12,
    ReturnKey = 13,
    PauseKey = 19,
    EscapeKey = 27,
    SpaceKey = 32,
    ExclaimKey = 33,
    QuotedblKey = 34,
    HashKey = 35,
    DollarKey = 36,
    AmpersandKey = 38,
    QuoteKey = 39,
    LeftParenKey = 40,
    RightParenKey = 41,
    AsteriskKey = 42,
    PlusKey = 43,
    CommaKey = 44,
    MinusKey = 45,
    PeriodKey = 46,
    SlashKey = 47,
    Num0Key = 48,
    Num1Key = 49,
    Num2Key = 50,
    Num3Key = 51,
    Num4Key = 52,
    Num5Key = 53,
    Num6Key = 54,
    Num7Key = 55,
    Num8Key = 56,
    Num9Key = 57,
    ColonKey = 58,
    SemicolonKey = 59,
    LessKey = 60,
    EqualsKey = 61,
    GreaterKey = 62,
    QuestionKey = 63,
    AtKey = 64,
     LeftBracketKey = 91,
     BackslashKey = 92,
     RightBracketKey = 93,
     CaretKey = 94,
     UnderscoreKey = 95,
     BackquoteKey = 96,
     AKey = 97,
     BKey = 98,
     CKey = 99,
     DKey = 100,
     EKey = 101,
     FKey = 102,
     GKey = 103,
     HKey = 104,
     IKey = 105,
     JKey = 106,
     KKey = 107,
     LKey = 108,
     MKey = 109,
     NKey = 110,
     OKey = 111,
     PKey = 112,
     QKey = 113,
     RKey = 114,
     SKey = 115,
     TKey = 116,
     UKey = 117,
     VKey = 118,
     WKey = 119,
     XKey = 120,
     YKey = 121,
     ZKey = 122,
     DeleteKey = 127,
     World0Key = 160,
     World1Key = 161,
     World2Key = 162,
     World3Key = 163,
     World4Key = 164,
     World5Key = 165,
     World6Key = 166,
     World7Key = 167,
     World8Key = 168,
     World9Key = 169,
     World10Key = 170,
     World11Key = 171,
     World12Key = 172,
     World13Key = 173,
     World14Key = 174,
     World15Key = 175,
     World16Key = 176,
     World17Key = 177,
     World18Key = 178,
     World19Key = 179,
     World20Key = 180,
     World21Key = 181,
     World22Key = 182,
     World23Key = 183,
     World24Key = 184,
     World25Key = 185,
     World26Key = 186,
     World27Key = 187,
     World28Key = 188,
     World29Key = 189,
     World30Key = 190,
     World31Key = 191,
     World32Key = 192,
     World33Key = 193,
     World34Key = 194,
     World35Key = 195,
     World36Key = 196,
     World37Key = 197,
     World38Key = 198,
     World39Key = 199,
     World40Key = 200,
     World41Key = 201,
     World42Key = 202,
     World43Key = 203,
     World44Key = 204,
     World45Key = 205,
     World46Key = 206,
     World47Key = 207,
     World48Key = 208,
     World49Key = 209,
     World50Key = 210,
     World51Key = 211,
     World52Key = 212,
     World53Key = 213,
     World54Key = 214,
     World55Key = 215,
     World56Key = 216,
     World57Key = 217,
     World58Key = 218,
     World59Key = 219,
     World60Key = 220,
     World61Key = 221,
     World62Key = 222,
     World63Key = 223,
     World64Key = 224,
     World65Key = 225,
     World66Key = 226,
     World67Key = 227,
     World68Key = 228,
     World69Key = 229,
     World70Key = 230,
     World71Key = 231,
     World72Key = 232,
     World73Key = 233,
     World74Key = 234,
     World75Key = 235,
     World76Key = 236,
     World77Key = 237,
     World78Key = 238,
     World79Key = 239,
     World80Key = 240,
     World81Key = 241,
     World82Key = 242,
     World83Key = 243,
     World84Key = 244,
     World85Key = 245,
     World86Key = 246,
     World87Key = 247,
     World88Key = 248,
     World89Key = 249,
     World90Key = 250,
     World91Key = 251,
     World92Key = 252,
     World93Key = 253,
     World94Key = 254,
     World95Key = 255,
     Kp0Key = 256,
     Kp1Key = 257,
     Kp2Key = 258,
     Kp3Key = 259,
     Kp4Key = 260,
     Kp5Key = 261,
     Kp6Key = 262,
     Kp7Key = 263,
     Kp8Key = 264,
     Kp9Key = 265,
     KpPeriodKey = 266,
     KpDivideKey = 267,
     KpMultiplyKey = 268,
     KpMinusKey = 269,
     KpPlusKey = 270,
     KpEnterKey = 271,
     KpEqualsKey = 272,
     UpKey = 273,
     DownKey = 274,
     RightKey = 275,
     LeftKey = 276,
     InsertKey = 277,
     HomeKey = 278,
     EndKey = 279,
     PageUpKey = 280,
     PageDownKey = 281,
     F1Key = 282,
     F2Key = 283,
     F3Key = 284,
     F4Key = 285,
     F5Key = 286,
     F6Key = 287,
     F7Key = 288,
     F8Key = 289,
     F9Key = 290,
     F10Key = 291,
     F11Key = 292,
     F12Key = 293,
     F13Key = 294,
     F14Key = 295,
     F15Key = 296,
     NumLockKey = 300,
     CapsLockKey = 301,
     ScrolLockKey = 302,
     RShiftKey = 303,
     LShiftKey = 304,
     RCtrlKey = 305,
     LCtrlKey = 306,
     RAltKey = 307,
     LAltKey = 308,
     RMetaKey = 309,
     LMetaKey = 310,
     LSuperKey = 311,
     RSuperKey = 312,
     ModeKey = 313,
     ComposeKey = 314,
     HelpKey = 315,
     PrintKey = 316,
     SysReqKey = 317,
     BreakKey = 318,
     MenuKey = 319,
     PowerKey = 320,
     EuroKey = 321,
     UndoKey = 322,
     LastKey
}

fn wrap_key(i: ll::SDLKey) -> Option<Key> {
    FromPrimitive::from_uint(i as uint)
}

#[deriving(Eq)]
pub enum Mod {
     NoMod = 0x0000,
     LShiftMod = 0x0001,
     RShiftMod = 0x0002,
     LCtrlMod = 0x0040,
     RCtrlMod = 0x0080,
     LAltMod = 0x0100,
     RAltMod = 0x0200,
     LMetaMod = 0x0400,
     RMetaMod = 0x0800,
     NumMod = 0x1000,
     CapsMod = 0x2000,
     ModeMod = 0x4000,
     ReservedMod = 0x8000
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

    flags.iter().filter_map(|&flag| {
        if bitflags & (flag as ll::SDLMod) != 0 { Some(flag) }
        else { None }
    }).collect()
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

    flags.iter().filter_map(|&flag| {
        if bitflags & (flag as u8) != 0 { Some(flag) }
        else { None }
    }).collect()
}

#[deriving(Eq, FromPrimitive)]
pub enum Mouse {
    LeftMouse = 1,
    MiddleMouse,
    RightMouse,
    WheelUpMouse,
    WheelDownMouse
}

fn wrap_mouse(bitflags: u8) -> Option<Mouse> {
    FromPrimitive::from_u8(bitflags)
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

    flags.iter().filter_map(|&flag| {
        if bitflags & (flag as u8) != 0 { Some(flag) }
        else { None }
    }).collect()
}

#[deriving(Eq)]
pub enum Event {
    // TODO: TextInputEvent, TextEditingEvent
     NoEvent,
     ActiveEvent(bool, ~[AppState]),
     KeyEvent(Key, bool, ~[Mod], u16), // RFC: do you need the scancode?
     MouseMotionEvent(~[MouseState], u16, u16, i16, i16),
     MouseButtonEvent(Mouse, bool, u16, u16),
     JoyAxisEvent(int, int, i16),
     JoyBallEvent(int, int, i16, i16),
     JoyHatEvent(int, int, ~[HatState]),
     JoyButtonEvent(int, int, bool),
     QuitEvent,
    // TODO: SysWmEvent
     ResizeEvent(int, int),
     ExposeEvent,
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

        let ty : EventType = match FromPrimitive::from_uint(ty as uint) {
            Some(ty) => ty,
            None => return NoEvent
        };

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
                let obutton = raw.button();
                let obutton = if obutton.is_null() { return NoEvent; }
                             else { *obutton };

                match wrap_mouse(obutton.button) {
                    Some(button) => {
                        MouseButtonEvent(button, obutton.state == 1,
                                 obutton.x, obutton.y)
                    }
                    None => NoEvent
                }
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

#[deriving(Eq, FromPrimitive)]
pub enum EventType {
    // TODO: TextInputEventType, TextEditingEventType
     NoEventType,
     ActiveEventType,
     KeyDownEventType,
     KeyUpEventType,
     MouseMotionEventType,
     MouseButtonDownEventType,
     MouseButtonUpEventType,
     JoyAxisMotionEventType,
     JoyBallMotionEventType,
     JoyHatMotionEventType,
     JoyButtonDownEventType,
     JoyButtonUpEventType,
     QuitEventType,
     SysWMEventType,
     ResizeEventType = 16,
     ExposeEventType,
     UserEventType = 24
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
        let buf = slice::raw::from_buf_raw(data, num as uint);
        buf.iter().filter_map(|&state| {
            i += 1;

            match wrap_key(i as ll::SDLKey) {
                Some(key) => Some((key, state == 1)),
                None => None
            }
        }).collect()
    }
}

pub fn get_mod_state() -> ~[Mod] {
    unsafe { wrap_mod_state(ll::SDL_GetModState()) }
}

pub fn set_mod_state(states: &[Mod]) {
    unsafe {
        ll::SDL_SetModState(states.iter().fold(0u32, |states, &state| {
            states | state as ll::SDLMod
        }));
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
