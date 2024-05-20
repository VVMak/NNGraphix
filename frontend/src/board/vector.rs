#[derive(PartialEq, Clone, Debug, Default)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

// maybe need to add implementations with references (e.g. AddAssign<&Self>)

impl std::ops::AddAssign<Self> for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::SubAssign<Self> for Vector {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl std::ops::MulAssign<Self> for Vector {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl std::ops::MulAssign<f64> for Vector {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl std::ops::DivAssign<Self> for Vector {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl std::ops::DivAssign<f64> for Vector {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl std::ops::Add<Self> for Vector {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl std::ops::Sub<Self> for Vector {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl std::ops::Mul<Self> for Vector {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self *= rhs;
        self
    }
}

impl std::ops::Mul<f64> for Vector {
    type Output = Self;

    fn mul(mut self, rhs: f64) -> Self::Output {
        self *= rhs;
        self
    }
}

impl std::ops::Div<Self> for Vector {
    type Output = Self;

    fn div(mut self, rhs: Self) -> Self::Output {
        self /= rhs;
        self
    }
}

impl std::ops::Div<f64> for Vector {
    type Output = Self;

    fn div(mut self, rhs: f64) -> Self::Output {
        self /= rhs;
        self
    }
}