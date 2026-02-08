mod arrow;
pub mod block;
mod event;
mod graph;
pub mod state;

use yew::{prelude::Context, Properties};

pub use event::Event;
pub use state::State;

use crate::tools::viewable::Viewable;

#[allow(unused)]
#[derive(PartialEq, Properties)]
pub struct Props {
    state: State,
    scope: yew::Callback<crate::editor::Event>,
}

#[allow(unused)]
#[derive(Default)]
pub struct Board;

impl yew::Component for Board {
    type Message = Event;

    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Board::default()
    }

    fn view(&self, ctx: &Context<Self>) -> yew::Html {
        ctx.props()
            .state
            .view(ctx.link().callback(std::convert::identity))
    }

    fn update(&mut self, ctx: &Context<Self>, event: Self::Message) -> bool {
        ctx.props()
            .scope
            .emit(crate::editor::Event::BoardEvent(event));
        false
    }
}
