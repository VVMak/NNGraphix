use super::{block, internal, states::*};
use crate::{
    editor::board::{arrow::Arrow, block::state::StateInterface},
    utils::viewable::Viewable,
};

#[derive(Debug, PartialEq, Clone)]
pub struct State {
    internal: internal::State,
    end_block: block::Id,
}

impl State {
    pub fn from(internal: internal::State, end_block: block::Id) -> Self {
        Self {
            internal,
            end_block,
        }
    }

    pub fn finish(self) -> arrow_creation::finish::State {
        arrow_creation::finish::State::from(self.internal, self.end_block)
    }

    pub fn cancel_preview(self) -> arrow_creation::start::State {
        arrow_creation::start::State::from(self.internal)
    }

    pub fn cancel_creation(self) -> basic::State {
        basic::State::from(self.internal)
    }

    pub fn to_states_enum(self) -> super::super::State {
        super::super::State::ArrowCreation(super::StateStages::Preview(self))
    }
}

impl Viewable<yew::Html> for State {
    type Callback = yew::Callback<crate::editor::board::Event>;

    fn view(&self, callback: Self::Callback) -> yew::Html {
        self.internal
            .graph()
            .iter_vertices()
            .map(|block| block::state::State::from(block))
            .filter(|block| block.selected())
            .map(|start| {
                Arrow::from(
                    (start.entry().id(), self.end_block),
                    self.internal.graph(),
                    true,
                )
                .html()
            })
            .chain(std::iter::once(self.internal.html(callback)))
            .collect::<yew::Html>()
    }
}

