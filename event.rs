export poll_event;

export event, quit, quit_event;

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

type event_ = {
    type_: event_type,
    // FIXME: Not sure exactly how big this needs to be
    event: (u64, u64, u64, u64, u64, u64, u64, u64)
};

enum event {
    quit_event(*quit_event_)
}

type quit_event_ = {
    type_: event_type
};

fn null_event() -> event_ {
    {
        type_: 0u8,
        event: (0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64)
    }
}

fn poll_event(f: fn(option<event>)) unsafe {
    let event = null_event();
    let result = SDL::SDL_PollEvent(ptr::addr_of(event));
    if result as int == 0 {
        let event_ptr = ptr::addr_of(event.event);
        if (event.type_ == quit) {
            f(some(quit_event(unsafe::reinterpret_cast(event_ptr))));
        } else {
            f(none);
        }
    } else {
        f(none);
    }
}

native mod SDL {
    fn SDL_PollEvent(event: *event_) -> c::c_int;
}