use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents: String =
        fs::read_to_string(&args[1])
            .expect("should be able to read the file");
    let version: u64 = args[2].parse().expect("Should have a problem version");

    let lines: Vec<&str> = contents.trim().split("\n").collect();
    let max_y: i32 = lines.len() as i32 - 1;
    let galaxies: Vec<Point> =
        lines.iter()
            .enumerate()
            .map(|(row, line)| (max_y - row as i32, line))
            .flat_map(|(y, line)| parse_line(line, y))
            .collect();
    let empty_space: HashSet<Line> = find_empty_lines(&galaxies);
    let expansion_factor: u32 = match version {
        1 => 2,
        2 => 1000000,
        _ => panic!("Unknown version {version}")
    };
    let result: u64 =
        (0..galaxies.len())
            .flat_map(|i| ((i+1)..galaxies.len()).map(move |j| (i, j)))
            .map(|(i, j)| find_shortest_path(galaxies[i], galaxies[j], &empty_space, expansion_factor))
            .sum();
    println!("Result is {result}");
}

fn parse_line(line: &str, y: i32) -> Vec<Point> {
    line.chars()
        .enumerate()
        .filter_map(|(x, c)| if c == '#' { Some(Point(x as i32, y)) } else { None })
        .collect()
}

fn find_empty_lines(points: &Vec<Point>) -> HashSet<Line> {
    if points.is_empty() {
        return HashSet::new();
    }
    let seen_x_coords: HashSet<i32> = points.iter().map(|p| p.0).collect();
    let seen_y_coords: HashSet<i32> = points.iter().map(|p| p.1).collect();

    let min_x: i32 = seen_x_coords.iter().min().map(|x| *x).unwrap_or(0);
    let max_x: i32 = seen_x_coords.iter().max().map(|x| *x).unwrap_or(0);
    let vertical_lines =
        (min_x..max_x)
            .filter(|x| !seen_x_coords.contains(x))
            .map(|x| Line(Point(x, 0), Vector(1, 0)));

    let min_y: i32 = seen_y_coords.iter().min().map(|y| *y).unwrap_or(0);
    let max_y: i32 = seen_y_coords.iter().max().map(|y| *y).unwrap_or(0);
    let horizontal_lines =
        (min_y..max_y)
            .filter(|y| !seen_y_coords.contains(y))
            .map(|y| Line(Point(0, y), Vector(0, 1)));

    vertical_lines.chain(horizontal_lines).collect()
}

fn find_shortest_path(galaxy1: Point, galaxy2: Point, empty_space: &HashSet<Line>, expansion_factor: u32) -> u64 {
    let intersections: u64 =
        empty_space.iter()
            .filter(|line| line.are_on_different_sides(galaxy1, galaxy2))
            .count() as u64;
    galaxy1.distance_l1(galaxy2) as u64 + intersections * (expansion_factor as u64 - 1)
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
struct Point(i32, i32);

impl Point {
    fn minus(&self, p: Point) -> Vector { Vector(self.0 - p.0, self.1 - p.1) }
    fn distance_l1(&self, other: Point) -> u32 { self.0.abs_diff(other.0) + self.1.abs_diff(other.1) }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
struct Vector(i32, i32);

impl Vector {
    fn dot_product(&self, other: Vector) -> i32 { (self.0 * other.0) + (self.1 * other.1) }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
struct Line(Point, Vector);

impl Line {
    fn are_on_different_sides(&self, a: Point, b: Point) -> bool {
        let dot_product_a: i32 = a.minus(self.0).dot_product(self.1);
        let dot_product_b: i32 = b.minus(self.0).dot_product(self.1);
        if dot_product_a == 0 || dot_product_b == 0 {
            return false;
        }
        dot_product_a.signum() != dot_product_b.signum()
    }
}