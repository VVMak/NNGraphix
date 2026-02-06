use glam::DVec2;

use yew::{Html, html};

use super::block;
use block::state::StateInterface;

use crate::editor::board::graph;

#[derive(Debug)]
pub struct Arrow<'a> {
    start: block::state::State<'a>,
    end: block::state::State<'a>,
}

impl<'a> Arrow<'a> {
    pub fn from(edge: graph::Edge, vertices: &'a graph::Graph<block::vertex_data::VertexData>) -> Self {
        Self {
            start: block::state::State::from(vertices.entry(edge.0).unwrap()),
            end: block::state::State::from(vertices.entry(edge.1).unwrap()),
        }
    }

    fn control_point_in(block: &block::state::State) -> ControlPoint {
        ControlPoint {
            point: block.center() + block.size() * DVec2 { x: -0.5, y: 0. },
            vector: block.size() * DVec2 { x: -0.5, y: 0. },
        }
    }
    fn control_point_out(block: &block::state::State) -> ControlPoint {
        ControlPoint {
            point: block.center() + block.size() * DVec2 { x: 0.5, y: 0. },
            vector: block.size() * DVec2 { x: 0.5, y: 0. },
        }
    }

    pub fn html(&self) -> Html {
        let start = Self::control_point_out(&self.start);
        let end = Self::control_point_in(&self.end);
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

impl<'a> PartialEq for Arrow<'a> {
    fn eq(&self, other: &Self) -> bool {
       self.start.id() == other.start.id() && self.end.id() == other.end.id()
    }
}
impl<'a> Eq for Arrow<'a> {}

pub struct ControlPoint {
    pub point: DVec2,
    pub vector: DVec2,
}

fn display_coords_path(coords: &DVec2) -> String {
    format!("{} {}", coords.x, coords.y)
}

fn display_coords_poly(coords: &DVec2) -> String {
    format!("{},{}", coords.x, coords.y)
}

const TR_DX: f64 = 7.0;
const TR_DY: f64 = TR_DX * 0.5774;

fn triangle_html(cp: &ControlPoint) -> Html {
    // TODO: this triangle ending works only with horizontal and long enough 'end' control vector
    let polygon_points = format!("{} {} {}",
            display_coords_poly(&(cp.point.clone() - DVec2 { x: TR_DX, y: TR_DY })),
            display_coords_poly(&(cp.point.clone() - DVec2 { x: TR_DX, y: -TR_DY })),
            display_coords_poly(&cp.point),
    );
    html!{
        <polygon points={polygon_points} fill="black" stroke-linejoin="round"/>
    }
}
