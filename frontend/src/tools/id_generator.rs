pub type Id = i64;

// for now it's stupid and now async- or thread- safe
#[derive(Default, Clone, Copy, Debug)]
pub struct IdGen {
    last: Id,
}

impl Iterator for IdGen {
    type Item = Id;

    fn next(&mut self) -> Option<Self::Item> {
        self.last += 1;
        Some(self.last)
    }
}
