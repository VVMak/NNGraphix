use crate::editor::board::block::{self, state::StateInterface};

pub type Graph = super::super::graph::Graph<block::vertex_data::VertexData>;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct State {
    graph: Graph
}

impl State {
    #[allow(unused)]
    pub fn from(graph: Graph) -> Self { Self { graph } }

    pub fn graph(&self) -> &Graph { &self.graph }
    pub fn graph_mut(&mut self) -> &mut Graph { &mut self.graph }

    pub fn block_mut(&mut self, id: block::Id) -> block::state::State<'_> {
        block::state::State::from(self.graph_mut().entry(id).unwrap())
    }

    pub fn iter_blocks(&mut self) -> impl Iterator<Item = block::state::State<'_>> {
        self.graph_mut().iter_vertices()
            .map(|block| block::state::State::from(block))
    }

    pub fn iter_selected(&mut self) -> impl Iterator<Item = block::state::State<'_>> {
        self.iter_blocks().filter(|block| block.selected())
    }

    pub fn clear_selection(&mut self) {
        self.iter_selected().for_each(|mut block| block.set_selected(false));
    }

    pub fn blocks_html(&self, callback: yew::Callback<crate::editor::board::Event>) -> yew::Html {
        self.graph().iter_vertices().map(|entry| yew::html!{
            <block::Block
                state={block::state::State::from(entry).dump()}
                scope={callback.reform(|event| crate::editor::board::Event::BlockEvent(event))}
            />
        }).collect::<yew::Html>()
    }

    pub fn arrows_html(&self) -> yew::Html {
        self.graph()
            .iter_edges()
            .map(|edge|
                crate::editor::board::arrow::Arrow::from(edge, self.graph(), false)
                    .html()
            )
            .collect::<yew::Html>()
    }

    pub fn html(&self, callback: yew::Callback<crate::editor::board::Event>) -> yew::Html {
        yew::html!{
            <>
                {self.arrows_html()}
                {self.blocks_html(callback)}
            </>
        }
    }
}
