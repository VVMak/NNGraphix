use arrow::Arrow;
use block::Block;
use coords::Coords;
use super::tools;
use yew::Html;

pub struct Graph {
    arrow_id_gen: tools::IdGenerator,
    arrows: HashMap<tools::Id, Arrow>,
    block_id_gen: tools::IdGenerator,
    blocks: HashMap<tools::Id, Block>,
}

impl Graph {
    pub fn html() -> Html;
    pub fn create_block(coords: &Coords) -> tools::Id;
    pub fn create_arrow(from: &tools::Id, to: &tools::Id);
}