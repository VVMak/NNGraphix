pub mod start;
pub mod finish;

use super::{block, internal, states};
use crate::tools::viewable::Viewable;

#[derive(Debug, PartialEq, Clone)]
pub enum StateStages {
    Start(start::State),
    Finish(finish::State),
}

impl Viewable<yew::Html> for StateStages {
    type Callback = yew::Callback<crate::editor::board::Event>;

    fn view(&self, callback: Self::Callback) -> yew::Html {
        match self {
            StateStages::Start(state) => state.view(callback),
            StateStages::Finish(state) => state.view(callback),
        }
    }
}

impl std::fmt::Display for StateStages {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Arrow creation {}", match self {
            Self::Start(_) => "start",
            Self::Finish(_) => "finish",
        })
    }
}