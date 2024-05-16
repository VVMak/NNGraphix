mod arrow;
mod block;
mod coords;
mod graph;
mod message;
mod state;

use log::info;
use yew::prelude::*;
use std::collections::HashSet;

use super::tools;
use coords::Coords;
use graph::Graph;
use message::Msg;
use state::State;


#[derive(PartialEq, Properties)]
pub struct Props;

#[derive(Default)]
pub struct Board {
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
}

impl Component for Board {
    type Message = Msg;

    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Board::default()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onmousemove = ctx.link().callback(
            |e: MouseEvent| Msg::MouseMove(Coords { x: e.client_x(), y: e.client_y() })
        );
        let onkeydown = ctx.link().callback(Msg::KeyDown);
        let onmouseup = ctx.link().callback(|_: MouseEvent| Msg::MouseLeftUp);
        let onmousedown = ctx.link().callback(|_: MouseEvent| Msg::MouseLeftDownOutsideOfBlock);
        html!{
            <div tabindex="0"
            onkeydown={onkeydown}
            onmousemove={onmousemove}
            onmousedown={onmousedown}
            onmouseup={onmouseup}
            >
                <svg width="1920" height="1080">
                    { self.graph.html(ctx.link()) }
                </svg>
            </div>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::MouseMove(coords) => {
                let delta = coords - self.mouse_position.clone();
                self.mouse_position += delta.clone();
                match self.state {
                    State::DraggingSelection => {
                        for id in &self.selected {
                            self.graph.get_block(id).unwrap().upper_left += delta.clone();
                        }
                        true
                    }
                    _ => false
                }
            },
            Msg::MouseLeftUp => {
                self.set_state(State::Basic);
                false
            },
            Msg::MouseLeftDownOutsideOfBlock => {
                self.clear_selection();
                true
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
