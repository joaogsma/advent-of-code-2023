use crate::EPSILON;
use crate::geometry::{Point, Vector};

// Line representation using a point and an orthogonal vector
#[derive(PartialEq, Debug, Copy, Clone, Hash)]
pub struct Line(pub Point, pub Vector);

impl Line {
  pub fn contains(&self, target: &Point) -> bool { (*target - self.0).dot_product(&self.1).abs() <= EPSILON }
}

impl Eq for Line {}