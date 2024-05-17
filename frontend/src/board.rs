mod arrow;
mod block;
mod coords;
mod graph;
mod message;
mod state;

use log::info;
use yew::prelude::*;
use std::collections::HashSet;
use derivative::Derivative;

use super::tools;
use coords::Coords;
use graph::Graph;
use message::Msg;
use state::State;


#[derive(PartialEq, Properties)]
pub struct Props;

#[derive(Derivative)]
#[derivative(Default)]
pub struct Board {
    #[derivative(Default(value = "4000.0"))]
    width: f64,
    #[derivative(Default(value = "4000.0"))]
    height: f64,
    #[derivative(Default(value = "0.0"))]
    origin_x: f64,
    #[derivative(Default(value = "0.0"))]
    origin_y: f64,
    _window_width: f64,
    _window_height: f64,
    graph: Graph,
    selected: HashSet<block::Id>,
    state: State,
    mouse_position: Coords,
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
        log::info!("Board got scaled by {}", scale_value); // new_scale_value = 125.0 or -125.0 (250.0 maybe) depending on wheel direction
        self.width  = self.width + scale_value / 5.0; // you can change 5.0 if you want; the higher the number the slower the board scales
        self.height = self.width;
        let delta_x = self.mouse_position.x - (self.origin_x + 1920.0 / 2.0); // Here we need to use window size somehow TODO
        let delta_y = self.mouse_position.y - (self.origin_y + 1080.0 / 2.0); // Here we need to use window size somehow TODO
        self.origin_x = self.origin_x + delta_x / 20.0;
        self.origin_y = self.origin_y + delta_y / 20.0;
        log::info!("Origin position {x}, {y}", x=self.origin_x, y=self.origin_y);
    }

}

impl Component for Board {
    type Message = Msg;

    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Board::default()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onmousemove = ctx.link().callback(
|e: MouseEvent| Msg::MouseMove(Coords { x: e.client_x() as f64, y: e.client_y() as f64})
        );
        let onkeydown = ctx.link().callback(Msg::KeyDown);
        let onmouseup = ctx.link().callback(|_: MouseEvent| Msg::MouseLeftUp);
        let onmousedown = ctx.link().callback(|e: MouseEvent| Msg::MouseLeftDownOutsideOfBlock(e));
        let onwheel = ctx.link().callback(|e: WheelEvent| Msg::MouseWheelScale(e));
        let view_box_str = format!("{origin_x}, {origin_y}, {width}, {height}",
                                            origin_x=self.origin_x,
                                            origin_y=self.origin_y,
                                            width=self.width,
                                            height=self.height);
        html!{
            <div tabindex="0"
            onkeydown={onkeydown}
            onmousemove={onmousemove}
            onmousedown={onmousedown}
            onmouseup={onmouseup}
            onwheel={onwheel}
            >
                <svg width = "4000.0" height = "4000.0" viewBox={view_box_str} xmlns="http://www.w3.org/2000/svg">
                    { self.graph.html(ctx.link()) }
                </svg>
            </div>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::MouseMove(coords) => {
                let delta = Coords {
                    x:  coords.x * (self.width / 4000.0) - self.mouse_position.clone().x + self.origin_x,
                    y:  coords.y * (self.width / 4000.0) - self.mouse_position.clone().y + self.origin_y
                };
                self.mouse_position += delta.clone();
                match self.state {
                    State::DraggingSelection => {
                        for id in &self.selected {
                            self.graph.get_block(id).unwrap().upper_left += delta.clone();
                        }
                        true
                    },
                    State::DraggingBoard => {
                        self.origin_x -= delta.clone().x;
                        self.origin_y -= delta.clone().y;
                        log::info!("Current mouse position {x}, {y}", x=self.mouse_position.clone().x, y=self.mouse_position.y);
                        log::info!("Origin position {x}, {y}", x=self.origin_x, y=self.origin_y);
                        true
                    }
                    _ => false
                }
            },
            Msg::MouseLeftUp => {
                self.set_state(State::Basic);
                false
            },
            Msg::MouseWheelScale(e) => {
                self.scale_board(e.delta_y());
                true
            },
            Msg::MouseLeftDownOutsideOfBlock(e) => match e.button()  {
                0 => { // left button click
                    self.clear_selection();
                    true
                },
                1 => { // middle button click
                    log::info!("Clicked middle button");
                    self.set_state(State::DraggingBoard);
                    true
                },
                _another_button => false
            },
            Msg::MouseLeftDownBlock(e, id) => match self.state {
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
            Msg::KeyDown(event) => {
                match self.state {
                    State::Basic => match event.key().as_str() {
                        "a" => {
                            self.set_state(State::ArrowCreation);
                            false
                        }
                        "n" => {
                            self.graph.create_block(self.mouse_position.clone(), Coords {x: self.origin_x,  y: self.origin_y});
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
