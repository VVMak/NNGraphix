use crate::editor::types::{AppCoords, BoardCoords};

pub mod basic;
mod common;
pub mod dragged;

#[derive(Debug, PartialEq, Clone)]
pub enum State {
    Basic(basic::State),
    Dragged(dragged::State),
}

impl Default for State {
    fn default() -> Self {
        State::Basic(basic::State::default())
    }
}

impl State {
    pub fn set_new_state(&mut self, new_state: Self) -> &mut Self {
        *self = new_state;
        log::debug!("New viewbox state: {}", self);
        self
    }

    pub fn make_viewbox_str(&self) -> String {
        match self {
            Self::Basic(s) => s.make_viewbox_str(),
            Self::Dragged(s) => s.make_viewbox_str(),
        }
    }

    pub fn scale(&mut self, cursor: AppCoords, factor: f64) -> &mut Self {
        match self {
            Self::Basic(s) => {
                s.scale(cursor, factor);
            }
            Self::Dragged(s) => {
                s.scale(cursor, factor);
            }
        };
        self
    }

    #[allow(unused)]
    pub fn to_app_coords(&self, board_coords: BoardCoords) -> AppCoords {
        match self {
            Self::Basic(s) => s.to_app_coords(board_coords),
            Self::Dragged(s) => s.to_app_coords(board_coords),
        }
    }
    pub fn to_board_coords(&self, app_coords: AppCoords) -> BoardCoords {
        match self {
            Self::Basic(s) => s.to_board_coords(app_coords),
            Self::Dragged(s) => s.to_board_coords(app_coords),
        }
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Basic(_) => "Basic",
                Self::Dragged(_) => "Dragged",
            }
        )
    }
}
