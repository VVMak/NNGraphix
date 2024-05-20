#[derive(Debug)]
#[derive(PartialEq)]
pub enum State {
    Basic,
    DraggingSelection,
    DraggingBoard,
    RectangleSelection,
    ArrowCreation,
}

impl Default for State {
    fn default() -> Self { State::Basic }
}