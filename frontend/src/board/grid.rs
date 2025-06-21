use super::vector::Vector;

use web_sys;

const BASE_SCALE: f64 = 1.0;
const GRID_SIZE: f64 = 80.0;

pub struct Grid {
    window_pos: Vector, // points to bottom right corner from top left corner
    scale: Vector, // how scaled the grid is
}

impl Grid {
    pub fn new() -> Grid {
        return Grid {
            scale: Vector { x: BASE_SCALE, y: BASE_SCALE },
            window_pos: Vector { x: 0.0, y: 0.0 },
        }
    }
    fn to_app_coords(&self, grid_coords: Vector) -> Vector {
        (grid_coords - self.window_pos) / self.scale
    }
    fn to_grid_coords(&self, app_coords: Vector) -> Vector {
        app_coords * self.scale + self.window_pos
    }
    pub fn scale(&mut self, pos: Vector, factor: f64) {
        let grid_pos = self.to_grid_coords(pos);
        let grid_window_pos = self.window_pos - grid_pos;
        self.scale *= factor;
        self.window_pos = grid_pos - grid_window_pos / factor;
    }
    pub fn board_size(&self) -> Vector {
        let window = web_sys::window().expect("There should be a window");
        Vector {
            x: window
                .inner_width()
                .expect("The window should have Some width")
                .as_f64()
                .expect("The width should be a number"),
            y: window
                .inner_height()
                .expect("The window should have Some height")
                .as_f64()
                .expect("The height should be a number"),
        } / self.scale
    }
}