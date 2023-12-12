use std::collections::HashSet;
use std::collections::LinkedList;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents: String =
        fs::read_to_string(&args[1])
            .expect("should be able to read the file");
    let version: u64 = args[2].parse().expect("Should have a problem version");

    let parsed_lines: Vec<Vec<Option<Pipe>>> = contents.trim()
        .split("\n")
        .enumerate()
        .map(|(row, line)| parse_line(line, row as i32))
        .collect();
    let grid =
        Grid {
            rows: parsed_lines.len(),
            cols: parsed_lines[0].len(),
            data: parsed_lines.into_iter().flatten().collect()
        };
    let start_pos: Coord = find_start_pos(&grid);

    match version {
        1 => {
            let result = find_pipe_loop_length(&grid, start_pos) / 2;
            println!("Result is {result}");
        },
        2 => {
            let result = find_coords_inside_pipe_loop(&grid, start_pos).len();
            println!("Result is {result}");
        },
        _ => panic!("Unknown version {version}")
    };
}

fn parse_line(line: &str, row: i32) -> Vec<Option<Pipe>> {
    line.chars()
        .enumerate()
        .map(|(col, c)| {
            let coord = Coord(row, col as i32);
            match c {
                '.' => None,
                '|' => Some(Pipe::Vertical(coord)),
                '-' => Some(Pipe::Horizontal(coord)),
                'L' => Some(Pipe::NECurve(coord)),
                'J' => Some(Pipe::NWCurve(coord)),
                '7' => Some(Pipe::SWCurve(coord)),
                'F' => Some(Pipe::SECurve(coord)),
                'S' => Some(Pipe::StarPos),
                _ => panic!("Unknown character {c}")
            }
        })
        .collect()
}

fn find_start_pos(grid: &Grid) -> Coord {
    for row in 0..(grid.rows as i32) {
        for col in 0..(grid.cols as i32) {
            if grid.get(Coord(row, col)) == Some(Pipe::StarPos) {
                return Coord(row, col);
            }
        }
    }
    panic!("Could not find start position");
}

fn find_pipe_loop_length(grid: &Grid, start_pos: Coord) -> u32 {
    find_pipe_loop_directions(grid, start_pos).len() as u32
}

fn find_pipe_loop_directions(grid: &Grid, start_pos: Coord) -> Vec<Direction> {
    for neighbour in get_connected_neighbours(grid, start_pos) {
        match find_pipe_loop_directions_rec(grid, neighbour, start_pos, start_pos) {
            None => continue,
            Some(directions) => return directions
        }
    }
    panic!("There are no loops!");
}

fn find_pipe_loop_directions_rec(
    grid: &Grid,
    current: Coord,
    previous: Coord,
    target: Coord,
) -> Option<Vec<Direction>> {
    if current == target {
        return Some(vec![previous.to(current)]);
    }
    match grid.get(current) {
        None => None,
        Some(pipe) => {
            if !pipe.can_pass_through(previous) {
                return None;
            }
            find_pipe_loop_directions_rec(grid, pipe.pass_through(previous), current, target)
                .map(|tail| {
                    let mut head = vec![previous.to(current)];
                    head.extend(tail);
                    head
                })
        }
    }
}

fn get_connected_neighbours(grid: &Grid, start_pos: Coord) -> Vec<Coord> {
    start_pos.neighbours().into_iter()
        .filter(|&neighbour| match grid.get(neighbour) {
            None => false,
            Some(pipe) => pipe.can_pass_through(start_pos)
        })
        .collect()
}

