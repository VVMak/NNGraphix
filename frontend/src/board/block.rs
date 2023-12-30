mod block_id;

pub use block_id::{BlockId, BlockIdGenerator};
use yew::prelude::{Html, html};

use super::Coords;

#[derive(PartialEq, Clone, Debug)]
pub struct Block {
    pub id: BlockId,
    pub upper_left: Coords,
    selected: bool,
}

const BLOCK_WIDTH: i32 = 150;
const BLOCK_HEIGHT: i32 = 150;

impl Block {
    pub fn new(id: BlockId, center: Coords) -> Block {
        Block { id, upper_left: center - Coords { x: BLOCK_WIDTH / 2, y: BLOCK_HEIGHT / 2 }, selected: false }
    }
    pub fn select(&mut self) { self.selected = true; }
    pub fn unselect(&mut self) { self.selected = false; }
    pub fn get_rect_html(&self) -> Html {
        let style = self.get_style();
        html!{
            <rect x={self.upper_left.x.to_string()} y={self.upper_left.y.to_string()}
            rx="20" ry="20" width={BLOCK_WIDTH.to_string()} height={BLOCK_HEIGHT.to_string()}
            style={style}/>
        }
    }
    
    fn get_style(&self) -> String {
        let stroke_color = if self.selected {"blue"} else {"black"};
        let stroke = format!("stroke:{stroke_color};stroke-width:5; stroke-opacity: 0.5");
        let block_color = if self.selected {"rgb(100, 100, 255)"} else {"red"};
        format!("fill:{block_color};fill-opacity:0.5;{stroke}")
    }
}
