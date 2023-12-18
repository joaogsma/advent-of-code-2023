use std::collections::HashMap;
use std::env;
use std::fs;


use geometry::{Point, Vector, Ray};
use obstacle::Obstacle;
use quad_tree::QuadTree;
use ray_tracer::find_energized_tiles;
use quad_tree::Positioned2D;
use ray_tracer::find_most_energized_configuration;

mod geometry;
mod quad_tree;
mod obstacle;
mod ray_tracer;

const EPSILON: f32 = 1e-4;

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents: String =
        fs::read_to_string(&args[1])
            .expect("should be able to read the file");
    let version: u32 = args[2].parse().expect("Should have a problem version");
    let lines: Vec<&str> = contents.trim().split("\n").collect();
    let obstacles: Vec<Obstacle> =
        lines.iter()
            .enumerate()
            .map(|(index, line)| ((lines.len() - 1 - index) as f32, line))
            .flat_map(|(y, line)| parse_line(line, y))
            .collect();
    let top_right_tile = Point(lines.len() as f32 - 1.0, lines[0].len() as f32 - 1.0);
    let qt: QuadTree<Obstacle> = QuadTree::from_bulk(&obstacles, 2);

    match version {
        1 => {
            let energized_tiles: HashMap<Point, Vec<Vector>> =
                find_energized_tiles(&Ray(Point(-1.0, top_right_tile.1), Vector::RIGHT), &qt, &top_right_tile);
            println!("Result is {}", energized_tiles.len());
        },
        2 => {
            let energized_tiles: HashMap<Point, Vec<Vector>> = find_most_energized_configuration(&qt, &top_right_tile);
            println!("Result is {}", energized_tiles.len());
        },
        _ => panic!("Unknown version {version}")
    };
}

fn parse_line(line: &str, y: f32) -> Vec<Obstacle> {
    line.chars()
        .enumerate()
        .map(|(index, c)| (index as f32, c))
        .filter_map(|(x, c)| {
            let position = Point(x, y);
            match c {
                '.' => None,
                '/' => Some(Obstacle::Mirror(position, Vector(-1.0, 1.0))),
                '\\' => Some(Obstacle::Mirror(position, Vector(1.0, 1.0))),
                '|' => Some(Obstacle::Splitter(position, Vector(1.0, 0.0))),
                '-' => Some(Obstacle::Splitter(position, Vector(0.0, 1.0))),
                _ => panic!("Unexpected character {c}")
            }
        })
        .collect()
}

#[allow(dead_code)]
fn to_string(items: &Vec<Obstacle>, top_right_tile: &Point, energized_tiles: &HashMap<Point, Vec<Vector>>) -> String {
    let mut result = String::new();
    for y in (0..=top_right_tile.1 as i32).rev() {
        for x in 0..=top_right_tile.0 as i32 {
            let x = x as f32;
            let y = y as f32;
            let item = match items.iter().find(|item| *item.position() == Point(x, y)) {
                None => {
                    match energized_tiles.get(&Point(x, y)) {
                        None => {
                            result.push('.');
                            continue;
                        }
                        Some(directions) => {
                            let character =
                                if directions.len() > 1 { directions.len().to_string() }
                                else if directions[0] == Vector::UP { String::from("^") }
                                else if directions[0] == Vector::DOWN { String::from("v") }
                                else if directions[0] == Vector::RIGHT { String::from(">") }
                                else if directions[0] == Vector::LEFT { String::from("<") }
                                else { panic!("Unknown directino vector") };
                            result.push_str(&character);
                            continue;
                        }
                    }
                }
                Some(item) => item
            };
            match item {
                Obstacle::Splitter(_, ort) =>
                    result.push(if ort.dot_product(&Vector::UP).abs() <= EPSILON { '|' } else { '-' } ),
                Obstacle::Mirror(_, ort) =>
                    result.push(if ort.dot_product(&Vector(1.0,1.0)).abs() <= EPSILON { '/' } else { '\\' } )
            }
        }
        result.push('\n');
    }
    return result;
}