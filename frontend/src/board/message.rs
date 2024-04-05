use yew::{KeyboardEvent, MouseEvent};

use super::{tools, Coords};

pub enum Msg {
    MouseMove(Coords),
    MouseLeftUp,
    MouseLeftDownBlock(MouseEvent, tools::Id),
    // maybe we should change it to key codes for simpler matching
    KeyDown(KeyboardEvent),
}
