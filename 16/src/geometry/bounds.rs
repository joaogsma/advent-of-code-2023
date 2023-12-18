use crate::geometry::{Point, Vector};

#[derive(Copy, Clone, Debug, PartialEq, Hash)]
pub struct Bounds {
  pub bottom_left: Point,
  pub top_right: Point
}

impl Bounds {
  pub fn from_origin(p: &Point) -> Bounds { Self::from_two(&Point::ORIGIN, p) }

  pub fn from_two(p1: &Point, p2: &Point) -> Bounds {
      let vec: Vector = *p2 - *p1;
      let vec_x_component: Vector = Vector(vec.0, 0.0);
      let vec_y_component: Vector = Vector(0.0, vec.1);
      Self::from_four(p1, &(*p1 + vec_x_component), p2, &(*p1 + vec_y_component))
  }

  pub fn from_four(p1: &Point, p2: &Point, p3: &Point, p4: &Point) -> Bounds {
      let min_x = p1.0.min(p2.0).min(p3.0).min(p4.0);
      let min_y = p1.1.min(p2.1).min(p3.1).min(p4.1);
      let max_x = p1.0.max(p2.0).max(p3.0).max(p4.0);
      let max_y = p1.1.max(p2.1).max(p3.1).max(p4.1);
      Bounds { bottom_left: Point(min_x, min_y), top_right: Point(max_x, max_y) }
  }

  pub fn contains(&self, p: &Point) -> bool {
      let within_x_bounds = self.bottom_left.0 <= p.0 && p.0 <= self.top_right.0;
      let withing_y_bounds = self.bottom_left.1 <= p.1 && p.1 <= self.top_right.1;
      within_x_bounds && withing_y_bounds
  }
}

impl Eq for Bounds {}