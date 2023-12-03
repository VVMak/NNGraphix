mod block_id;

pub use block_id::{BlockId, BlockIdGenerator};

use super::Coords;

#[derive(PartialEq, Clone, Debug)]
pub struct Block {
    pub id: BlockId,
    pub upper_left: Coords,
}

impl Block {
    pub fn def_width() -> i32 { 150 }
    pub fn def_height() -> i32 { 150 }
}
