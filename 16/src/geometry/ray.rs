use crate::EPSILON;
use crate::geometry::{Point, Vector, Line, Bounds};

// Ray implementation using a point and a vector in the direction of the ray
#[derive(PartialEq, Eq, Debug, Copy, Clone, Hash)]
pub struct Ray(pub Point, pub Vector);

impl Ray {
  pub fn hits(&self, target: &Point) -> bool {
      if self.0.distance(&target) <= EPSILON {
          // target is ray origin - checks below depend on diff vector and would fail because it would be Vector(0, 0)
          return true;
      }
      let diff: Vector = *target - self.0;
      self.1.orthogonal().dot_product(&diff).abs() <= EPSILON  // Target is aligned with ray direction
          && diff.dot_product(&self.1) > 0.0 // Check if at or ahead of ray origin
  }

  pub fn intersect_line(&self, target: &Line) -> Option<Point> {
      if target.contains(&self.0) {
          return Some(self.0);
      }
      if self.1.dot_product(&target.1).abs() <= EPSILON {
          // Ray parallel to line
          return None;
      }

      let ray_line = Line(self.0, self.1.orthogonal());
      
      let a1: f32 = ray_line.1.0;
      let b1: f32 = ray_line.1.1;
      let c1: f32 = - ray_line.1.0 * ray_line.0.0 - ray_line.1.1 * ray_line.0.1;

      let a2: f32 = target.1.0;
      let b2: f32 = target.1.1;
      let c2: f32 = - target.1.0 * target.0.0 - target.1.1 * target.0.1;

      let inv_denominator: f32 = 1.0 / (a1*b2 - a2*b1);
      let x: f32 = (b1*c2 - b2*c1) * inv_denominator;
      let y: f32 = (c1*a2 - c2*a1) * inv_denominator;
      let intersection: Point = Point(x, y);
      
      if !self.hits(&intersection) {
          // Ray goes away from intersection
          return None;
      }
      Some(intersection)
  }

  pub fn intersect_bounds(&self, target: &Bounds) -> Option<Point> {
      let side_lines = vec![
          Line(target.top_right, Vector::UP),
          Line(target.top_right, Vector::RIGHT),
          Line(target.bottom_left, Vector::DOWN),
          Line(target.bottom_left, Vector::LEFT)
      ];
      side_lines.iter()
          .filter_map(|side_line| self.intersect_line(&side_line))
          .filter(|intersection| target.contains(intersection))
          .reduce(|intersection1, intersection2| {
              let distance1: f32 = self.0.distance(&intersection1);
              let distance2: f32 = self.0.distance(&intersection2);
              if distance1 <= distance2 { intersection1 } else { intersection2 }
          })
  }
}