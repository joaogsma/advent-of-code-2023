use std::collections::{HashSet, HashMap};

use crate::quad_tree::{QuadTree, Positioned2D};
use crate::geometry::{Ray, Point, Vector, Bounds};
use crate::obstacle::Obstacle;

pub fn find_most_energized_configuration(qt: &QuadTree<Obstacle>, top_right_tile: &Point) -> HashMap<Point, Vec<Vector>> {
    let mut rays: Vec<Ray> = Vec::new();
    for x in (0..=top_right_tile.0 as i32).map(|x| x as f32) {
        rays.push(Ray(Point(x, top_right_tile.1 + 1.0), Vector::DOWN));
        rays.push(Ray(Point(x, -1.0), Vector::UP));
    }
    for y in (0..=top_right_tile.1 as i32).map(|y| y as f32) {
        rays.push(Ray(Point(-1.0, y), Vector::RIGHT));
        rays.push(Ray(Point(top_right_tile.0 + 1.0, y), Vector::LEFT));
    }
    rays.iter()
        .map(|ray| (ray, find_energized_tiles(ray, qt, top_right_tile)))
        .max_by(|(_, tiles1), (_, tiles2)| tiles1.len().cmp(&tiles2.len()))
        .map(|(ray, tiles)| {
            println!("Best Ray is {:?}", ray);
            tiles
        })
        .unwrap_or(HashMap::new())
}

pub fn find_energized_tiles(ray: &Ray, qt: &QuadTree<Obstacle>, top_right_tile: &Point) -> HashMap<Point, Vec<Vector>> {
    // We check the intersection with an expanded boundary because we want to count
    // rays that run along the edges
    let expanded_bounds =
        Bounds::from_two(&Point(-1.0, -1.0), &(*top_right_tile + Vector(1.0, 1.0)));
    let tiles: HashMap<Point, Vec<Vector>> = find_energized_tiles_rec(qt, &ray, &mut HashSet::new(), &expanded_bounds);
    tiles.iter()
        .filter(|(pos, _)| Bounds::from_origin(top_right_tile).contains(pos))
        .map(|(pos, vectors)| (pos.clone(), vectors.clone()))
        .collect()
}

fn find_energized_tiles_rec(
    qt: &QuadTree<Obstacle>,
    ray: &Ray,
    visited: &mut HashSet<Ray>,
    expanded_bounds: &Bounds
) -> HashMap<Point, Vec<Vector>> {
    if visited.contains(ray) {
        return HashMap::new();
    }
    visited.insert(ray.clone());
    match qt.intersect(ray, true) {
        None => {
            let bounds_hit = match ray.intersect_bounds(&expanded_bounds) {
                None => return HashMap::new(),
                Some(hit) => hit
            };
            return get_discrete_points_until(&ray.0, &bounds_hit);
        }
        Some(hit) => {
            let mut energized_by_ray: HashMap<Point, Vec<Vector>> = get_discrete_points_until(&ray.0, hit.position());
            let new_rays: Vec<Ray> = hit.reflect(ray);
            let energized_by_new_rays =
                new_rays.iter().flat_map(|new_ray| find_energized_tiles_rec(qt, new_ray, visited, expanded_bounds));
            for (point, vectors) in energized_by_new_rays {
                if !energized_by_ray.contains_key(&point) {
                    energized_by_ray.insert(point, Vec::new());
                }
                energized_by_ray.get_mut(&point).unwrap().extend(vectors);
            }
            return energized_by_ray;
        }
    };
}

fn get_discrete_points_until(src: &Point, dst: &Point) -> HashMap<Point, Vec<Vector>> {
    let distance: i32 = src.distance(dst).round() as i32;
    let step: Vector = (*dst - *src).normalize();
    let step = Vector(step.0.round(), step.1.round());
    (0..=distance)
        .map(|n| discretize(*src + (step * n as f32)))
        .map(|point| (point, vec![step]))
        .collect()
}

fn discretize(point: Point) -> Point { Point(point.0.round(), point.1.round()) }