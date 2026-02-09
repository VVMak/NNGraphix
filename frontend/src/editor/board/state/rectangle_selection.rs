use crate::editor::board::block::state::StateInterface;
use crate::editor::types::BoardCoords;
use crate::utils::viewable::Viewable;

use super::states::*;

#[derive(Debug, PartialEq, Clone)]
pub struct State {
    internal: internal::State,
    start: BoardCoords,
    end: BoardCoords,
}

impl State {
    pub fn from(internal: internal::State, start: BoardCoords, end: BoardCoords) -> Self {
        Self {
            internal,
            start,
            end,
        }
    }

    pub fn to_states_enum(self) -> super::State {
        super::State::RectangleSelection(self)
    }

    pub fn move_end(mut self, new_value: BoardCoords) -> Self {
        self.end = new_value;
        self
    }

    pub fn finish(mut self) -> basic::State {
        self.internal
            .iter_blocks()
            .filter(|block| {
                rectangles_overlap(self.start, self.end, block.top_left(), block.bottom_right())
            })
            .for_each(|mut block| block.set_selected(true));
        basic::State::from(self.internal)
    }

    pub fn draw_selection_rect(&self) -> yew::Html {
        let (top_left, bottom_right) = sort_rectangle_coordinates(self.start, self.end);
        let size = bottom_right.clone() - top_left.clone();
        yew::html! {
            <g>
                <rect x={top_left.clone().x().to_string()}
                      y={top_left.clone().y().to_string()}
                      width={size.x().to_string()}
                      height={size.y().to_string()}
                      fill-opacity=0.1
                      stroke-opacity=0.5
                      style="fill:rgb(0,0,255);stroke-width:1;stroke:blue"/>
            </g>
        }
    }
}

fn sort_rectangle_coordinates(
    first: BoardCoords,
    second: BoardCoords,
) -> (BoardCoords, BoardCoords) {
    let top_left = BoardCoords::new(first.x().min(second.x()), first.y().min(second.y()));
    let bottom_right = BoardCoords::new(first.x().max(second.x()), first.y().max(second.y()));
    (top_left, bottom_right)
}

fn rectangles_overlap(
    first_rect_one: BoardCoords,
    first_rect_two: BoardCoords,
    second_rect_one: BoardCoords,
    second_rect_two: BoardCoords,
) -> bool {
    let (top_left_one, bottom_right_one) =
        sort_rectangle_coordinates(first_rect_one, first_rect_two);
    let (top_left_two, bottom_right_two) =
        sort_rectangle_coordinates(second_rect_one, second_rect_two);
    if top_left_one.x() > bottom_right_two.x() || top_left_two.x() > bottom_right_one.x() {
        return false;
    }
    if bottom_right_one.y() < top_left_two.y() || bottom_right_two.y() < top_left_one.y() {
        return false;
    }
    true
}

impl Viewable<yew::Html> for State {
    type Callback = yew::Callback<crate::editor::board::Event>;

    fn view(&self, callback: Self::Callback) -> yew::Html {
        yew::html! {
            <>
                {self.draw_selection_rect()}
                {self.internal.html(callback)}
            </>
        }
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Rectangle selection, start_x={}, start_y={}, end_x={}, end_y={}",
            self.start.x(),
            self.start.y(),
            self.end.x(),
            self.end.y()
        )
    }
}
