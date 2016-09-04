use util::units::{Point, Size};
use engine::Actor;

pub struct Popup {
    pub actor: Actor,
    pub message: String,
    pub is_visible: bool,
}

impl Popup {
    pub fn new(actor: Actor, location: Point, size: Size, message: String) -> Popup {
        Popup {
            actor: actor,
            message: message,
            is_visible: false,
        }
    }

    pub fn set_visible(&mut self, is_visible: bool) {
        self.is_visible = is_visible;
    }
}
