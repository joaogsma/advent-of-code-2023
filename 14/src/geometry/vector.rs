use std::hash::{Hash, Hasher};
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

use crate::geometry::truncate;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Vector(pub f32, pub f32);

impl Vector {
    pub const UP: Vector = Vector(0.0, 1.0);
    pub const DOWN: Vector = Vector(0.0, -1.0);
    pub const RIGHT: Vector = Vector(1.0, 0.0);
    pub const LEFT: Vector = Vector(-1.0, 0.0);

    pub fn inverse(&self) -> Vector { Vector(-self.0, -self.1) }
}

impl Add<Vector> for Vector {
    type Output = Vector;
    fn add(self, rhs: Vector) -> Vector { Vector(self.0 + rhs.0, self.1 + rhs.1) }
}

impl AddAssign<Vector> for Vector {
    fn add_assign(&mut self, rhs: Vector) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;
    fn sub(self, rhs: Vector) -> Vector { self + rhs.inverse() }
}

impl SubAssign<Vector> for Vector {
    fn sub_assign(&mut self, rhs: Vector) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

impl Mul<f32> for Vector {
    type Output = Vector;
    fn mul(self, rhs: f32) -> Vector { Vector(self.0 * rhs, self.1 * rhs) }
}

impl MulAssign<f32> for Vector {
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= rhs;
        self.1 *= rhs;
    }
}

impl Div<f32> for Vector {
    type Output = Vector;
    fn div(self, rhs: f32) -> Vector { Vector(self.0 / rhs, self.1 / rhs) }
}

impl DivAssign<f32> for Vector {
    fn div_assign(&mut self, rhs: f32) {
        self.0 /= rhs;
        self.1 /= rhs;
    }
}

impl Hash for Vector {
    fn hash<H: Hasher>(&self, state: &mut H) {
        truncate(self.0, 4).hash(state);
        truncate(self.1, 4).hash(state);
    }
}

impl Eq for Vector {}