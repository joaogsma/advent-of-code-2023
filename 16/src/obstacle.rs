use crate::EPSILON;
use crate::geometry::{Point, Ray, Vector};
use crate::quad_tree::Positioned2D;

// Obstacle implementation based on position and orthogonal vector
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Obstacle {
    Splitter(Point, Vector),
    Mirror(Point, Vector)
}

impl Obstacle {
    pub fn reflect(&self, ray: &Ray) -> Vec<Ray> {
        match self {
            Obstacle::Splitter(pos, ort) => Self::reflect_on_splitter(pos, ort, ray),
            Obstacle::Mirror(pos, ort) => Self::reflect_on_mirror(pos, ort, ray)
        }
    }

    fn reflect_on_splitter(pos: &Point, ort: &Vector, ray: &Ray) -> Vec<Ray> {
        let dot_product = ray.1.dot_product(ort);
        let angle_cos = dot_product / (ort.magnitude() * ray.1.magnitude());
        let is_parallel = angle_cos.abs() <= EPSILON;
        let is_orthogonal = (angle_cos.abs() - 1.0).abs() <= EPSILON;
        if is_parallel {
            return vec![Ray(*pos, ray.1)];
        }
        if is_orthogonal {
            let new_direction = ort.orthogonal();
            return vec![Ray(*pos, new_direction), Ray(*pos, new_direction.inverse())];
        }
        panic!("Hit a splitter from a weird angle!");
    }

    fn reflect_on_mirror(pos: &Point, ort: &Vector, ray: &Ray) -> Vec<Ray> {
        let dot_product = ray.1.dot_product(ort);
        let angle_cos = dot_product / (ort.magnitude() * ray.1.magnitude());
        let is_parallel = angle_cos.abs() <= EPSILON;
        let is_orthogonal = (angle_cos.abs() - 1.0).abs() <= EPSILON;
        if is_parallel {
            panic!("Hit a mirror from a weird angle!");
        }
        if is_orthogonal {
            // Hitting the mirror head-on at an orthogonal angle
            return vec![Ray(*pos, ray.1.inverse())];
        }
        let normal_same_side: Vector = if angle_cos < 0.0 { ort.normalize() } else { ort.inverse().normalize() };
        let new_direction = ray.1 - normal_same_side * (2.0 * ray.1.dot_product(&normal_same_side));
        let clamped_new_direction =
            [Vector::UP, Vector::DOWN, Vector::LEFT, Vector::RIGHT]
                .iter()
                .max_by(|vec1, vec2| {
                    let dot_product1 = vec1.dot_product(&new_direction);
                    let dot_product2 = vec2.dot_product(&new_direction);
                    dot_product1.partial_cmp(&dot_product2).unwrap()
                })
                .unwrap();
        vec![Ray(pos.clone(), clamped_new_direction.clone())]
    }
}

impl Positioned2D for Obstacle {
    fn position(&self) -> &Point {
        match self {
            Obstacle::Splitter(pos, _) => pos,
            Obstacle::Mirror(pos, _, ) => pos
        }
    }
}