fn find_coords_inside_pipe_loop(grid: &Grid, start_pos: Coord) -> HashSet<Coord> {
    let rows_micro: i32 = 2 * grid.rows as i32;
    let cols_micro: i32 = 2 * grid.cols as i32;

    let forbidden: HashSet<Coord> = find_pipe_loop_micro_coords(grid, start_pos);
    let mut visited: HashSet<Coord> = HashSet::new();
    let mut queue: LinkedList<Coord> = LinkedList::new();

    (0..cols_micro)
        .flat_map(|col_micro| [Coord(0, col_micro), Coord(rows_micro - 1, col_micro)])
        .filter(|coord_micro| !forbidden.contains(coord_micro))
        .for_each(|coord_micro| queue.push_back(coord_micro));
    (0..rows_micro)
        .flat_map(|col_micro| [Coord(0, col_micro), Coord(rows_micro - 1, col_micro)])
        .filter(|coord_micro| !forbidden.contains(coord_micro))
        .for_each(|coord_micro| queue.push_back(coord_micro));

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        if visited.contains(&current) || forbidden.contains(&current) {
            continue;
        }
        current.neighbours().iter()
            .filter(|Coord(row_micro, col_micro)|
                *row_micro >= 0 && *row_micro < rows_micro
                    && *col_micro >= 0 && *col_micro < cols_micro)
            .filter(|coord| !visited.contains(coord))
            .filter(|coord| !forbidden.contains(coord))
            .for_each(|&coord| queue.push_back(coord));
        visited.insert(current);
    }

    let all_micro_coords =
        (0..rows_micro)
            .flat_map(|row| (0..cols_micro).map(move |col| Coord(row, col)));
    let unvisited_micro_coords =
        all_micro_coords
            .filter(|coord| !visited.contains(coord))
            .filter(|coord| !forbidden.contains(coord));
    unvisited_micro_coords
        .filter(|Coord(row_micro, col_micro)| row_micro % 2 == 0 && col_micro % 2 == 0)
        .map(|Coord(row_micro, col_micro)| Coord(row_micro / 2, col_micro / 2))
        .collect::<HashSet<Coord>>()
}

fn find_pipe_loop_micro_coords(grid: &Grid, start_pos_macro: Coord) -> HashSet<Coord> {
    let directions_macro: Vec<Direction> = find_pipe_loop_directions(grid, start_pos_macro);
    let start_pos_micro: Coord = Coord(start_pos_macro.0 * 2, start_pos_macro.1 * 2);
    let mut result: HashSet<Coord> = HashSet::from([start_pos_micro]);

    let mut current = start_pos_micro;
    for direction in directions_macro {
        for _ in 0..2 {
            current = current.plus(direction);
            result.insert(current);
        }
    }

    result
}

struct Grid {
    rows: usize,
    cols: usize,
    data: Vec<Option<Pipe>>
}

impl Grid {
    fn contains(&self, coord: Coord) -> bool {
        coord.0 >= 0 && (coord.0 as usize) < self.rows
            && coord.1 >= 0 && (coord.1 as usize) < self.cols
    }

    fn get(&self, coord: Coord) -> Option<Pipe> {
        if !self.contains(coord) {
            return None;
        }
        self.data[self.to_index(coord)]
    }

    // fn set(&mut self, coord: Coord, value: Pipe) {
    //     let index = self.to_index(coord);
    //     self.data[index] = Some(value)
    // }

    // fn remove(&mut self, coord: Coord) {
    //     let index = self.to_index(coord);
    //     self.data[index] = None
    // }

    fn to_index(&self, coord: Coord) -> usize { self.cols * (coord.0 as usize) + (coord.1 as usize)}

    // fn to_string(&self) -> String {
    //     let mut result = String::new();
    //     for row in 0..self.rows {
    //         for col in 0..self.cols {
    //             match self.get(Coord(row, col)) {
    //                 None => result.push('.'),
    //                 Some(Pipe::StarPos) => result.push('S'),
    //                 Some(Pipe::Vertical(_)) => result.push('|'),
    //                 Some(Pipe::Horizontal(_)) => result.push('-'),
    //                 Some(Pipe::NECurve(_)) => result.push('L'),
    //                 Some(Pipe::NWCurve(_)) => result.push('J'),
    //                 Some(Pipe::SWCurve(_)) => result.push('7'),
    //                 Some(Pipe::SECurve(_)) => result.push('F')
    //             }
    //         }
    //         result.push('\n');
    //     }
    //     result
    // }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Pipe {
    Vertical(Coord),
    Horizontal(Coord),
    NECurve(Coord),
    NWCurve(Coord),
    SWCurve(Coord),
    SECurve(Coord),
    StarPos
}

