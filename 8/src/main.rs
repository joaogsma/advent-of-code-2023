use std::collections::HashMap;
use std::env;
use std::fs;
use std::cmp::max;
use std::cmp::min;

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents: String =
        fs::read_to_string(&args[1])
            .expect("should be able to read the file");
    let version: u64 = args[2].parse().expect("Should have a problem version");

    let input: Vec<&str> = contents.trim().split("\n").collect();
    let (graph, directions) = parse(&input);

    match version {
        1 => {
            let result = graph.count_moves("AAA", "ZZZ", &directions);
            println!("Result is {result}");
        },
        2 => {
            let result = graph.count_moves_parallel(&directions);
            println!("Result is {result}");
        },
        _ => panic!("Unknown version {version}")
    };
}

fn parse(lines: &Vec<&str>) -> (Graph, Vec<Direction>) {
    let directions = parse_directions(lines[0]);
    let mut graph_data: HashMap<String, Node> = HashMap::new();
    for line in lines.iter().skip(2) {
        let (node_id, node) = parse_node(line);
        graph_data.insert(node_id, node);
    }
    (Graph { data: graph_data }, directions)
}

fn parse_directions(line: &str) -> Vec<Direction> {
    line.chars()
        .map(|c| match c {
            'R' => Direction::Right,
            'L' => Direction::Left,
            _ => panic!("Unknown direction character")
        })
        .collect()
}

fn parse_node(line: &str) -> (String, Node) {
    let node_id = line[..3].to_string();
    let left = line[7..10].to_string();
    let right = line[12..15].to_string();
    (node_id, Node(left, right))
}

struct Graph {
    data: HashMap<String, Node>
}

impl Graph {
    fn go(&self, current: &str, direction: Direction) -> &str {
        let current = self.data.get(current).expect("Current node should exist");
        match current {
            Node(left, right) => match direction {
                Direction::Left => left,
                Direction::Right => right
            }
        }
    }

    fn count_moves(&self, start: &str, target: &str, directions: &Vec<Direction>) -> u64 {
        let mut current = start;
        let mut sum: u64 = 0;
        for direction in directions.iter().cycle() {
            if current == target {
                break;
            }
            sum += 1;
            current = self.go(current, *direction);
        }
        sum
    }

    fn count_moves_parallel(&self, directions: &Vec<Direction>) -> u64 {
        let start_positions: Vec<&str> =
            self.data.keys()
                .filter(|k| k.chars().nth(2).unwrap() == 'A')
                .map(|k| k.as_str())
                .collect();

        let loop_sizes: Vec<u64> =
            start_positions.iter()
                .map(|start_pos| {
                    let loops: Vec<Loop> = self.find_loops(start_pos, directions);
                    loops.iter().map(|l| l.size).max().unwrap()
                })
                .collect();
        
        let gcd_all = loop_sizes.clone().into_iter().reduce(gcd).unwrap();
        loop_sizes[1..].iter().fold(loop_sizes[0], |acc, elem| acc * (elem / gcd_all))
    }

    fn find_loops(&self, start_position: &str, directions: &Vec<Direction>) -> Vec<Loop> {
        let mut tracing: HashMap<(&str, usize), u64> = HashMap::new();
        let mut current_node: &str = start_position;
        let mut current_step: u64 = 0;

        for current_direction_idx in (0..directions.len()).cycle() {
            let tracing_element = (current_node, current_direction_idx);
            if tracing.contains_key(&tracing_element) { // We've see this before - we found a loop!
                break;
            }
            tracing.insert(tracing_element, current_step);
            current_node = self.go(current_node, directions[current_direction_idx]);
            current_step += 1;
        }

        let loop_start = current_node;
        let steps_until_loop_start: u64 = self.count_moves(start_position, loop_start, directions);
        let loop_size: u64 = current_step - steps_until_loop_start;

        tracing.into_iter()
            .filter(|((node, _), _)| node.chars().nth(2).unwrap() == 'Z')
            .map(|(_, steps)| Loop::from(steps, loop_size))
            .collect()
    }
}

#[derive(Debug)]
struct Node(String, String);

#[derive(Debug, Clone, Copy)]
enum Direction { Left, Right }

#[derive(Debug, Clone, Copy)]
struct Loop {
    size: u64,
    total_steps: u64
}

impl Loop {
    fn from(offset: u64, size: u64) -> Loop {
        Loop { size, total_steps: offset }
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    if a == b {
        return a;
    }
    gcd(max(a, b) - min(a, b), min(a, b))
}