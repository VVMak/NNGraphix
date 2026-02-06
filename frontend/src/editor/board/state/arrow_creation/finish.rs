use super::{block, internal, states::*};
use crate::tools::viewable::Viewable;

#[derive(Debug, PartialEq, Clone)]
pub struct State {
    internal: internal::State,
    end_block: block::Id,
}

impl State {
    pub fn from(internal: internal::State, end_block: block::Id) -> Self {
        Self { internal, end_block }
    }

    pub fn commit(mut self) -> basic::State {
        let end_block = self.end_block.clone();
        self.internal
            .iter_selected()
            .for_each(|mut start| start.entry_mut().add_outgoing(end_block));
        basic::State::from(self.internal)
    }

    pub fn cancel(self) -> basic::State {
        basic::State::from(self.internal)
    }

    pub fn to_states_enum(self) -> super::super::State {
        super::super::State::ArrowCreation(super::StateStages::Finish(self))
    }
}

impl Viewable<yew::Html> for State {
    type Callback = yew::Callback<crate::editor::board::Event>;

    fn view(&self, callback: Self::Callback) -> yew::Html {
        self.internal.html(callback)
    }
}