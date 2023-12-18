use crate::geometry::Point;

pub trait Positioned2D {
  fn position(&self) -> &Point;
}