use std::collections::HashSet;
use std::env;
use std::fs;
use std::cmp;

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents: String =
        fs::read_to_string(&args[1])
            .expect("should be able to read the file");
    let version: u32 = args[2].parse().expect("Should have a problem version");

    let parsed_lines: Vec<Scratchcard> =
        contents
            .split("\n")
            .filter(|e| !e.is_empty())
            .map(parse_line)
            .collect();

    match version {
        1 => {
            let result: u32 = parsed_lines.iter().map(compute_points).sum();
            println!("Result: {result}");
        }
        2 => {
            let result: u32 = process_scratchcards(&parsed_lines).iter().map(|x| x.0).sum();
            println!("Result: {result}");
        }
        _ => panic!("Unknown version {version}")
    }
}

fn parse_line(line: &str) -> Scratchcard {
    let pos = line.find(":").expect("Line should have : symbol");
    let line = &line[pos + 2..];
    let parts: Vec<&str> = line.split(" | ").collect();
    Scratchcard(parse_numbers(parts[0]), parse_numbers(parts[1]))
}

fn parse_numbers(value: &str) -> HashSet<u32> {
    value
        .split(" ")
        .filter(|e| !e.is_empty()) // Two spaces in a row are possible with <10 numbers
        .map(|e| {
            e.parse().expect("Should be number")
        })
        .collect()
}

fn process_scratchcards(elements: &Vec<Scratchcard>) -> Vec<(u32, Scratchcard)> {
    let mut result: Vec<(u32, Scratchcard)> = Vec::new();
    for e in elements {
        result.push((1, e.clone()));
    }

    for index in 0..result.len() {
        let mult_factor: u32 = result[index].0;
        let current: &Scratchcard = &result[index].1;
        let matches: usize = compute_matches(current) as usize;
        let copy_index_end: usize = cmp::min(index + matches + 1, result.len());
        for copy_index in (index + 1)..copy_index_end  {
            result[copy_index].0 += mult_factor;
        }
    }

    result
}

fn compute_points(value: &Scratchcard) -> u32 {
    let matches = compute_matches(value);
    if matches == 0 {
        return 0;
    }
    let base: u32 = 2;
    base.pow(matches - 1)
}

fn compute_matches(value: &Scratchcard) -> u32 {
    let intersection: Vec<&u32> = value.0.intersection(&value.1).collect();
    intersection.len() as u32
}

#[derive(Clone)]
struct Scratchcard(HashSet<u32>, HashSet<u32>);