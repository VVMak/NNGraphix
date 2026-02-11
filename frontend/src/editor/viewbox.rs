use std::mem;

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

    pub fn make_viewbox_tuple(&self) -> (f64, f64, f64, f64) {
        match self {
            Self::Basic(s) => s.make_viewbox_tuple(),
            Self::Dragged(s) => s.make_viewbox_tuple(),
        }
    }
    pub fn zoom_at_cursor(&mut self, cursor_pos: AppCoords, factor: f64) -> &mut Self {
        match self {
            Self::Basic(s) => {
                s.zoom_at_cursor(cursor_pos, factor);
            }
            Self::Dragged(s) => {
                s.zoom_at_cursor(cursor_pos, factor);
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
    pub fn handle_mouse_move(&mut self, old_pos: AppCoords, new_pos: AppCoords) -> bool {
        match self {
            Self::Dragged(s) => s.handle_mouse_move(old_pos, new_pos),
            _ => false,
        }
    }
    pub fn handle_middle_mouse_button_release(&mut self) -> bool {
        let current = mem::take(self);
        match current {
            State::Basic(s) => {
                self.set_new_state(State::Basic(s));
                log::warn!("basic state on middle click mouse release");
                false
            }
            State::Dragged(s) => {
                let new_state = s.drop().to_states_enum();
                self.set_new_state(new_state);
                false
            }
        }
    }
    pub fn handle_middle_mouse_button_press(&mut self) -> bool {
        let current = mem::take(self);
        match current {
            State::Basic(s) => {
                let new_state = s.drag().to_states_enum();
                self.set_new_state(new_state);
            }
            State::Dragged(s) => {
                self.set_new_state(State::Dragged(s));
                log::warn!("dragged state on middle click mouse hold");
            }
        };
        false
    }
    pub fn handle_mouse_wheel(&mut self, cursor_pos: AppCoords, factor: f64) -> bool {
        self.zoom_at_cursor(cursor_pos, factor);
        true
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
