use std::collections::HashSet;
use std::env;
use std::fs;
use std::hash::{Hash, Hasher};

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents: String =
        fs::read_to_string(&args[1])
            .expect("should be able to read the file");
    let version: u64 = args[2].parse().expect("Should have a problem version");
    let mut lines = contents.trim().split("\n").map(String::from);

    let mut terrains: Vec<Terrain> = Vec::new();
    loop {
        match parse_terrain(&mut lines) {
            None => break,
            Some(terrain) => terrains.push(terrain)
        }
    }

    match version {
        1 => {
            let result: u32 = terrains.iter()
                .map(|terrain|
                    find_reflections(terrain).iter()
                        .map(|line| line.score(&terrain.bounds))
                        .sum::<u32>())
                .sum();
            println!("Result is {result}");
        },
        2 => {
            let result: u32 =
                terrains.iter()
                    .map(|terrain| find_reflection_after_smudge(terrain).score(&terrain.bounds))
                    .sum();
            println!("Result is {result}");
        },
        _ => panic!("Unknown version {version}")
    };
}

fn parse_terrain<T: Iterator<Item = String>>(lines: &mut T) -> Option<Terrain> {
    let lines: Vec<String> = lines.by_ref().take_while(|line| !line.is_empty()).collect();
    if lines.is_empty() {
        return None;
    }
    let bounds = Bounds::from_origin(&Point(lines[0].len() as f32 - 1.0, lines.len() as f32 - 1.0));
    let rocks: HashSet<Point> =
        lines.iter()
            .enumerate()
            .map(|(index, line)| ((lines.len() - 1 - index) as f32, line))
            .flat_map(|(y, line)| parse_line(line, y))
            .collect();
    Some(Terrain { bounds, rocks })
}

fn parse_line(line: &str, y: f32) -> Vec<Point> {
    line.chars()
        .enumerate()
        .filter_map(|(x, c)| if c == '.' { None } else { Some(Point(x as f32, y)) })
        .collect()
}

fn find_reflections(terrain: &Terrain) -> Vec<Line> {
    let Terrain { bounds, rocks } = terrain;
    generate_all_lines(terrain).into_iter()
        .filter(|line| {
            rocks.iter().all(|rock| {
                let mirrored_pos: Point = rock.mirror_across(line);
                !bounds.contains(&mirrored_pos) || bounds.contains(&mirrored_pos) == rocks.contains(&mirrored_pos)
            })
        })
        .collect()
}

fn find_reflection_after_smudge(terrain: &Terrain) -> Line {
    let Terrain { bounds, rocks } = terrain;
    let smudge: Point =
        generate_all_lines(terrain).into_iter()
            .find_map(|line| {
                let mut last_mismatch: Option<&Point> = None;
                let mut mismatch_count = 0;
                for rock in rocks.iter() {
                    let mirrored_pos: Point = rock.mirror_across(&line);
                    if !bounds.contains(&mirrored_pos) || bounds.contains(&mirrored_pos) == rocks.contains(&mirrored_pos) {
                        continue;
                    }
                    last_mismatch = Some(rock);
                    mismatch_count += 1;
                }
                last_mismatch.filter(|_| mismatch_count == 1).cloned()
            })
            .unwrap();

    let new_terrain: Terrain =
        Terrain { bounds: *bounds, rocks: rocks.iter().filter(|e| **e != smudge).cloned().collect() };
    let reflections_with_smudge = find_reflections(terrain);
    let mut reflections_without_smudge: HashSet<Line> = find_reflections(&new_terrain).into_iter().collect();
    for old_reflection in reflections_with_smudge.iter() {
        reflections_without_smudge.remove(old_reflection);
    }
    reflections_without_smudge.iter().find(|_| true).cloned().unwrap()
}

// Generates all possible lines between integer numbers. The vector defining the line is
// always normalized and the point is always in one of the axis.
fn generate_all_lines(terrain: &Terrain) -> Vec<Line> {
    let bottom_left: &Point = &terrain.bounds.bottom_left;
    let top_right: &Point = &terrain.bounds.top_right;
    let vertical_lines =
        (bottom_left.0 as i32 .. top_right.0 as i32)
            .map(|x| x as f32 + 0.5)
            .map(|x| Line(Point(x, terrain.bounds.bottom_left.1), Vector(1.0, 0.0)));
    let horizontal_lines =
        (bottom_left.1 as i32 .. top_right.1 as i32)
            .map(|y| y as f32 + 0.5)
            .map(|y| Line(Point(terrain.bounds.bottom_left.0, y), Vector(0.0, 1.0)));
    vertical_lines.chain(horizontal_lines).collect()
}

fn truncate(value: f32, decimal_places: u32) -> u64 {
    if decimal_places == 0 {
        return value as u64;
    }
    ((value * 10.0_f32.powi(decimal_places as i32 - 1)) * 10.0) as u64
}

struct Terrain {
    bounds: Bounds,
    rocks: HashSet<Point>
}

