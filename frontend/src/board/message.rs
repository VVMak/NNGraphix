use yew::KeyboardEvent;

use super::{BlockId, Coords};

pub enum Msg {
    MouseMove(Coords),
    MouseLeftUp,
    MouseLeftDownBlock(BlockId),
    // maybe we should change it to key codes for simpler matching
    KeyDown(KeyboardEvent),
}
