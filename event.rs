use conversions::*;
use keyboard;
use ll::event::*;

use core::cast::transmute;
use core::libc::c_int;

#[deriving_eq]
pub enum Event {
    pub KeyDownEvent(KeyboardEvent),
    pub KeyUpEvent(KeyboardEvent),
    pub QuitEvent,
    pub NoEvent
}

#[deriving_eq]
pub struct KeyboardEvent {
    window_id: u8,
    state: u8,
    keycode: keyboard::Key,
    modifier: keyboard::CombinedModifier 
}

fn null_event() -> ll::event::SDL_Event {
    ll::event::SDL_Event {
        type_: 0,
        event: [ 0, ..24 ]
    }
}

fn log_event(e: &ll::event::SDL_Event) {
    let name = match e.type_ {
        SDL_NOEVENT => return,
        SDL_ACTIVEEVENT => "active",
        SDL_KEYDOWN => "keydown",
        SDL_KEYUP => "keyup",
        SDL_MOUSEMOTION => "mousemotion",
        SDL_MOUSEBUTTONDOWN => "mousebuttondown",
        SDL_MOUSEBUTTONUP => "mousebuttonup",
        SDL_QUIT => "quit",
        SDL_SYSWMEVENT => "syswmevent",
        SDL_VIDEORESIZE => "videoresize",
        SDL_VIDEOEXPOSE => "videoexpose",
        _ => "other",
    };
    debug!("event: %s", name);
}

pub fn poll_event() -> Event {
    unsafe {
        let raw_event = null_event();
        let result = ll::event::SDL_PollEvent(&raw_event);
        if result != 1 {
            return NoEvent;
        }

        match raw_event.type_ {
            SDL_QUIT => QuitEvent,
            SDL_KEYDOWN => {
                let raw_kb_event: &SDL_KeyboardEvent = transmute(&raw_event);
                KeyDownEvent(raw_kb_event.to_hl())
            }
            SDL_KEYUP => {
                let raw_kb_event: &SDL_KeyboardEvent = transmute(&raw_event);
                KeyUpEvent(raw_kb_event.to_hl())
            }
            _ => NoEvent
        }
    }
}
