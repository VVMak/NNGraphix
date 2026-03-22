use yew::MouseEvent;

use shared::tools::Id;

pub enum Event {
    MouseDown(MouseEvent, Id),
    MouseOver(Id),
    MouseLeave,
}