impl Pipe {
    fn pass_through(&self, origin: Coord) -> Coord {
        match self {
            Pipe::Vertical(pipe_coord) => {
                if pipe_coord.to(origin) == Direction::Up {
                    return pipe_coord.plus(Direction::Down)
                }
                pipe_coord.plus(Direction::Up)
            }
            Pipe::Horizontal(pipe_coord) => {
                if pipe_coord.to(origin) == Direction::Left {
                    return pipe_coord.plus(Direction::Right);
                }
                pipe_coord.plus(Direction::Left)
            }
            Pipe::NECurve(pipe_coord) => {
                if pipe_coord.to(origin) == Direction::Up {
                    return pipe_coord.plus(Direction::Right);
                }
                pipe_coord.plus(Direction::Up)
            }
            Pipe::NWCurve(pipe_coord) => {
                if pipe_coord.to(origin) == Direction::Up {
                    return pipe_coord.plus(Direction::Left)
                }
                pipe_coord.plus(Direction::Up)
            }
            Pipe::SWCurve(pipe_coord) => {
                if pipe_coord.to(origin) == Direction::Down {
                    return pipe_coord.plus(Direction::Left)
                }
                pipe_coord.plus(Direction::Down)
            }
            Pipe::SECurve(pipe_coord) => {
                if pipe_coord.to(origin) == Direction::Down {
                    return pipe_coord.plus(Direction::Right)
                }
                pipe_coord.plus(Direction::Down)
            },
            Pipe::StarPos => panic!("Can't pass through start position")
        }
    }

    fn can_pass_through(&self, origin: Coord) -> bool {
        match self {
            Pipe::Vertical(pipe_coord) => {
                let direction = pipe_coord.to(origin);
                direction == Direction::Down || direction == Direction::Up
            }
            Pipe::Horizontal(pipe_coord) => {
                let direction = pipe_coord.to(origin);
                direction == Direction::Left || direction == Direction::Right
            }
            Pipe::NECurve(pipe_coord) => {
                let direction = pipe_coord.to(origin);
                direction == Direction::Up || direction == Direction::Right
            }
            Pipe::NWCurve(pipe_coord) => {
                let direction = pipe_coord.to(origin);
                direction == Direction::Up || direction == Direction::Left
            }
            Pipe::SWCurve(pipe_coord) => {
                let direction = pipe_coord.to(origin);
                direction == Direction::Down || direction == Direction::Left
            }
            Pipe::SECurve(pipe_coord) => {
                let direction = pipe_coord.to(origin);
                direction == Direction::Down || direction == Direction::Right
            }
            Pipe::StarPos => false
        }
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Direction { Up, Down, Left, Right }

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
struct Coord(i32, i32);

impl Coord {
    fn to(&self, other: Coord) -> Direction {
        if *self == other {
            panic!("Same coordinates");
        }
        if self.0.abs_diff(other.0) + self.1.abs_diff(other.1) > 1 {
            panic!("Coordinates are not 4-neighbours");
        }
        if self.0 < other.0 {
            return Direction::Down;
        }
        if self.0 > other.0 {
            return Direction::Up;
        }
        if self.1 < other.1 {
            return Direction::Right;
        }
        Direction::Left
    }

    fn plus(&self, vector: Direction) -> Coord {
        match vector {
            Direction::Up => Coord(self.0 - 1, self.1),
            Direction::Down => Coord(self.0 + 1, self.1),
            Direction::Left => Coord(self.0, self.1 - 1),
            Direction::Right => Coord(self.0, self.1 + 1)
        }
    }

    fn neighbours(&self) -> Vec<Coord> {
        vec![
            self.plus(Direction::Up),
            self.plus(Direction::Down),
            self.plus(Direction::Left),
            self.plus(Direction::Right)]
    }
}