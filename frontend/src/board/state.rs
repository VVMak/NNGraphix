use crate::board::vector::Vector;

#[derive(Debug, PartialEq, Clone)]
pub enum State {
    Basic,
    DraggingSelection,
    DraggingBoard,
    RectangleSelection(Vector),
    ArrowCreation,
}

impl Default for State {
    fn default() -> Self { State::Basic }
}
