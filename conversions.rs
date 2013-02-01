use event;
use ll;

pub trait ToLl<T> {
    fn to_ll(&self) -> T;
}

pub trait ToHl<T> {
    fn to_hl(&self) -> T;
}

pub impl ll::event::SDL_KeyboardEvent: ToHl<event::KeyboardEvent> {
    pub fn to_hl(&self) -> event::KeyboardEvent {
        event::KeyboardEvent {
            window_id: self.which,
            state: self.state,
            keycode: unsafe{ cast::transmute(self.keysym.sym as u64) },
            modifier: unsafe{ cast::transmute(self.keysym.mod_ as u32) }
        }
    }
}

pub impl ll::event::SDL_MouseMotionEvent: ToHl<event::MouseMotionEvent> {
    pub fn to_hl(&self) -> event::MouseMotionEvent {
        event::MouseMotionEvent {
            which: self.which,
            state: self.state,
            x: self.x,
            y: self.y,
            xrel: self.xrel,
            yrel: self.yrel
        }
    }
}
