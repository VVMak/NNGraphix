mod block;
mod coords;
mod message;

use yew::prelude::*;
use std::collections::{HashMap, HashSet};

use block::{Block, BlockId};
use coords::Coords;
use message::Msg;


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
            <rect x={block.upper_left.x.to_string()} y={block.upper_left.y.to_string()}
            rx="20" ry="20" width={Block::def_width().to_string()} height={Block::def_height().to_string()}
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
                let delta = coords - self.mouse_position.clone();
                self.mouse_position += delta.clone();
                if self.drugging {
                    for id in &self.selected {
                        self.blocks.get_mut(id).unwrap().upper_left += delta.clone();
                    }
                };
                self.drugging
            },
            Msg::MouseLeftUp => {
                self.drugging = false;
                false
            },
            Msg::MouseLeftDownBlock(id) => {
                self.drugging = true;
                self.selected.clear();
                self.selected.insert(id);
                false
            },
            Msg::KeyDown(key) => {
                if key == "n" {
                    let id = self.block_id_gen.next().unwrap();
                    self.blocks.insert(id, Block { id: id, upper_left: self.mouse_position.clone() - Coords { x: Block::def_width() / 2, y: Block::def_height() / 2 } });
                    self.selected.clear();
                    true
                } else {
                    false
                }
            },
        }
    }
}
