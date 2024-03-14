use std::collections::HashMap;

use yew::{Html, html};

use super::block::Block;
use super::tools;
use super::Coords;

pub struct Arrow {
    pub id: tools::Id,
    pub start_id: tools::Id,
    pub end_id: tools::Id,
}

impl Arrow {
    pub fn create_html(&self, blocks: &HashMap<tools::Id, Block>) -> Html {
        let start = blocks[&self.start_id].get_control_point_out();
        let end = blocks[&self.end_id].get_control_point_in();
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

#[derive(Hash)]
pub struct ControlPoint {
    pub point: Coords,
    pub vector: Coords,
}

fn display_coords_path(coords: &Coords) -> String {
    format!("{} {}", coords.x, coords.y)
}

fn display_coords_poly(coords: &Coords) -> String {
    format!("{},{}", coords.x, coords.y)
}

const TR_DX: i32 = 7;
const TR_DY: i32 = (TR_DX as f32 * 0.5774) as i32;

fn triangle_html(cp: &ControlPoint) -> Html {
    // TODO: this triangle ending works only with horizontal and long enough 'end' control vector
    let polygon_points = format!("{} {} {}",
            display_coords_poly(&(cp.point.clone() - Coords { x: TR_DX, y: TR_DY })),
            display_coords_poly(&(cp.point.clone() - Coords { x: TR_DX, y: -TR_DY })),
            display_coords_poly(&cp.point),
    );
    html!{
        <polygon points={polygon_points} fill="black" stroke-linejoin="round"/>
    }
}
