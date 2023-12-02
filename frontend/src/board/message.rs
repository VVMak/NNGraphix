pub enum Msg {
    MouseMove(super::Coords),
    MouseLeftUp,
    MouseLeftDownBlock(super::BlockId),
    // maybe we should change it to key codes for simpler matching
    KeyDown(String),
}