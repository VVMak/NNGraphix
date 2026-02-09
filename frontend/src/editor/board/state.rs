pub mod arrow_creation;
pub mod basic;
pub mod dragging_blocks;
pub mod internal;
pub mod predrag;
pub mod rectangle_selection;

pub mod states;

use super::block;
use crate::utils::viewable::Viewable;

#[derive(PartialEq, Clone)]
pub enum State {
    Basic(basic::State),
    PredragBlocks(predrag::State),
    DraggingBlocks(dragging_blocks::State),
    RectangleSelection(rectangle_selection::State),
    ArrowCreation(arrow_creation::StateStages),
}

impl Default for State {
    fn default() -> Self {
        State::Basic(basic::State::default())
    }
}

impl State {
    pub fn set_new_state(&mut self, new_state: Self) -> &mut Self {
        *self = new_state;
        log::debug!("New board state: {}", self);
        self
    }
}

impl Viewable<yew::Html> for State {
    type Callback = yew::Callback<crate::editor::board::Event>;

    fn view(&self, callback: Self::Callback) -> yew::Html {
        match self {
            State::Basic(state) => state.view(callback),
            State::PredragBlocks(state) => state.view(callback),
            State::DraggingBlocks(state) => state.view(callback),
            State::RectangleSelection(state) => state.view(callback),
            State::ArrowCreation(state) => state.view(callback),
        }
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Basic(s) => s.fmt(f),
            State::PredragBlocks(s) => s.fmt(f),
            State::DraggingBlocks(s) => s.fmt(f),
            State::RectangleSelection(s) => s.fmt(f),
            State::ArrowCreation(stages) => stages.fmt(f),
        }
    }
}
