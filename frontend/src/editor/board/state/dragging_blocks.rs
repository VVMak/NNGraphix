use crate::editor::board::block::state::StateInterface;
use crate::tools::viewable::Viewable;

use super::{internal, states::*};

#[derive(Debug, PartialEq, Clone)]
pub struct State(internal::State);

impl State {
    pub fn from(internal: internal::State) -> Self {
        Self(internal)
    }

    pub fn to_states_enum(self) -> super::State {
        super::State::DraggingBlocks(self)
    }

    pub fn move_selected(mut self, delta: glam::DVec2) -> Self {
        self.0.iter_selected().for_each(|mut block| block.move_block(delta));
        self
    }
    pub fn stop(self) -> basic::State {
        basic::State::from(self.0)
    }
}

impl Viewable<yew::Html> for State {
    type Callback = yew::Callback<crate::editor::board::Event>;

    fn view(&self, callback: Self::Callback) -> yew::Html {
        self.0.html(callback)
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Dragging blocks")
    }
}