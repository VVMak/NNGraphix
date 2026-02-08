use glam::DVec2;
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

/// Координаты в пространстве доски
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct BoardCoords(DVec2);

impl BoardCoords {
    pub const fn new(x: f64, y: f64) -> Self {
        Self(DVec2::new(x, y))
    }
    pub fn x(&self) -> f64 {
        self.0.x
    }
    pub fn y(&self) -> f64 {
        self.0.y
    }
}
impl Neg for BoardCoords {
    type Output = Self;

    fn neg(self) -> Self::Output {
        BoardCoords(-self.0)
    }
}
impl Add for BoardCoords {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}
impl AddAssign for BoardCoords {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}
impl Sub for BoardCoords {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}
impl Mul<DVec2> for BoardCoords {
    type Output = AppCoords;

    fn mul(self, rhs: DVec2) -> Self::Output {
        AppCoords(self.0 * rhs)
    }
}
impl Div<f64> for BoardCoords {
    type Output = BoardCoords;

    fn div(self, rhs: f64) -> Self::Output {
        BoardCoords(self.0 / rhs)
    }
}

/// Координаты в пространстве окна приложения
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct AppCoords(DVec2);

impl AppCoords {
    pub fn new(x: f64, y: f64) -> Self {
        Self(DVec2::new(x, y))
    }
    #[allow(unused)]
    pub fn x(&self) -> f64 {
        self.0.x
    }
    #[allow(unused)]
    pub fn y(&self) -> f64 {
        self.0.y
    }
}
impl Neg for AppCoords {
    type Output = Self;

    fn neg(self) -> Self::Output {
        AppCoords(-self.0)
    }
}
impl Add for AppCoords {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}
impl Sub for AppCoords {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}
impl AddAssign for AppCoords {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}
impl Div<DVec2> for AppCoords {
    type Output = BoardCoords;

    fn div(self, rhs: DVec2) -> Self::Output {
        BoardCoords(self.0 / rhs)
    }
}
