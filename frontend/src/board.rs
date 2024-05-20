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
        self.board_size += Vector {x: scale_value / SCALING_SPEED, y: scale_value / SCALING_SPEED};
        let delta = self.mouse_position.clone() - (self.origin.clone() + get_viewport_size() / 2.0);
        self.origin += delta / DRAGGING_SPEED;
    }

}

impl Component for Board {
    type Message = Event;

    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Board {
            board_size: Vector {x: BASE_BOARD_SIZE, y: BASE_BOARD_SIZE},
            origin: Vector::default(),
            graph: Graph::default(),
            selected: HashSet::<block::Id>::default(),
            state: State::default(),
            mouse_position: Vector::default()
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onmousemove = ctx.link().callback(
|e: MouseEvent| Event::MouseMove(Vector { x: e.client_x() as f64, y: e.client_y() as f64})
        );
        let onkeydown = ctx.link().callback(Event::KeyDown);
        let onmouseup = ctx.link().callback(|e: MouseEvent| Event::MouseLeftUp(e));
        let onmousedown = ctx.link().callback(|e: MouseEvent| Event::MouseDownBoard(e));
        let onclick = ctx.link().callback(|e: MouseEvent| Event::MouseClick(e));
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
            onclick={onclick}
            onmouseup={onmouseup}
            onwheel={onwheel}
            >
                <svg
                width = {BASE_BOARD_SIZE.to_string()}
                height = {BASE_BOARD_SIZE.to_string()}
                viewBox={view_box_str} 
                xmlns="http://www.w3.org/2000/svg">
                    { self.graph.html(ctx.link()) }
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
                    }
                    _ => false
                }
            },
            Event::MouseLeftUp(e) => match e.button() {
                0 => false, // left button
                _ => {
                    self.set_state(State::Basic);
                    true
                }
            },
            Event::MouseWheel(e) => {
                self.scale_board(e.delta_y());
                true
            },
            Event::MouseClick(e) => match e.button()  {
                0 => { // left button click
                    match self.state {
                        State::DraggingSelection => {
                            self.set_state(State::Basic);
                            false
                        }
                        _ => {
                            self.clear_selection();
                            true
                        }
                    }
                },
                _ => false
            }
            Event::MouseDownBoard(e) => match e.button()  {
                1 => { // middle button click
                    log::info!("Holding middle button");
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
                    if !e.ctrl_key() {
                        self.clear_selection();
                    }
                    if self.selected.contains(&id) {
                        self.deselect_block(&id);
                    } else {
                        self.select_block(id);
                    }
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
