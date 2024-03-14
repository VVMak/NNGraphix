mod arrow;
mod block;
mod coords;
mod message;
mod state;

use log::info;
use yew::prelude::*;
use std::{borrow::Borrow, collections::{HashMap, HashSet}};

use arrow::Arrow;
use block::Block;
use super::tools;
use coords::Coords;
use message::Msg;
use state::State;


#[derive(PartialEq, Properties)]
pub struct Props;

#[derive(Default)]
pub struct Board {
    arrow_id_gen: tools::IdGenerator,
    arrows: HashMap<tools::Id, Arrow>,
    block_id_gen: tools::IdGenerator,
    blocks: HashMap<tools::Id, Block>,
    selected: HashSet<tools::Id>,
    state: State,
    mouse_position: Coords,
}

impl Board {
    fn set_state(&mut self, new_state: state::State) {
        info!("Set state {:?}", new_state);
        self.state = new_state;
    }

    fn create_block_html(&self, block: &Block, ctx: &Context<Self>) -> Html {
        let id = block.id.clone();
        let onmousedown: Callback<MouseEvent> = ctx.link().callback(move |e: MouseEvent| {
            e.stop_immediate_propagation();
            Msg::MouseLeftDownBlock(id)
        });
        html! {
            <g
            onmousedown={onmousedown}
            >
            {block.get_rect_html()}
            </g>
        }
    }

    fn create_arrow_html(&self, arrow: &Arrow) -> Html {
        arrow.create_html(&self.blocks)
    }

    fn select_block(&mut self, block_id: tools::Id) {
        self.selected.insert(block_id);
        self.blocks.get_mut(&block_id).unwrap().select();
    }

    fn clear_selection(&mut self) {
        for block_id in &self.selected {
            self.blocks.get_mut(block_id).unwrap().unselect();
        }
        self.selected.clear();
    }

    fn create_arrow(&mut self, start_id: tools::Id, end_id: tools::Id) {
        let id = self.arrow_id_gen.next().unwrap();
        self.blocks.get_mut(&start_id).unwrap().add_next(end_id, id);
        self.blocks.get_mut(&end_id).unwrap().add_prev(start_id, id);
        self.arrows.insert(id, Arrow { id, start_id, end_id });
    }

    fn remove_arrow(&mut self, arrow_id: tools::Id) {
        let arrow = &self.arrows[&arrow_id];
        self.blocks.get_mut(&arrow.start_id).unwrap().remove_next(arrow.end_id);
        self.blocks.get_mut(&arrow.end_id).unwrap().remove_prev(arrow.start_id);
        self.arrows.remove(&arrow_id);
    }

    fn remove_arrows_with(&mut self, block_id: tools::Id) {
        let block = self.blocks[&block_id].clone();
        for next_arrow_id in block.arrows_nexts() {
            self.remove_arrow(next_arrow_id.to_owned());
        }
        for prev_arrow_id in block.arrows_prevs() {
            self.remove_arrow(prev_arrow_id.to_owned());
        }
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
        html!{
            <div tabindex="0"
            onkeydown={onkeydown}
            onmousemove={onmousemove}
            onmouseup={onmouseup}
            >
                <svg width="1920" height="1080">
                    { self.blocks.iter().map(|(_, block)| {
                        self.create_block_html(&block, ctx)
                    }).collect::<Html>()}
                    { self.arrows.iter().map(|(_, arrow)| {
                        self.create_arrow_html(arrow)
                    }).collect::<Html>()}
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
                            self.blocks.get_mut(id).unwrap().upper_left += delta.clone();
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
            Msg::MouseLeftDownBlock(id) => match self.state {
                State::ArrowCreation => {
                    self.set_state(State::Basic);
                    for start_id in self.selected.clone() {
                        self.create_arrow(start_id, id);
                    }
                    true
                }
                State::Basic => {
                    self.set_state(State::DraggingSelection);
                    self.clear_selection();
                    self.select_block(id);
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
                            let id = self.block_id_gen.next().unwrap();
                            self.blocks.insert(id, Block::new(id, self.mouse_position.clone()));
                            self.clear_selection();
                            true
                        }
                        "Delete" => {
                            for id in &self.selected {
                                self.remove_arrows_with(id.clone());
                                self.blocks.remove(&id);
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
