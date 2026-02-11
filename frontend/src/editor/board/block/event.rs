use yew::MouseEvent;

use crate::utils::Id;

pub enum Event {
    MouseDown(MouseEvent, Id),
    MouseOver(Id),
    MouseLeave,
}
