use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::iter;

use geometry::Bounds;
use geometry::Point;
use geometry::Vector;

mod geometry;

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents: String =
        fs::read_to_string(&args[1])
            .expect("should be able to read the file");
    let version: u32 = args[2].parse().expect("Should have a problem version");
    let lines: Vec<&str> = contents.trim().split("\n").collect();
    let (mobile_rocks, fixed_rocks) =
        lines.iter()
            .enumerate()
            .map(|(index, line)| ((lines.len() - 1 - index) as f32, line))
            .map(|(y, line)| parse_line(line, y))
            .reduce(|mut a, b| {
                a.0.extend(b.0);
                a.1.extend(b.1);
                (a.0, a.1)
            })
            .unwrap();
    let top_right_tile = Point(lines.len() as f32 - 1.0, lines[0].len() as f32 - 1.0);
    let bounds = Bounds::from_origin(&top_right_tile);

    match version {
        1 => {
            let result: u32 =
                tilt_north(&mobile_rocks, &fixed_rocks, &bounds).iter().map(|Point(_, y)| *y as u32 + 1).sum();
            println!("Result is {result}");
        },
        2 => {
            let looped_config = find_repeated_config(&mobile_rocks, &fixed_rocks, &bounds);
            let remaining_cycles_after_loop: u32 = (1_000_000_000 - looped_config.offset) % looped_config.loop_len;
            let final_config: HashSet<Point> =
                iter::successors(
                    Some(looped_config.configuration),
                    |configuration| Some(tilt_cycle(configuration, &fixed_rocks, &bounds)))
                    .nth(remaining_cycles_after_loop as usize)
                    .unwrap();
            let result: u32 = final_config.iter().map(|Point(_, y)| *y as u32 + 1).sum();
            println!("Result is {result}");
        },
        _ => panic!("Unknown version {version}")
    };
}

fn parse_line(line: &str, y: f32) -> (HashSet<Point>, HashSet<Point>) {
    let mut mobile_rocks: HashSet<Point> = HashSet::new();
    let mut fixed_rocks: HashSet<Point> = HashSet::new();
    for (x, c) in line.chars().enumerate() {
        let x = x as f32;
        let position = Point(x, y);
        match c {
            '.' => continue,
            'O' => {
                mobile_rocks.insert(position);
            },
            '#' => { fixed_rocks.insert(position); },
            _ => panic!("Unexpected character {c}")
        }
    }
    (mobile_rocks, fixed_rocks)
}

fn find_repeated_config(
    mobile_rocks: &HashSet<Point>,
    fixed_rocks: &HashSet<Point>,
    bounds: &Bounds
) -> Loop {
    let mut seen_configurations: HashMap<Vec<Point>, u32> = HashMap::new();
    let mut current_configuration: HashSet<Point> = mobile_rocks.clone();
    for iteration in 1_u32.. {
        current_configuration = tilt_cycle(&current_configuration, fixed_rocks, bounds);
        let mut hash: Vec<Point> = current_configuration.iter().cloned().collect();
        hash.sort_by(|Point(x1, y1), Point(x2, y2)| {
            let y_cmp = y1.total_cmp(y2);
            if y_cmp != Ordering::Equal {
                return y_cmp;
            }
            x1.total_cmp(x2)
        });

        if seen_configurations.contains_key(&hash) {
            let offset: u32 = *seen_configurations.get(&hash).unwrap();
            let loop_len: u32 = iteration - offset;
            return Loop { configuration: current_configuration, offset, loop_len };
        }
        seen_configurations.insert(hash, iteration);
    }
    panic!("Escaped an infinite loop!");
}

struct Loop {
    configuration: HashSet<Point>,
    offset: u32,
    loop_len: u32
}

fn tilt_cycle(
    mobile_rocks: &HashSet<Point>,
    fixed_rocks: &HashSet<Point>,
    bounds: &Bounds
) -> HashSet<Point> {
    let tilted1 = tilt_north(mobile_rocks, fixed_rocks, bounds);
    let tilted2 = tilt_west(&tilted1, fixed_rocks, bounds);
    let tilted3 = tilt_south(&tilted2, fixed_rocks, bounds);
    let tilted4 = tilt_east(&tilted3, fixed_rocks, bounds);
    tilted4
}

fn tilt_north(
    mobile_rocks: &HashSet<Point>,
    fixed_rocks: &HashSet<Point>,
    bounds: &Bounds
) -> HashSet<Point> {
    tilt(mobile_rocks, fixed_rocks, bounds, &bounds.top_right, &Vector::LEFT, &Vector::DOWN)
}

fn tilt_east(
    mobile_rocks: &HashSet<Point>,
    fixed_rocks: &HashSet<Point>,
    bounds: &Bounds
) -> HashSet<Point> {
    tilt(mobile_rocks, fixed_rocks, bounds, &bounds.top_right, &Vector::DOWN, &Vector::LEFT)
}

fn tilt_south(
    mobile_rocks: &HashSet<Point>,
    fixed_rocks: &HashSet<Point>,
    bounds: &Bounds
) -> HashSet<Point> {
    tilt(mobile_rocks, fixed_rocks, bounds, &bounds.bottom_left, &Vector::RIGHT, &Vector::UP)
}

fn tilt_west(
    mobile_rocks: &HashSet<Point>,
    fixed_rocks: &HashSet<Point>,
    bounds: &Bounds
) -> HashSet<Point> {
    tilt(mobile_rocks, fixed_rocks, bounds, &bounds.bottom_left, &Vector::UP, &Vector::RIGHT)
}

fn tilt(
    mobile_rocks: &HashSet<Point>,
    fixed_rocks: &HashSet<Point>,
    bounds: &Bounds,
    initial_pos: &Point,
    initial_pos_step: &Vector,
    scan_step: &Vector
) -> HashSet<Point> {
    let mut final_positions: HashSet<Point> = HashSet::new();
    let initial_positions =
        iter::successors(
            Some(*initial_pos),
            |pos| Some(*pos + *initial_pos_step).filter(|p| bounds.contains(p)));

    for mut landing_pos in initial_positions {
        let scan_positions =
            iter::successors(
                Some(landing_pos),
                |pos| Some(*pos + *scan_step).filter(|p| bounds.contains(&p)));
        for pos in scan_positions {
            if mobile_rocks.contains(&pos) {
                final_positions.insert(landing_pos);
                landing_pos += *scan_step;
            } else if fixed_rocks.contains(&pos) {
                landing_pos = pos + *scan_step;
            }
        }
    }
    final_positions
}

#[allow(dead_code)]
fn to_string(mobile_rocks: &HashMap<Point, u32>, fixed_rocks: &HashSet<Point>, top_right_tile: &Point) -> String {
    let mut result = String::new();
    for y in (0..=top_right_tile.1 as i32).rev() {
        for x in 0..=top_right_tile.0 as i32 {
            let position = Point(x as f32, y as f32);
            if mobile_rocks.contains_key(&position) {
                result.push('O');
                continue;
            }
            if fixed_rocks.contains(&position) {
                result.push('#');
                continue;
            }
            result.push('.');
        }
        result.push('\n');
    }
    return result;
}