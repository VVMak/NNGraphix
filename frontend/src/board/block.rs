use std::collections::HashMap;

use yew::prelude::{Html, html};

use super::Coords;
use super::arrow;

pub type Id = super::tools::Id;

#[derive(PartialEq, Clone, Debug)]
pub struct Block {
    pub id: Id,
    pub upper_left: Coords,
    pub next: HashMap<Id, arrow::Id>,
    pub prev: HashMap<Id, arrow::Id>,
    selected: bool,
}

const BLOCK_WIDTH: i32 = 150;
const BLOCK_HEIGHT: i32 = 150;
const CONTROL_POINT_VECTOR_LENGTH: i32 = BLOCK_WIDTH / 2;

impl Block {
    pub fn new(id: Id, center: Coords) -> Block {
        Block { id, upper_left: center - Coords { x: BLOCK_WIDTH / 2, y: BLOCK_HEIGHT / 2 },
                next: HashMap::new(), prev: HashMap::new(), selected: false }
    }
    pub fn select(&mut self) { self.selected = true; }
    pub fn deselect(&mut self) { self.selected = false; }
    pub fn get_rect_html(&self) -> Html {
        let style = self.get_style();
        html!{
            <rect x={self.upper_left.x.to_string()} y={self.upper_left.y.to_string()}
            rx="20" ry="20" width={BLOCK_WIDTH.to_string()} height={BLOCK_HEIGHT.to_string()}
            style={style}/>
        }
    }

    pub fn get_control_point_out(&self) -> arrow::ControlPoint {
        arrow::ControlPoint {
            point: self.upper_left.clone() + Coords { x: BLOCK_WIDTH, y: BLOCK_HEIGHT / 2 },
            vector: Coords { x: CONTROL_POINT_VECTOR_LENGTH, y: 0 }
        }
    }
    pub fn get_control_point_in(&self) -> arrow::ControlPoint {
        arrow::ControlPoint {
            point: self.upper_left.clone() + Coords { x: 0, y: BLOCK_HEIGHT / 2 },
            vector: Coords { x: -CONTROL_POINT_VECTOR_LENGTH, y: 0 }
        }
    }

    pub fn add_next(&mut self, block_id: Id, arrow_id: arrow::Id) {
        self.next.insert(block_id, arrow_id);
    }
    pub fn remove_next(&mut self, block_id: &Id) -> Option<arrow::Id> {
        self.next.remove(block_id)
    }
    pub fn add_prev(&mut self, block_id: Id, arrow_id: arrow::Id) {
        self.prev.insert(block_id, arrow_id);
    }
    pub fn remove_prev(&mut self, block_id: &Id) -> Option<arrow::Id> {
        self.prev.remove(block_id)
    }

    fn get_style(&self) -> String {
        let stroke_color = if self.selected {"blue"} else {"black"};
        let stroke = format!("stroke:{stroke_color};stroke-width:5; stroke-opacity: 0.5");
        let block_color = if self.selected {"rgb(100, 100, 255)"} else {"red"};
        format!("fill:{block_color};fill-opacity:0.5;{stroke}")
    }

}
