use std::{cell::RefCell, collections::HashMap};

use yew::{Html, html};

use super::block;
use block::Block;
use super::Vector;

pub type Id = super::tools::Id;

pub struct Arrow {
    pub id: Id,
    pub start_id: block::Id,
    pub end_id: block::Id,
}

impl Arrow {
    pub fn create_html(&self, blocks: &HashMap<block::Id, RefCell<Block>>) -> Html {
        let start = blocks[&self.start_id].borrow().get_control_point_out();
        let end = blocks[&self.end_id].borrow().get_control_point_in();
        let path_content = format!("M {} C {}, {}, {}",
                display_coords_path(&start.point),
                display_coords_path(&(start.point.clone() + start.vector.clone())),
                display_coords_path(&(end.point.clone() + end.vector.clone())),
                display_coords_path(&end.point),
        );
        html!{
            <>
            <path d={path_content} stroke="black" fill="transparent"/>
            {triangle_html(&end)}
            </>
        }
    }
}

impl PartialEq for Arrow {
    fn eq(&self, other: &Self) -> bool {
       self.start_id == other.start_id && self.end_id == other.end_id
    }
}
impl Eq for Arrow {}

pub struct ControlPoint {
    pub point: Vector,
    pub vector: Vector,
}

fn display_coords_path(coords: &Vector) -> String {
    format!("{} {}", coords.x, coords.y)
}

fn display_coords_poly(coords: &Vector) -> String {
    format!("{},{}", coords.x, coords.y)
}

const TR_DX: f64 = 7.0;
const TR_DY: f64 = TR_DX * 0.5774;

fn triangle_html(cp: &ControlPoint) -> Html {
    // TODO: this triangle ending works only with horizontal and long enough 'end' control vector
    let polygon_points = format!("{} {} {}",
            display_coords_poly(&(cp.point.clone() - Vector { x: TR_DX, y: TR_DY })),
            display_coords_poly(&(cp.point.clone() - Vector { x: TR_DX, y: -TR_DY })),
            display_coords_poly(&cp.point),
    );
    html!{
        <polygon points={polygon_points} fill="black" stroke-linejoin="round"/>
    }
}
