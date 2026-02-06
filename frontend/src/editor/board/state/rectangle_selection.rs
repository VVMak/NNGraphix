use glam::DVec2;

use crate::editor::board::block::state::StateInterface;
use crate::tools::viewable::Viewable;

use super::states::*;

#[derive(Debug, PartialEq, Clone)]
pub struct State {
    internal: internal::State,
    start: DVec2,
    end: DVec2,
}

impl State {
    pub fn from(internal: internal::State, start: DVec2, end: DVec2) -> Self {
        Self { internal, start, end }
    }

    pub fn to_states_enum(self) -> super::State {
        super::State::RectangleSelection(self)
    }

    pub fn move_end(mut self, new_value: glam::DVec2) -> Self {
        self.end = new_value;
        self
    }

    pub fn finish(mut self) -> basic::State {
        self.internal
            .iter_blocks()
            .filter(|block| rectangles_overlap(
                self.start, self.end, block.top_left(), block.bottom_right())
            )
            .for_each(|mut block| block.set_selected(true));
        basic::State::from(self.internal)
    }

    pub fn draw_selection_rect(&self) -> yew::Html {
        let (top_left, bottom_right) = sort_rectangle_coordinates(self.start, self.end);
        let size = bottom_right.clone() - top_left.clone();
        yew::html!{
            <svg>
                <rect x={top_left.clone().x.to_string()}
                      y={top_left.clone().y.to_string()}
                      width={size.x.to_string()}
                      height={size.y.to_string()}
                      fill-opacity=0.1
                      stroke-opacity=0.5
                      style="fill:rgb(0,0,255);stroke-width:1;stroke:blue"/>
            </svg>
        }
    }
}

fn sort_rectangle_coordinates(first: DVec2, second: DVec2) -> (DVec2, DVec2) {
    let top_left = DVec2 {
        x: first.x.min(second.x),
        y: first.y.min(second.y),
    };
    let bottom_right = DVec2 {
        x: first.x.max(second.x),
        y: first.y.max(second.y),
    };
    (top_left, bottom_right)
}

fn rectangles_overlap(
    first_rect_one: DVec2,
    first_rect_two: DVec2,
    second_rect_one: DVec2,
    second_rect_two: DVec2,
) -> bool {
    let (top_left_one, bottom_right_one) =
        sort_rectangle_coordinates(first_rect_one, first_rect_two);
    let (top_left_two, bottom_right_two) =
        sort_rectangle_coordinates(second_rect_one, second_rect_two);
    if top_left_one.x > bottom_right_two.x || top_left_two.x > bottom_right_one.x {
        return false;
    }
    if bottom_right_one.y < top_left_two.y || bottom_right_two.y < top_left_one.y {
        return false;
    }
    true
}

impl Viewable<yew::Html> for State {
    type Callback = yew::Callback<crate::editor::board::Event>;

    fn view(&self, callback: Self::Callback) -> yew::Html {
        yew::html!{
            <>
                {self.draw_selection_rect()}
                {self.internal.html(callback)}
            </>
        }
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rectangle selection, start={}, end={}", self.start, self.end)
    }
}