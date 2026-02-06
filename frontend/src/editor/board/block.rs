pub mod state;
pub mod event;
pub(super) mod vertex_data;

pub use state::StateDump;
pub use event::Event;

use crate::editor::board::block::state::StateInterface;

pub type Id = crate::tools::Id;

#[derive(PartialEq, yew::Properties)]
pub struct Props {
    pub state: StateDump,
    pub scope: yew::Callback<Event>,
}

#[derive(PartialEq, Clone, Debug, Default)]
pub struct Block;

impl Block {
    fn get_style(&self, selected: bool) -> String {
        let stroke_color = if selected { "blue" } else { "black" };
        let stroke = format!("stroke:{stroke_color};stroke-width:5; stroke-opacity: 0.5");
        let block_color = if selected { "rgb(100, 100, 255)" } else { "red" };
        format!("fill:{block_color};fill-opacity:0.5;{stroke}")
    }
    pub fn html(&self, props: &Props) -> yew::Html {
        let style = self.get_style(props.state.selected());
        let scope = props.scope.clone();
        let id = props.state.id().clone();
        let onmousedown = move |e: yew::MouseEvent| {
            e.stop_immediate_propagation();
            scope.emit(Event::MouseDown(e, id))
        };
        yew::html! {
            <g
            onmousedown={onmousedown}
            >
                <rect x={props.state.top_left().x.to_string()} y={props.state.top_left().y.to_string()}
                rx="20" ry="20" width={props.state.size().x.to_string()} height={props.state.size().y.to_string()}
                style={style}/>
            </g>
        }
    }
}

impl yew::Component for Block {
    type Message = Event;

    type Properties = Props;

    fn create(_: &yew::Context<Self>) -> Self {
        Self::default()
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        self.html(ctx.props())
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        ctx.props().scope.emit(msg);
        false
    }
}
