use libc::c_int;

pub enum Event {
    pub KeyDownEvent(*KeyboardEvent_),
    pub KeyUpEvent(*KeyboardEvent_),
    pub QuitEvent,
    pub NoEvent
}

impl Event: cmp::Eq {
    pure fn eq(&self, other: &Event) -> bool {
        match (*self, *other) {
            (QuitEvent, QuitEvent) | (NoEvent, NoEvent) => true,
            (KeyDownEvent(left), KeyDownEvent(right)) |
            (KeyUpEvent(left), KeyUpEvent(right)) => {
                left == right
            },
            _ => false
        }
    }
    pure fn ne(&self, other: &Event) -> bool {
        !self.eq(other)
    }
}

type QuitEvent_ = {
    type_: ll::event::sdl_event_type
};

type KeyboardEvent_ = {
    type_: ll::event::sdl_event_type,
    which: u8,
    state: u8,
    keysym: keyboard::KeySym
};

fn null_event() -> ll::event::Event {
    {
        type_: 0u8,
        event: (0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
                0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
                0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64)
    }
}

fn log_event(e: &ll::event::Event) {
    if e.type_ == ll::event::SDL_NOEVENT { return }
    let name = if e.type_ == ll::event::SDL_NOEVENT { ~"none" }
    else if e.type_ == ll::event::SDL_ACTIVEEVENT { ~"active" }
    else if e.type_ == ll::event::SDL_KEYDOWN { ~"keydown" }
    else if e.type_ == ll::event::SDL_KEYUP { ~"keyup" }
    else if e.type_ == ll::event::SDL_MOUSEMOTION { ~"mousemotion" }
    else if e.type_ == ll::event::SDL_MOUSEBUTTONDOWN { ~"mousebuttondown" }
    else if e.type_ == ll::event::SDL_MOUSEBUTTONUP { ~"mousebuttonup" }
    else if e.type_ == ll::event::SDL_QUIT { ~"quit" }
    else if e.type_ == ll::event::SDL_SYSWMEVENT { ~"syswmevent" }
    else if e.type_ == ll::event::SDL_VIDEORESIZE { ~"videoresize" }
    else if e.type_ == ll::event::SDL_VIDEOEXPOSE { ~"videoexpose" }
    else { ~"other" };
    debug!("event: %s", name);
}

pub fn poll_event(f: fn(Event)) unsafe {
    let raw_event = null_event();
    let result = ll::event::SDL_PollEvent(ptr::addr_of(&raw_event));
    if result as int == 1 {
        let event_ptr = ptr::addr_of(&raw_event.event);
        log_event(&raw_event);
        if (raw_event.type_ == ll::event::SDL_QUIT) {
            f(QuitEvent);
        } else if (raw_event.type_ == ll::event::SDL_KEYDOWN) {
            f(KeyDownEvent(cast::reinterpret_cast(&event_ptr)));
        } else if (raw_event.type_ == ll::event::SDL_KEYUP) {
            f(KeyUpEvent(cast::reinterpret_cast(&event_ptr)));
        } else {
            f(NoEvent);
        }
    } else {
        f(NoEvent);
    }
}
