use std::hash::{Hash, Hasher};
use std::ops::{Add, AddAssign, Sub};

use crate::geometry::Vector;
use crate::geometry::truncate;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Point(pub f32, pub f32);

impl Point {
    pub const ORIGIN: Point = Point(0.0, 0.0);
}

impl Add<Vector> for Point {
    type Output = Point;
    fn add(self, rhs: Vector) -> Point { Point(self.0 + rhs.0, self.1 + rhs.1) }
}

impl AddAssign<Vector> for Point {
    fn add_assign(&mut self, rhs: Vector) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Sub<Point> for Point {
    type Output = Vector;
    fn sub(self, rhs: Point) -> Vector { Vector(self.0 - rhs.0, self.1 - rhs.1) }
}

impl Eq for Point {}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        truncate(self.0, 4).hash(state);
        truncate(self.1, 4).hash(state);
    }
}