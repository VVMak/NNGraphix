use yew::{KeyboardEvent, MouseEvent, WheelEvent};

use super::{tools, Vector};

pub enum Event {
    MouseMove(Vector),
    MouseWheel(WheelEvent),
    MouseClick(MouseEvent),
    MouseDownBoard(MouseEvent),
    MouseDownBlock(MouseEvent, tools::Id),
    MouseLeftUp(MouseEvent),
    // maybe we should change it to key codes for simpler matching
    KeyDown(KeyboardEvent),
}
