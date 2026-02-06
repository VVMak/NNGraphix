use glam::DVec2;

#[derive(Default, Clone, Copy)]
pub struct Cursor {
    pos: DVec2,
}

impl Cursor {
    pub fn update(&mut self, new_pos: DVec2) -> DVec2 {
        let pos = self.pos;
        self.pos = new_pos;
        self.pos - pos
    }

    pub fn get(&self) -> DVec2 { self.pos }
}

