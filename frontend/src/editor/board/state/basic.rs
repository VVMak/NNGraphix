use crate::editor::{board::block::state::StateInterface, types::BoardCoords};
use crate::utils::viewable::Viewable;

use super::{block, internal, states::*};
use block::vertex_data::VertexData;

pub use predrag::SelectionModifier;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct State(internal::State);

pub enum HoldBlockResult {
    Basic(State),
    PredragBlocks(predrag::State),
}

impl HoldBlockResult {
    pub fn to_states_enum(self) -> super::State {
        match self {
            Self::Basic(s) => s.to_states_enum(),
            Self::PredragBlocks(s) => s.to_states_enum(),
        }
    }
}

impl State {
    pub(super) fn from(internal: internal::State) -> Self {
        Self(internal)
    }

    pub fn to_states_enum(self) -> super::State {
        super::State::Basic(self)
    }

    pub fn hold_block(
        mut self,
        block_id: block::Id,
        modifier: SelectionModifier,
    ) -> HoldBlockResult {
        if !self.0.block_mut(block_id).selected() {
            if modifier == SelectionModifier::None {
                self.0.clear_selection();
            }
            self.0.block_mut(block_id).set_selected(true);
            if modifier == SelectionModifier::None {
                HoldBlockResult::PredragBlocks(predrag::State::from(self.0, block_id, modifier))
            } else {
                HoldBlockResult::Basic(self)
            }
        } else {
            HoldBlockResult::PredragBlocks(predrag::State::from(self.0, block_id, modifier))
        }
    }

    fn can_create_arrow(&self) -> bool {
        self.0
            .graph()
            .iter_vertices()
            .any(|block| block::state::State::from(block).selected())
    }

    fn create_arrow(self) -> arrow_creation::start::State {
        arrow_creation::start::State::from(self.0)
    }
    pub fn try_create_arrow(self) -> Result<arrow_creation::start::State, Self> {
        if self.can_create_arrow() {
            Ok(self.create_arrow())
        } else {
            log::info!("cannot create arrow: no selected blocks");
            Err(self)
        }
    }

    pub fn create_block(&mut self, pos: BoardCoords) -> block::Id {
        self.0.clear_selection();
        let mut entry =
            block::state::State::from(self.0.graph_mut().new_vertex(VertexData::from(pos)));
        entry.set_selected(true);
        entry.id()
    }

    pub fn start_rectangle_selection(self, start: BoardCoords) -> rectangle_selection::State {
        rectangle_selection::State::from(self.0, start.clone(), start)
    }

    pub fn remove_selected_blocks(&mut self) {
        let selected = self
            .0
            .iter_selected()
            .map(|block| block.id())
            .collect::<Vec<_>>();
        selected.into_iter().for_each(|block_id| {
            self.0.graph_mut().remove_vertex(block_id);
        });
    }

    pub fn clear_selection(&mut self) -> &mut Self {
        self.0.clear_selection();
        self
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
        write!(f, "Basic")
    }
}
