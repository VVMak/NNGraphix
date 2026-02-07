use crate::editor::types::{AppCoords, BoardCoords, Scale};

const SCALES: &[f64] = &[
    0.25, 0.33, 0.5, 0.66, 0.75, 0.9, 1.0, 1.1, 1.2, 1.5, 2.0, 3.0, 4.0,
];
const DEFAULT_SCALE_INDEX: usize = 6;

#[derive(Debug, Clone, PartialEq)]
pub struct Viewbox {
    pos: BoardCoords,   // points to top left corner
    scale_index: usize, // index in SCALES array
}

impl Default for Viewbox {
    fn default() -> Self {
        Self {
            pos: BoardCoords::default(),
            scale_index: DEFAULT_SCALE_INDEX,
        }
    }
}

impl Viewbox {
    fn get_window_size() -> AppCoords {
        let window = web_sys::window().expect("There should be a window");
        AppCoords::new(
            window
                .inner_width()
                .expect("The window should have Some width")
                .as_f64()
                .expect("The width should be a number"),
            window
                .inner_height()
                .expect("The window should have Some height")
                .as_f64()
                .expect("The height should be a number"),
        )
    }
    fn get_scale(&self) -> Scale {
        Scale::new(SCALES[self.scale_index], SCALES[self.scale_index])
    }
    fn board_size(&self) -> BoardCoords {
        self.to_board_coords(Self::get_window_size()) - self.to_board_coords(AppCoords::default())
    }
    #[allow(unused)]
    pub fn to_app_coords(&self, board_coords: BoardCoords) -> AppCoords {
        (board_coords - self.pos) * self.get_scale()
    }
    pub fn to_board_coords(&self, app_coords: AppCoords) -> BoardCoords {
        app_coords / self.get_scale() + self.pos
    }

    pub fn scale(&mut self, cursor: AppCoords, factor: f64) {
        let cursor_board_pos = self.to_board_coords(cursor);
        if factor > 0. && self.scale_index > 0 {
            self.scale_index -= 1;
        } else if factor < 0. && self.scale_index < SCALES.len() - 1 {
            self.scale_index += 1;
        }
        let new_cursor_board_pos = self.to_board_coords(cursor);
        self.pos += cursor_board_pos - new_cursor_board_pos;
    }
    pub fn move_box(&mut self, delta: BoardCoords) {
        self.pos += delta;
    }

    pub fn make_viewbox_str(&self) -> String {
        let board_size = self.board_size();
        format!(
            "{box_size_x}, {box_size_y}, {width}, {height}",
            box_size_x = self.pos.x(),
            box_size_y = self.pos.y(),
            width = board_size.x(),
            height = board_size.y(),
        )
    }
}

