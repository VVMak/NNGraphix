use yew::{MouseEvent};

use crate::tools::Id;

pub enum Event {
    MouseDown(MouseEvent, Id),
    MouseOver(Id),
    MouseLeave
}
