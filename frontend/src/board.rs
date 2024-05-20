mod arrow;
mod block;
mod vector;
mod graph;
mod event;
mod state;

use log::info;
use yew::prelude::*;
use std::collections::HashSet;
use web_sys;
use derivative::Derivative;

use self::block::Block;

use super::tools;
use vector::Vector;
use graph::Graph;
use event::Event;
use state::State;

const BASE_BOARD_SIZE: f64 = 4000.0;
const SCALING_SPEED: f64 = 5.0;
const DRAGGING_SPEED: f64 = 20.0;


pub fn get_viewport_size() -> Vector {
    Vector {
        x :
        web_sys::window()
        .expect("There should be a window")
        .inner_width()
        .expect("The window should have Some width")
        .as_f64()
        .expect("The width should be a number"),
        y: 
        web_sys::window()
        .expect("There should be a window")
        .inner_height()
        .expect("The window should have Some height")
        .as_f64()
        .expect("The width should be a number")
    }
}


#[derive(PartialEq, Properties)]
pub struct Props;

#[derive(Derivative)]
#[derivative(Default)]
pub struct Board {
    board_size: Vector,
    origin: Vector,
    graph: Graph,
    selected: HashSet<block::Id>,
    state: State,
    mouse_position: Vector,
    start_mouse_position: Vector, // position where rectangle selection started
}

pub fn rectangles_overlap(first_rect_one: Vector, first_rect_two: Vector, second_rect_one: Vector, second_rect_two: Vector) -> bool {
    let top_left_one = Vector{x: first_rect_one.x.min(first_rect_two.x), y: first_rect_one.y.min(first_rect_two.y)};
    let bottom_right_one = Vector{x: first_rect_one.x.max(first_rect_two.x), y: first_rect_one.y.max(first_rect_two.y)};
    let top_left_two = Vector{x: second_rect_one.x.min(second_rect_two.x), y: second_rect_one.y.min(second_rect_two.y)};
    let bottom_right_two = Vector{x: second_rect_one.x.max(second_rect_two.x), y: second_rect_one.y.max(second_rect_two.y)};
    if top_left_one.x > bottom_right_two.x || top_left_two.x > bottom_right_one.x {return false;}
    if bottom_right_one.y < top_left_two.y || bottom_right_two.y < top_left_one.y {return false;}
    return true;
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

    fn scale_board(&mut self, scale_value: f64) {
        let screen_space_mouse_position = (self.mouse_position.clone() - self.origin.clone()) / self.board_size.clone() * BASE_BOARD_SIZE;
        self.board_size += Vector {x: scale_value / SCALING_SPEED, y: scale_value / SCALING_SPEED};
        let delta = self.mouse_position.clone() - (self.origin.clone() + get_viewport_size() / 2.0);
        self.origin += delta.clone() / DRAGGING_SPEED;
        self.mouse_position = screen_space_mouse_position.clone() * self.board_size.clone() / BASE_BOARD_SIZE + self.origin.clone();
    }

}

impl Component for Board {
    type Message = Event;

    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Board {
            board_size: Vector {x: BASE_BOARD_SIZE, y: BASE_BOARD_SIZE},
            origin: Vector {x: BASE_BOARD_SIZE / 2.0, y: BASE_BOARD_SIZE / 2.0},
            graph: Graph::default(),
            selected: HashSet::<block::Id>::default(),
            state: State::default(),
            mouse_position: Vector::default(),
            start_mouse_position: Vector::default()
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onmousemove = ctx.link().callback(
|e: MouseEvent| Event::MouseMove(Vector { x: e.client_x() as f64, y: e.client_y() as f64})
        );
        let onkeydown = ctx.link().callback(Event::KeyDown);
        let onmouseup = ctx.link().callback(|e: MouseEvent| Event::MouseUp(e));
        let onmousedown = ctx.link().callback(|e: MouseEvent| Event::MouseDownBoard(e));
        let onwheel = ctx.link().callback(|e: WheelEvent| Event::MouseWheel(e));
        let view_box_str = format!("{origin_x}, {origin_y}, {width}, {height}",
                                            origin_x=self.origin.x,
                                            origin_y=self.origin.y,
                                            width=self.board_size.x,
                                            height=self.board_size.y);
        html!{
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
                    { self.graph.html(ctx.link()) }
                    if self.state == State::RectangleSelection {
                        { self.graph.draw_selection_rect(self.start_mouse_position.clone(), self.mouse_position.clone())}
                    }
                </svg>
            </div>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, event: Self::Message) -> bool {
        match event {
            Event::MouseMove(vector) => {
                let delta = vector.clone() * self.board_size.clone() / BASE_BOARD_SIZE + self.origin.clone() - self.mouse_position.clone();
                self.mouse_position += delta.clone();
                match self.state {
                    State::DraggingSelection => {
                        for id in &self.selected {
                            self.graph.get_block(id).unwrap().upper_left += delta.clone();
                        }
                        true
                    },
                    State::DraggingBoard => {
                        self.origin -= delta.clone();
                        self.mouse_position -= delta.clone();
                        true
                    },
                    State::RectangleSelection => {
                        for block in self.graph.get_blocks() {
                            let top_left = block.1.borrow().upper_left.clone();
                            let bottom_right = Vector {x: top_left.x.clone() + Block::get_block_width(),
                                y: top_left.y.clone() + Block::get_block_height()};
                            if rectangles_overlap(top_left,
                                                  bottom_right,
                                            self.start_mouse_position.clone(),
                                            self.mouse_position.clone()) {
                                                self.select_block(block.0);
                                            }
                            else {
                                if block.1.borrow().is_selected() {
                                    self.deselect_block(&block.0);
                                }
                            }
                        }
                        true
                    },
                    _ => false
                }
            },
            Event::MouseUp(e) => match e.button() {
                _ => {
                    self.set_state(State::Basic);
                    true
                }
            },
            Event::MouseWheel(e) => {
                self.scale_board(e.delta_y());
                true
            },
            Event::MouseDownBoard(e) => match e.button()  {
                0 => { // left button click
                    self.start_mouse_position = Vector{x: e.client_x() as f64, y: e.client_y() as f64} * self.board_size.clone() / BASE_BOARD_SIZE + self.origin.clone();
                    self.clear_selection();
                    self.set_state(State::RectangleSelection);
                    true
                },
                1 => { // middle button click
                    self.set_state(State::DraggingBoard);
                    true
                },
                _ => false
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
                    self.select_block(id);
                    true
                }
                _ => false
            }
            Event::KeyDown(event) => {
                match self.state {
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
                        _ => false
                    }
                    State::ArrowCreation => match event.key().as_str() {
                        "Escape" => {
                            self.set_state(State::Basic);
                            false
                        }
                        _ => false
                    }
                    _ => false            
                }
            },
        }
    }
}
