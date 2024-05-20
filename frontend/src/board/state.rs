#[derive(Debug)]
pub enum State {
    Basic,
    DraggingSelection,
    DraggingBoard,
    ArrowCreation,
}

impl Default for State {
    fn default() -> Self { State::Basic }
}