use crate::editor::types::{AppCoords, BoardCoords};

use super::common::Viewbox;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct State(Viewbox);

impl State {
    pub(super) fn from(viewbox: Viewbox) -> Self {
        State(viewbox)
    }

    pub fn to_states_enum(self) -> super::State {
        super::State::Basic(self)
    }

    pub fn drag(self) -> super::dragged::State {
        super::dragged::State::from(self.0)
    }

    pub fn scale(&mut self, cursor: AppCoords, factor: f64) -> &mut Self {
        self.0.scale(cursor, factor);
        self
    }

    pub fn make_viewbox_str(&self) -> String {
        self.0.make_viewbox_str()
    }

    #[allow(unused)]
    pub fn to_app_coords(&self, board_coords: BoardCoords) -> AppCoords {
        self.0.to_app_coords(board_coords)
    }
    pub fn to_board_coords(&self, app_coords: AppCoords) -> BoardCoords {
        self.0.to_board_coords(app_coords)
    }
}
