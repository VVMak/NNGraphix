#[derive(Debug)]
pub enum State {
    Basic,
    DraggingSelection,
    DraggingBoard, // TODO: https://github.com/VVMak/NNGraphix/issues/10
    ArrowCreation,
}

impl Default for State {
    fn default() -> Self { State::Basic }
}