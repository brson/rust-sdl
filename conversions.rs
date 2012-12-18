pub trait ToLl<T> {
    fn to_ll(&self) -> T;
}

pub trait ToHl<T> {
    fn to_hl(&self) -> T;
}

pub impl ll::event::SDL_KeyboardEvent: ToHl<event::KeyboardEvent> {
    pub fn to_hl(&self) -> event::KeyboardEvent {
        event::KeyboardEvent {
            window_id: 0,
            state: 0,
            keycode: keyboard::SDLKUnknown,
            modifier: 0
        };
    }
}
