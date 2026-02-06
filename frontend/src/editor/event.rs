use yew::{KeyboardEvent, MouseEvent, WheelEvent};

pub enum Event {
    MouseUp(MouseEvent),
    MouseDown(MouseEvent),
    MouseWheel(WheelEvent),
    KeyDown(KeyboardEvent),
    CursorMove{new_value: glam::DVec2},
    BoardEvent(super::board::Event),
}
