use crate::editor::types::AppCoords;

#[derive(Default, Clone, Copy)]
pub struct Cursor {
    pos: AppCoords,
}

impl Cursor {
    pub fn update(&mut self, new_pos: AppCoords) -> AppCoords {
        let pos = self.pos;
        self.pos = new_pos;
        pos
    }

    pub fn get(&self) -> AppCoords {
        self.pos
    }
}
