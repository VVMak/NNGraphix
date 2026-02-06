pub mod arrow_creation;
pub mod basic;
pub mod internal;
pub mod dragging_blocks;
pub mod predrag;
pub mod rectangle_selection;

pub mod states;

use super::block;
use crate::tools::viewable::Viewable;

#[derive(PartialEq, Clone)]
pub enum State {
    Basic(basic::State),
    PredragBlocks(predrag::State),
    DraggingBlocks(dragging_blocks::State),
    RectangleSelection(rectangle_selection::State),
    ArrowCreation(arrow_creation::StateStages),
}

impl Default for State {
    fn default() -> Self { State::Basic(basic::State::default()) }
}

impl State {
    #[allow(unused)]
    const GRID_SIZE: f64 = 80.0;

    #[allow(unused)]
    fn grid_html() -> yew::Html {
        yew::html! {
        <>
        <defs>
            <pattern id="smallGrid" width={(Self::GRID_SIZE / 10.0).to_string()} height={(Self::GRID_SIZE / 10.0).to_string()} patternUnits="userSpaceOnUse">
            <path d={format!("M {small_grid_size} 0 L 0 0 0 {small_grid_size}", small_grid_size=Self::GRID_SIZE / 10.0)} fill="none" stroke="gray" stroke-width="0.5"/>
            </pattern>
            <pattern id="grid" width={Self::GRID_SIZE.to_string()} height={Self::GRID_SIZE.to_string()} patternUnits="userSpaceOnUse">
            <rect width={Self::GRID_SIZE.to_string()} height={Self::GRID_SIZE.to_string()} fill="url(#smallGrid)"/>
            <path d={format!("M {grid_size} 0 L 0 0 0 {grid_size}", grid_size=Self::GRID_SIZE)} fill="none" stroke="gray" stroke-width="1"/>
            </pattern>
            </defs>
            <rect width="100%" height="100%" x="0" y="0" fill="url(#grid)" />
        </>
        }
    }

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