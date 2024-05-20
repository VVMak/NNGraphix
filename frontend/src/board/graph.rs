use super::arrow;
use arrow::Arrow;
use super::block;
use block::Block;
use super::vector::Vector;
use super::event::Event;
use super::tools;
use std::{cell::{Ref, RefCell, RefMut}, collections::HashMap};
use yew::{html, Html, html::Scope, prelude::*};

#[derive(Default)]
pub struct Graph {
    arrow_id_gen: tools::IdGenerator,
    arrows: HashMap<arrow::Id, Arrow>,
    block_id_gen: tools::IdGenerator,
    blocks: HashMap<block::Id, RefCell<Block>>,
}

impl Graph {
    fn create_block_html(&self, block: Ref<'_, Block>, scope: &Scope<super::Board>) -> Html {
        let id = block.id.clone();
        let onmousedown: Callback<MouseEvent> = scope.callback(move |e: MouseEvent| {
            e.stop_immediate_propagation();
            Event::MouseDownBlock(e, id)
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
    pub fn html(&self, scope: &Scope<super::Board>) -> Html {
        html!{
            <>
                <defs>
                <pattern id="smallGrid" width="8" height="8" patternUnits="userSpaceOnUse">
                <path d="M 8 0 L 0 0 0 8" fill="none" stroke="gray" stroke-width="0.5"/>
                </pattern>
                <pattern id="grid" width="80" height="80" patternUnits="userSpaceOnUse">
                <rect width="80" height="80" fill="url(#smallGrid)"/>
                <path d="M 80 0 L 0 0 0 80" fill="none" stroke="gray" stroke-width="1"/>
                </pattern>
                </defs> 
                <rect width="100%" height="100%" x="-1000" y="-1000" fill="url(#grid)" />
                { self.arrows.iter().map(|(_, arrow)| {
                    self.create_arrow_html(arrow)
                }).collect::<Html>()}
                { self.blocks.iter().map(|(_, block)| {
                    self.create_block_html(block.borrow(), scope)
                }).collect::<Html>()}
            </>
        }
    }
    
    pub fn create_block(&mut self, vector: Vector) -> tools::Id {
        let id = self.block_id_gen.next().unwrap();
        self.blocks.insert(id, Block::new(id, vector).into());
        id
    }
    pub fn remove_block(&mut self, id: &block::Id) {
        self.remove_arrows_for_block(id);
        self.blocks.remove(id);
    }
    pub fn create_arrow(&mut self, from: block::Id, to: block::Id) {
        let id = self.arrow_id_gen.next().unwrap();
        self.arrows.insert(id, Arrow{id, start_id: from, end_id: to});
        self.blocks.get(&from).map(|x| x.borrow_mut().add_next(to, id));
        self.blocks.get(&to).map(|x| x.borrow_mut().add_prev(from, id));
    }
    fn remove_arrows_for_block(&mut self, id: &block::Id) {
        let block_opt = self.blocks.get(id);
        if block_opt.is_none() { return }
        for (block_id, arrow_id) in block_opt.unwrap().borrow_mut().next.drain() {
            self.get_block(&block_id).and_then(|mut x| x.remove_prev(id));
            self.arrows.remove(&arrow_id);
        }
        for (block_id, arrow_id) in block_opt.unwrap().borrow_mut().prev.drain() {
            self.get_block(&block_id).and_then(|mut x| x.remove_next(id));
            self.arrows.remove(&arrow_id);
        }
    }

    pub fn get_block(&self, id: &block::Id) -> Option<RefMut<'_, Block>> {
        self.blocks.get(id).map(|x| x.borrow_mut())
    }
}