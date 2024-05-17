use yew::{KeyboardEvent, MouseEvent, WheelEvent};

use super::{tools, Coords};

pub enum Msg {
    MouseMove(Coords),
    MouseLeftUp,
    MouseWheelScale(WheelEvent),
    MouseLeftDownOutsideOfBlock,
    MouseLeftDownBlock(MouseEvent, tools::Id),
    // maybe we should change it to key codes for simpler matching
    KeyDown(KeyboardEvent),
}
