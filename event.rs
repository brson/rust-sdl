import libc::c_int;

export poll_event;

export event, quit_event, no_event;

type event_type = u8;

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

type raw_event = {
    type_: event_type,
    // FIXME: Not sure exactly how big this needs to be
    event: (u64, u64, u64, u64, u64, u64, u64, u64,
            u64, u64, u64, u64, u64, u64, u64, u64,
            u64, u64, u64, u64, u64, u64, u64, u64)
};

enum event {
    keydown_event(*keyboard_event_),
    keyup_event(*keyboard_event_),
    quit_event,
    no_event
}

type quit_event_ = {
    type_: event_type
};

const SDL_RELEASED: u8 = 0u8;
const SDL_PRESSED: u8 = 1u8;

type keyboard_event_ = {
    type_: event_type,
    which: u8,
    state: u8,
    keysym: keyboard::keysym
};

fn null_event() -> raw_event {
    {
        type_: 0u8,
        event: (0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
                0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
                0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64)
    }
}

fn log_event(e: raw_event) {
    if e.type_ == noevent { ret }
    let name = if e.type_ == noevent { "none" }
    else if e.type_ == activeevent { "active" }
    else if e.type_ == keydown { "keydown" }
    else if e.type_ == keyup { "keyup" }
    else if e.type_ == mousemotion { "mousemotion" }
    else if e.type_ == mousebuttondown { "mousebuttondown" }
    else if e.type_ == mousebuttonup { "mousebuttonup" }
    else if e.type_ == quit { "quit" }
    else if e.type_ == syswmevent { "syswmevent" }
    else if e.type_ == videoresize { "videoresize" }
    else if e.type_ == videoexpose { "videoexpose" }
    else { "other" };
    #debug("event: %s", name);
}

fn poll_event(f: fn(event)) unsafe {
    let raw_event = null_event();
    let result = SDL::SDL_PollEvent(ptr::addr_of(raw_event));
    if result as int == 1 {
        let event_ptr = ptr::addr_of(raw_event.event);
        log_event(raw_event);
        if (raw_event.type_ == quit) {
            f(quit_event);
        } else if (raw_event.type_ == keydown) {
            f(keydown_event(unsafe::reinterpret_cast(event_ptr)));
        } else if (raw_event.type_ == keyup) {
            f(keyup_event(unsafe::reinterpret_cast(event_ptr)));
        } else {
            f(no_event);
        }
    } else {
        f(no_event);
    }
}

native mod SDL {
    fn SDL_PollEvent(event: *raw_event) -> c_int;
}
