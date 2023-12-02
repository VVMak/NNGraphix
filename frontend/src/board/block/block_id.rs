pub type BlockId = i64;

// for now it's stupid and now async- or thread- safe
#[derive(Default)]
pub struct BlockIdGenerator {
    last: BlockId,
}

impl Iterator for BlockIdGenerator {
    type Item = BlockId;

    fn next(&mut self) -> Option<Self::Item> {
       self.last += 1;
       Some(self.last)
    }
}
