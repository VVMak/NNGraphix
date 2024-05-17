#[derive(PartialEq, Clone, Debug, Default)]
pub struct Coords {
    pub x: f64,
    pub y: f64,
}

// maybe need to add implementations with references (e.g. AddAssign<&Self>)

impl std::ops::AddAssign<Self> for Coords {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::SubAssign<Self> for Coords {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl std::ops::Add<Self> for Coords {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl std::ops::Sub<Self> for Coords {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}
