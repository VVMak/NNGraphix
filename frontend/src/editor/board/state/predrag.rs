use crate::editor::board::block::state::StateInterface;
use crate::tools::viewable::Viewable;

use super::{block, internal, states::*};

#[derive(Debug, Clone, PartialEq)]
pub enum SelectionModifier {
    None,
    Add,
}

#[derive(Debug, PartialEq, Clone)]
pub struct State {
    internal: internal::State,
    clicked_block: block::Id,
    selection_modifier: SelectionModifier,
}

impl State {
    pub fn from(internal: internal::State, clicked_block: block::Id, selection_modifier: SelectionModifier) -> Self {
        Self { internal, clicked_block, selection_modifier }
    }

    pub fn to_states_enum(self) -> super::State {
        super::State::PredragBlocks(self)
    }

    pub fn deselect(mut self) -> basic::State {
        match self.selection_modifier {
            SelectionModifier::Add => {
                self.internal.block_mut(self.clicked_block).set_selected(false);
            }
            SelectionModifier::None => {
                self.internal.clear_selection();
                self.internal.block_mut(self.clicked_block).set_selected(true);
            },
        }
        basic::State::from(self.internal)
    }

    pub fn drag_blocks(self) -> dragging_blocks::State {
        dragging_blocks::State::from(self.internal)
    }
}

impl Viewable<yew::Html> for State {
    type Callback = yew::Callback<crate::editor::board::Event>;

    fn view(&self, callback: Self::Callback) -> yew::Html {
        self.internal.html(callback)
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Block click, preparing to drag selected")
    }
}