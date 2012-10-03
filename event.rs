use libc::c_int;

type EventType = u8;

const noevent: u8 = 0u8;
const activeevent: u8 = 1u8;
const keydown: u8 = 2u8;
const keyup: u8 = 3u8;
const mousemotion: u8 = 4u8;
const mousebuttondown: u8 = 5u8;
const mousebuttonup: u8 = 6u8;
const joyaxismotion: u8 = 7u8;
const joyballmotion: u8 = 8u8;
const joyhatmotion: u8 = 9u8;
const joybuttondown: u8 = 10u8;
const joybuttonup: u8 = 11u8;
const quit: u8 = 12u8;
const syswmevent: u8 = 13u8;
const event_reserveda: u8 = 14u8;
const event_reservedb: u8 = 15u8;
const videoresize: u8 = 16u8;
const videoexpose: u8 = 17u8;
const event_reserved2: u8 = 18u8;
const event_reserved3: u8 = 19u8;
const event_reserved4: u8 = 20u8;
const event_reserved5: u8 = 21u8;
const event_reserved6: u8 = 22u8;
const event_reserved7: u8 = 23u8;
const event_userevent: u8 = 24u8;

type RawEvent = {
    type_: EventType,
    // FIXME: Not sure exactly how big this needs to be
    event: (u64, u64, u64, u64, u64, u64, u64, u64,
            u64, u64, u64, u64, u64, u64, u64, u64,
            u64, u64, u64, u64, u64, u64, u64, u64)
};

pub enum Event {
    pub KeyDownEvent(*KeyboardEvent_),
    pub KeyUpEvent(*KeyboardEvent_),
    pub QuitEvent,
    pub NoEvent
}

impl Event: cmp::Eq {
    pure fn eq(other: &Event) -> bool {
        match (self, *other) {
            (QuitEvent, QuitEvent) | (NoEvent, NoEvent) => true,
            (KeyDownEvent(left), KeyDownEvent(right)) |
            (KeyUpEvent(left), KeyUpEvent(right)) => {
                left == right
            },
            _ => false
        }
    }
    pure fn ne(other: &Event) -> bool {
        !self.eq(other)
    }
}


type QuitEvent_ = {
    type_: EventType 
};

const SDL_RELEASED: u8 = 0u8;
const SDL_PRESSED: u8 = 1u8;

type KeyboardEvent_ = {
    type_: EventType,
    which: u8,
    state: u8,
    keysym: keyboard::KeySym
};

fn null_event() -> RawEvent {
    {
        type_: 0u8,
        event: (0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
                0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
                0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64)
    }
}

pub fn log_event(e: &RawEvent) {
    if e.type_ == noevent { return }
    let name = if e.type_ == noevent { ~"none" }
    else if e.type_ == activeevent { ~"active" }
    else if e.type_ == keydown { ~"keydown" }
    else if e.type_ == keyup { ~"keyup" }
    else if e.type_ == mousemotion { ~"mousemotion" }
    else if e.type_ == mousebuttondown { ~"mousebuttondown" }
    else if e.type_ == mousebuttonup { ~"mousebuttonup" }
    else if e.type_ == quit { ~"quit" }
    else if e.type_ == syswmevent { ~"syswmevent" }
    else if e.type_ == videoresize { ~"videoresize" }
    else if e.type_ == videoexpose { ~"videoexpose" }
    else { ~"other" };
    #debug("event: %s", name);
}

pub fn poll_event(f: fn(Event)) unsafe {
    let raw_event = null_event();
    let result = SDL::SDL_PollEvent(ptr::addr_of(raw_event));
    if result as int == 1 {
        let event_ptr = ptr::addr_of(&raw_event.event);
        log_event(&raw_event);
        if (raw_event.type_ == quit) {
            f(QuitEvent);
        } else if (raw_event.type_ == keydown) {
            f(KeyDownEvent(cast::reinterpret_cast(&event_ptr)));
        } else if (raw_event.type_ == keyup) {
            f(KeyUpEvent(cast::reinterpret_cast(&event_ptr)));
        } else {
            f(NoEvent);
        }
    } else {
        f(NoEvent);
    }
}

extern mod SDL {
    fn SDL_PollEvent(event: *RawEvent) -> c_int;
}
