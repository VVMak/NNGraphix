use std::collections::HashMap;

use yew::prelude::{Html, html};

use super::Vector;
use super::arrow;

pub type Id = super::tools::Id;

#[derive(PartialEq, Clone, Debug)]
pub struct Block {
    pub id: Id,
    pub upper_left: Vector,
    pub next: HashMap<Id, arrow::Id>,
    pub prev: HashMap<Id, arrow::Id>,
    selected: bool,
}

const BLOCK_WIDTH: f64 = 150.0;
const BLOCK_HEIGHT: f64 = 150.0;
const CONTROL_POINT_VECTOR_LENGTH: f64 = BLOCK_WIDTH / 2.0;

impl Block {
    pub fn new(id: Id, center: Vector) -> Block {
        Block { id, upper_left: center - Vector { x: BLOCK_WIDTH / 2.0, y: BLOCK_HEIGHT / 2.0 },
                next: HashMap::new(), prev: HashMap::new(), selected: false }
    }
    pub fn select(&mut self) { self.selected = true; }
    pub fn deselect(&mut self) { self.selected = false; }
    pub fn is_selected(&self) -> bool{
        return self.selected;
    }
    pub fn get_rect_html(&self) -> Html {
        let style = self.get_style();
        html!{
            <rect x={self.upper_left.x.to_string()} y={self.upper_left.y.to_string()}
            rx="20" ry="20" width={BLOCK_WIDTH.to_string()} height={BLOCK_HEIGHT.to_string()}
            style={style}/>
        }
    }
    pub fn get_block_width() -> f64 {
        return BLOCK_WIDTH;
    }
    pub fn get_block_height() -> f64 {
        return BLOCK_HEIGHT;
    }
    pub fn get_control_point_out(&self) -> arrow::ControlPoint {
        arrow::ControlPoint {
            point: self.upper_left.clone() + Vector { x: BLOCK_WIDTH, y: BLOCK_HEIGHT / 2.0 },
            vector: Vector { x: CONTROL_POINT_VECTOR_LENGTH, y: 0.0}
        }
    }
    pub fn get_control_point_in(&self) -> arrow::ControlPoint {
        arrow::ControlPoint {
            point: self.upper_left.clone() + Vector { x: 0.0, y: BLOCK_HEIGHT / 2.0 },
            vector: Vector { x: -CONTROL_POINT_VECTOR_LENGTH, y: 0.0 }
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
