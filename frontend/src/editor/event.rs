use yew::{KeyboardEvent, MouseEvent, WheelEvent};

use crate::editor::types::AppCoords;

pub enum Event {
    MouseUp(MouseEvent),
    MouseDown(MouseEvent),
    MouseWheel(WheelEvent),
    KeyDown(KeyboardEvent),
    CursorMove { new_pos: AppCoords },
    BoardEvent(super::board::Event),
}
