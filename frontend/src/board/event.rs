use yew::{KeyboardEvent, MouseEvent, WheelEvent};

use super::{tools, Vector};

pub enum Event {
    MouseMove(Vector),
    MouseWheel(WheelEvent),
    MouseDownBoard(MouseEvent),
    MouseDownBlock(MouseEvent, tools::Id),
    MouseUp(MouseEvent),
    // maybe we should change it to key codes for simpler matching
    KeyDown(KeyboardEvent),
}
