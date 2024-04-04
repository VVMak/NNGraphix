pub type Id = i64;

// for now it's stupid and now async- or thread- safe
#[derive(Default)]
pub struct IdGenerator {
    last: Id,
}

impl Iterator for IdGenerator {
    type Item = Id;

    fn next(&mut self) -> Option<Self::Item> {
        self.last += 1;
        Some(self.last)
    }
}
