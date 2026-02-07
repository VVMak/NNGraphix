use yew::{html, Html};

use super::block;
use block::state::StateInterface;

use crate::editor::{board::graph, types::BoardCoords};

#[derive(Debug)]
pub struct Arrow<'a> {
    start: block::state::State<'a>,
    end: block::state::State<'a>,
    preview: bool,
}

impl<'a> Arrow<'a> {
    pub fn from(
        edge: graph::Edge,
        vertices: &'a graph::Graph<block::vertex_data::VertexData>,
        preview: bool,
    ) -> Self {
        Self {
            start: block::state::State::from(vertices.entry(edge.0).unwrap()),
            end: block::state::State::from(vertices.entry(edge.1).unwrap()),
            preview: preview,
        }
    }

    fn control_point_in(block: &block::state::State) -> ControlPoint {
        ControlPoint {
            point: block.center() + BoardCoords::new(block.size().x() * -0.5, 0.),
            vector: BoardCoords::new(block.size().x() * -0.5, 0.),
        }
    }
    fn control_point_out(block: &block::state::State) -> ControlPoint {
        ControlPoint {
            point: block.center() + BoardCoords::new(block.size().x() * 0.5, 0.),
            vector: BoardCoords::new(block.size().x() * 0.5, 0.),
        }
    }

    pub fn html(&self) -> Html {
        let start = Self::control_point_out(&self.start);
        let end = Self::control_point_in(&self.end);
        let path_content = format!(
            "M {} C {}, {}, {}",
            display_coords_path(&start.point),
            display_coords_path(&(start.point.clone() + start.vector.clone())),
            display_coords_path(&(end.point.clone() + end.vector.clone())),
            display_coords_path(&end.point),
        );
        let color = if self.preview { "grey" } else { "black" };
        html! {
            <>
            <path
                d={path_content}
                stroke={color}
                fill="transparent"/>
            {triangle_html(&end, color.to_string())}
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
    pub point: BoardCoords,
    pub vector: BoardCoords,
}

fn display_coords_path(coords: &BoardCoords) -> String {
    format!("{} {}", coords.x(), coords.y())
}

fn display_coords_poly(coords: &BoardCoords) -> String {
    format!("{},{}", coords.x(), coords.y())
}

const TR_DX: f64 = 7.0;
const TR_DY: f64 = TR_DX * 0.5774;

fn triangle_html(cp: &ControlPoint, color: String) -> Html {
    // TODO: this triangle ending works only with horizontal and long enough 'end' control vector
    let polygon_points = format!(
        "{} {} {}",
        display_coords_poly(&(cp.point.clone() - BoardCoords::new(TR_DX, TR_DY))),
        display_coords_poly(&(cp.point.clone() - BoardCoords::new(TR_DX, -TR_DY))),
        display_coords_poly(&cp.point),
    );
    html! {
        <polygon
            points={polygon_points}
            fill={color}
            stroke-linejoin="round"
        />
    }
}
