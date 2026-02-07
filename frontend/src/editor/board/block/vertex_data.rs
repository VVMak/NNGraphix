use crate::editor::types::BoardCoords;

#[derive(PartialEq, Debug, Clone)]
pub struct VertexData {
    pub(super) center: BoardCoords,
    pub(super) selected: bool,
}

impl VertexData {
    pub fn from(center: BoardCoords) -> Self {
        Self {
            center,
            selected: false,
        }
    }
}
