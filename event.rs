use conversions::*;
use keyboard;

use core::libc::c_int;

pub enum Event {
    pub KeyDownEvent(KeyboardEvent),
    pub KeyUpEvent(KeyboardEvent),
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

pub struct KeyboardEvent {
    window_id: u8,
    state: u8,
    keycode: keyboard::Key,
    modifier: keyboard::CombinedModifier 
}

impl KeyboardEvent: cmp::Eq {
    pure fn eq(&self, other: &KeyboardEvent) -> bool {
        (self.window_id == other.window_id &&
        self.state == other.state &&
        self.keycode == other.keycode &&
        self.modifier == other.modifier)
    }

    pure fn ne(&self, other: &KeyboardEvent) -> bool {
        !self.eq(other)
    }
}

fn null_event() -> ll::event::SDL_Event {
    {
        type_: 0u8,
        event: (0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
                0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
                0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64)
    }
}

fn log_event(e: &ll::event::SDL_Event) {
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
        let event_ptr = ptr::addr_of(&raw_event);
        log_event(&raw_event);
        if (raw_event.type_ == ll::event::SDL_QUIT) {
            f(QuitEvent);
        } else if (raw_event.type_ == ll::event::SDL_KEYDOWN) {
            let raw_keyboard_event: *ll::event::SDL_KeyboardEvent = cast::reinterpret_cast(&event_ptr);
            f(KeyDownEvent((*raw_keyboard_event).to_hl()));
        } else if (raw_event.type_ == ll::event::SDL_KEYUP) {
            let raw_keyboard_event: *ll::event::SDL_KeyboardEvent = cast::reinterpret_cast(&event_ptr);
            f(KeyUpEvent((*raw_keyboard_event).to_hl()));
        } else {
            f(NoEvent);
        }
    } else {
        f(NoEvent);
    }
}
