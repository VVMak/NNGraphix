use yew::KeyboardEvent;

use super::{tools, Coords};

pub enum Msg {
    MouseMove(Coords),
    MouseLeftUp,
    MouseLeftDownBlock(tools::Id),
    // maybe we should change it to key codes for simpler matching
    KeyDown(KeyboardEvent),
}
