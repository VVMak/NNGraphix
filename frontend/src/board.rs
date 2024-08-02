mod arrow;
mod block;
mod event;
mod graph;
mod state;
mod vector;

use log::info;
use std::collections::HashSet;
use web_sys;
use yew::prelude::*;

use self::block::Block;

use super::tools;
use event::Event;
use graph::Graph;
use state::State;
use vector::Vector;

const BASE_BOARD_SIZE: f64 = 4000.0;
const GRID_SIZE: f64 = 80.0;
const SCALING_SPEED: f64 = 5.0;
const DRAGGING_SPEED: f64 = 20.0;

pub fn get_viewport_size() -> Vector {
    Vector {
        x: web_sys::window()
            .expect("There should be a window")
            .inner_width()
            .expect("The window should have Some width")
            .as_f64()
            .expect("The width should be a number"),
        y: web_sys::window()
            .expect("There should be a window")
            .inner_height()
            .expect("The window should have Some height")
            .as_f64()
            .expect("The height should be a number"),
    }
}

pub fn sort_rectangle_coordinates(first: Vector, second: Vector) -> (Vector, Vector) {
    let top_left = Vector {
        x: first.x.min(second.x),
        y: first.y.min(second.y),
    };
    let bottom_right = Vector {
        x: first.x.max(second.x),
        y: first.y.max(second.y),
    };
    return (top_left, bottom_right);
}

pub fn rectangles_overlap(
    first_rect_one: Vector,
    first_rect_two: Vector,
    second_rect_one: Vector,
    second_rect_two: Vector,
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
    return true;
}

#[derive(PartialEq, Properties)]
pub struct Props;

pub struct Board {
    board_size: Vector,
    origin: Vector,
    graph: Graph,
    selected: HashSet<block::Id>,
    state: State,
    mouse_position: Vector,
}

impl Board {
    fn set_state(&mut self, new_state: state::State) {
        info!("Set state {:?}", new_state);
        self.state = new_state;
    }

    fn select_block(&mut self, block_id: block::Id) {
        self.selected.insert(block_id);
        self.graph.get_block(&block_id).unwrap().select();
    }

    fn deselect_block(&mut self, block_id: &block::Id) {
        self.selected.remove(block_id);
        self.graph.get_block(block_id).unwrap().deselect();
    }

    fn clear_selection(&mut self) {
        for block_id in &self.selected {
            self.graph.get_block(block_id).unwrap().deselect();
        }
        self.selected.clear();
    }

    fn screen_space_to_world_space(&mut self, position: Vector) -> Vector {
        return position * self.board_size.clone() / BASE_BOARD_SIZE + self.origin.clone();
    }

