mod block_id;

pub use block_id::{BlockId, BlockIdGenerator};

#[derive(PartialEq, Clone, Debug)]
pub struct Block {
  pub id: BlockId,
  pub center: super::Coords,
}
