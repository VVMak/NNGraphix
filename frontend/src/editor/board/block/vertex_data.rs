use glam::DVec2;

#[derive(PartialEq, Debug, Clone)]
pub struct VertexData {
    pub(super) center: DVec2,
    pub(super) selected: bool,
}

impl VertexData {
    pub fn from(center: DVec2) -> Self {
        Self { center, selected: false }
    }
}
