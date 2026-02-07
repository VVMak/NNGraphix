use super::{block, internal, states::*};
use crate::tools::viewable::Viewable;

#[derive(Debug, PartialEq, Clone)]
pub struct State(internal::State);

impl State {
    pub(in crate::editor::board::state) fn from(internal: internal::State) -> Self {
        Self(internal)
    }

    pub fn cancel(self) -> basic::State {
        basic::State::from(self.0)
    }

    pub fn preview(self, end_block: block::Id) -> arrow_creation::preview::State {
        arrow_creation::preview::State::from(self.0, end_block)
    }

    pub fn to_states_enum(self) -> super::super::State {
        super::super::State::ArrowCreation(super::StateStages::Start(self))
    }
}

impl Viewable<yew::Html> for State {
    type Callback = yew::Callback<crate::editor::board::Event>;

    fn view(&self, callback: Self::Callback) -> yew::Html {
        self.0.html(callback)
    }
}