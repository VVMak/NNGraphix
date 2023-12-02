mod block;
mod message;

use log::info;
use yew::prelude::*;
use std::collections::{HashMap, HashSet};

use block::{Block, BlockId};
use message::Msg;

#[derive(PartialEq, Clone, Debug, Default)]
pub struct Coords {
    x: i32,
    y: i32,
}


#[derive(PartialEq, Properties)]
pub struct Props;

#[derive(Default)]
pub struct Board {
    block_id_gen: block::BlockIdGenerator,
    blocks: HashMap<BlockId, Block>,
    selected: HashSet<BlockId>,
    drugging: bool,
    mouse_position: Coords,
}

impl Board {
    fn create_block(&self, block: &Block, ctx: &Context<Self>) -> Html {
        let id = block.id.clone();
        let onmousedown: Callback<MouseEvent> = ctx.link().callback(move |e: MouseEvent| {
            e.stop_immediate_propagation();
            Msg::MouseLeftDownBlock(id)
        });
        html! {
            <g
            onmousedown={onmousedown}
            >
            <rect x={(block.center.x - 75).to_string()} y={(block.center.y - 75).to_string()} rx="20" ry="20" width="150" height="150"
            style="fill:red;stroke:black;stroke-width:5;opacity:0.5"/>
            </g>
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
        let onkeydown = ctx.link().callback(
            |e: KeyboardEvent| Msg::KeyDown(e.key())
        );
        let onmouseup = ctx.link().callback(|_: MouseEvent| Msg::MouseLeftUp);
        html!{
            <div tabindex="0"
            onkeydown={onkeydown}
            onmousemove={onmousemove}
            onmouseup={onmouseup}
            >
                <svg width="1920" height="1080">
                    { self.blocks.iter().map(|(_, block)| {
                        self.create_block(&block, ctx)
                    }).collect::<Html>()}
                </svg>
            </div>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::MouseMove(coords) => {
                self.mouse_position = coords;
                if self.drugging {
                    for id in &self.selected {
                        self.blocks.get_mut(id).unwrap().center = self.mouse_position.clone();
                    }
                };
                self.drugging
            },
            Msg::MouseLeftUp => {
                self.drugging = false;
                false
            },
            Msg::MouseLeftDownBlock(id) => {
                info!("Block click");
                self.drugging = true;
                self.selected.clear();
                self.selected.insert(id);
                false
            },
            Msg::KeyDown(key) => {
                if key == "n" {
                    let id = self.block_id_gen.next().unwrap();
                    self.blocks.insert(id, Block { id: id, center: self.mouse_position.clone() });
                    self.selected.clear();
                    true
                } else {
                    false
                }
            },
        }
    }
}