impl Terrain {
    #[allow(dead_code)]
    fn to_string(&self) -> String {
        let mut result = String::new();
        for y in (self.bounds.bottom_left.1 as u32 ..= self.bounds.top_right.1 as u32).rev() {
            for x in self.bounds.bottom_left.0 as u32 ..= self.bounds.top_right.0 as u32 {
                let p = Point(x as f32, y as f32);
                result.push(if self.rocks.contains(&p) { '#' } else { '.' });
            }
            if y != self.bounds.bottom_left.1 as u32 {
                result.push('\n');
            }
        }
        result
    }
}

#[derive(Copy, Clone, Debug)]
struct Bounds {
    bottom_left: Point,
    top_right: Point
}

impl Bounds {
    fn from_origin(p: &Point) -> Bounds { Self::from_two(&Point::ORIGIN, p) }

    fn from_two(p1: &Point, p2: &Point) -> Bounds {
        let vec: Vector = p2.minus(p1);
        let vec_x_component: Vector = Vector(vec.0, 0.0);
        let vec_y_component: Vector = Vector(0.0, vec.1);
        Self::from_four(p1, &p1.plus(&vec_x_component), p2, &p1.plus(&vec_y_component))
    }

    fn from_four(p1: &Point, p2: &Point, p3: &Point, p4: &Point) -> Bounds {
        let min_x = p1.0.min(p2.0).min(p3.0).min(p4.0);
        let min_y = p1.1.min(p2.1).min(p3.1).min(p4.1);
        let max_x = p1.0.max(p2.0).max(p3.0).max(p4.0);
        let max_y = p1.1.max(p2.1).max(p3.1).max(p4.1);
        Bounds { bottom_left: Point(min_x, min_y), top_right: Point(max_x, max_y) }
    }

    fn contains(&self, p: &Point) -> bool {
        let within_x_bounds = self.bottom_left.0 <= p.0 && p.0 <= self.top_right.0;
        let withing_y_bounds = self.bottom_left.1 <= p.1 && p.1 <= self.top_right.1;
        within_x_bounds && withing_y_bounds
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
struct Point(f32, f32);

impl Point {
    const ORIGIN: Point = Point(0.0, 0.0);

    fn plus(&self, v: &Vector) -> Point { Point(self.0 + v.0, self.1 + v.1) }
    fn minus(&self, p: &Point) -> Vector { Vector(self.0 - p.0, self.1 - p.1) }

    fn mirror_across(&self, line: &Line) -> Point {
        let closest: Point = line.point_closest_to(self);
        let transform: Vector = closest.minus(self).mult(2.0);
        self.plus(&transform)
    }
}

impl Eq for Point {}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        truncate(self.0, 3).hash(state);
        truncate(self.1, 3).hash(state);
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
struct Vector(f32, f32);

impl Vector {
    fn mult(&self, value: f32) -> Vector { Vector(self.0 * value, self.1 * value) }

    #[allow(dead_code)]
    fn div(&self, value: f32) -> Vector { Vector(self.0 / value, self.1 / value) }

    #[allow(dead_code)]
    fn dot_product(&self, other: &Vector) -> f32 { (self.0 * other.0) + (self.1 * other.1) }

    #[allow(dead_code)]
    fn orthogonal(&self) -> Vector { Vector(self.1, -self.0) }

    #[allow(dead_code)]
    fn inverse(&self) -> Vector { Vector(-self.0, -self.1) }
}

impl Hash for Vector {
    fn hash<H: Hasher>(&self, state: &mut H) {
        truncate(self.0, 3).hash(state);
        truncate(self.1, 3).hash(state);
    }
}

impl Eq for Vector {}

// Line representation using a point and an orthogonal vector
#[derive(PartialEq, Debug, Copy, Clone, Hash)]
struct Line(Point, Vector);

impl Line {
    fn point_closest_to(&self, p: &Point) -> Point {
        let Point(x0, y0) = self.0;
        let Point(px, py) = p;
        let Vector(vx, vy) = self.1;
        // Convert from point and vector line representation to line equation representation: ax + by + c = 0.
        // Values a, b and c below correspond to the equation
        let a: f32 = vx;
        let b: f32 = vy;
        let c: f32 = - vx * x0 - vy * y0;
        
        let inv_denominator: f32 = 1_f32 / (a*a + b*b);
        let common_factor: f32 = b*px - a*py; 
        let x: f32 = (b*common_factor - a*c) * inv_denominator;
        let y: f32 = (a*-common_factor - b*c) * inv_denominator;
        
        Point(x, y)
    }

    fn score(&self, bounds: &Bounds) -> u32 {
        let is_vertical = self.1.0.abs() > 0.0;
        if is_vertical {
            return (self.0.0 + 0.5) as u32;
        }
        let max_y = bounds.top_right.1;
        100 * (max_y - self.0.1 + 0.5) as u32
    }
}

impl Eq for Line {}