    fn scale_board(&mut self, scale_value: f64) {
        self.board_size += Vector {
            x: scale_value / SCALING_SPEED,
            y: scale_value / SCALING_SPEED,
        };
        let delta = self.mouse_position.clone() - (self.origin.clone() + get_viewport_size() / 2.0);
        self.origin += delta.clone() / DRAGGING_SPEED;
        self.mouse_position += delta.clone() / DRAGGING_SPEED;
    }
    pub fn draw_selection_rect(&self, corner_one: Vector, corner_two: Vector) -> Html {
        let (top_left, bottom_right) = sort_rectangle_coordinates(corner_one, corner_two);
        let size = bottom_right.clone() - top_left.clone();
        html! {
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
    pub fn draw_grid() -> Html {
        html! {
        <>
        <defs>
            <pattern id="smallGrid" width={(GRID_SIZE / 10.0).to_string()} height={(GRID_SIZE / 10.0).to_string()} patternUnits="userSpaceOnUse">
            <path d={format!("M {small_grid_size} 0 L 0 0 0 {small_grid_size}", small_grid_size=GRID_SIZE / 10.0)} fill="none" stroke="gray" stroke-width="0.5"/>
            </pattern>
            <pattern id="grid" width={GRID_SIZE.to_string()} height={GRID_SIZE.to_string()} patternUnits="userSpaceOnUse">
            <rect width={GRID_SIZE.to_string()} height={GRID_SIZE.to_string()} fill="url(#smallGrid)"/>
            <path d={format!("M {grid_size} 0 L 0 0 0 {grid_size}", grid_size=GRID_SIZE)} fill="none" stroke="gray" stroke-width="1"/>
            </pattern>
            </defs>
            <rect width="100%" height="100%" x="0" y="0" fill="url(#grid)" />
        </>
        }
    }
}

impl Component for Board {
    type Message = Event;

    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Board {
            board_size: Vector {
                x: BASE_BOARD_SIZE,
                y: BASE_BOARD_SIZE,
            },
            origin: Vector {
                x: BASE_BOARD_SIZE / 2.0,
                y: BASE_BOARD_SIZE / 2.0,
            },
            graph: Graph::default(),
            selected: HashSet::<block::Id>::default(),
            state: State::default(),
            mouse_position: Vector::default(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onmousemove = ctx.link().callback(|e: MouseEvent| {
            Event::MouseMove(Vector {
                x: e.client_x() as f64,
                y: e.client_y() as f64,
            })
        });
        let onkeydown = ctx.link().callback(Event::KeyDown);
        let onmouseup = ctx.link().callback(|e: MouseEvent| Event::MouseUp(e));
        let onmousedown = ctx
            .link()
            .callback(|e: MouseEvent| Event::MouseDownBoard(e));
        let onwheel = ctx.link().callback(|e: WheelEvent| Event::MouseWheel(e));
        let view_box_str = format!(
            "{origin_x}, {origin_y}, {width}, {height}",
            origin_x = self.origin.x,
            origin_y = self.origin.y,
            width = self.board_size.x,
            height = self.board_size.y
        );
        html! {
            <div tabindex="0"
            onkeydown={onkeydown}
            onmousemove={onmousemove}
            onmousedown={onmousedown}
            onmouseup={onmouseup}
            onwheel={onwheel}
            >
                <svg
                width = {BASE_BOARD_SIZE.to_string()}
                height = {BASE_BOARD_SIZE.to_string()}
                viewBox={view_box_str}
                xmlns="http://www.w3.org/2000/svg">
                    { Board::draw_grid() }
                    { self.graph.html(ctx.link()) }
                    if let State::RectangleSelection(start) = &self.state {
                        { self.draw_selection_rect(start.clone(), self.mouse_position.clone())}
                    }
                </svg>
            </div>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, event: Self::Message) -> bool {
        match event {
            Event::MouseMove(vector) => {
                let delta = self.screen_space_to_world_space(vector) - self.mouse_position.clone();
                self.mouse_position += delta.clone();
                match self.state.clone() {
                    State::DraggingSelection => {
                        for id in &self.selected {
                            self.graph.get_block(id).unwrap().upper_left += delta.clone();
                        }
                        true
                    }
                    State::DraggingBoard => {
                        self.origin -= delta.clone();
                        self.mouse_position -= delta.clone();
                        true
                    }
                    State::RectangleSelection(start) => {
                        self.clear_selection();
                        for (id, block) in self.graph.get_blocks() {
                            let top_left = block.borrow().upper_left.clone();
                            let bottom_right = top_left.clone() + Block::get_block_size();
                            if rectangles_overlap(
                                top_left,
                                bottom_right,
                                start.clone(),
                                self.mouse_position.clone(),
                            ) {
                                self.select_block(id);
                            }
                        }
                        true
                    }
                    _ => false,
                }
            }
            Event::MouseUp(e) => match e.button() {
                _ => {
                    self.set_state(State::Basic);
                    true
                }
            },
            Event::MouseWheel(e) => {
                self.scale_board(e.delta_y());
                true
            }
            Event::MouseDownBoard(e) => match e.button() {
                0 => {
                    // left button click
                    self.clear_selection();
                    let start_mouse_position = self.screen_space_to_world_space(Vector {
                        x: e.client_x() as f64,
                        y: e.client_y() as f64,
                    });
                    self.set_state(State::RectangleSelection(start_mouse_position));
                    true
                }
                1 => {
                    // middle button click
                    self.set_state(State::DraggingBoard);
                    true
                }
                _ => false,
            },
            Event::MouseDownBlock(e, id) => match self.state {
                State::ArrowCreation => {
                    self.set_state(State::Basic);
                    for start_id in self.selected.clone() {
                        self.graph.create_arrow(start_id, id);
                    }
                    true
                }
                State::Basic => {
                    self.set_state(State::DraggingSelection);
                    if !e.ctrl_key() && !self.selected.contains(&id) {
                        self.clear_selection();
                    }
                    if e.ctrl_key() && self.selected.contains(&id) {
                        self.deselect_block(&id);
                    } else {
                        self.select_block(id);
                    }
                    true
                }
                _ => false,
            },
            Event::KeyDown(event) => match self.state {
                State::Basic => match event.key().as_str() {
                    "a" => {
                        self.set_state(State::ArrowCreation);
                        false
                    }
                    "n" => {
                        self.graph.create_block(self.mouse_position.clone());
                        self.clear_selection();
                        true
                    }
                    "Delete" => {
                        for id in &self.selected {
                            self.graph.remove_block(id);
                        }
                        self.selected.clear();
                        true
                    }
                    _ => false,
                },
                State::ArrowCreation => match event.key().as_str() {
                    "Escape" => {
                        self.set_state(State::Basic);
                        false
                    }
                    _ => false,
                },
                _ => false,
            },
        }
    }
}
