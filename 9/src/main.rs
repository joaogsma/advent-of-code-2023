use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents: String =
        fs::read_to_string(&args[1])
            .expect("should be able to read the file");
    let version: u64 = args[2].parse().expect("Should have a problem version");

    let sequences: Vec<Vec<i32>> = contents.trim().split("\n").map(parse_line).collect();

    match version {
        1 => {
            let result: i32 = sequences.iter().map(extrapolate_forwards).sum();
            println!("Result is {result}");
        },
        2 => {
            let result: i32 = sequences.iter().map(extrapolate_backwards).sum();
            println!("Result is {result}");
        },
        _ => panic!("Unknown version {version}")
    };
}

fn parse_line(line: &str) -> Vec<i32> {
    line.split(' ').map(|x| x.parse().expect("Should be an integer")).collect()
}

fn extrapolate_forwards(sequence: &Vec<i32>) -> i32 {
    let all_zeroes = sequence.iter().all(|x| *x == 0);
    if all_zeroes {
        return 0;
    }
    let mut diff_sequence: Vec<i32> = Vec::new();
    for i in 1..sequence.len() {
        let diff = sequence[i] - sequence[i - 1];
        diff_sequence.push(diff);
    }
    let extrapolated_diff: i32 = extrapolate_forwards(&diff_sequence);
    sequence.last().unwrap() + extrapolated_diff
}

fn extrapolate_backwards(sequence: &Vec<i32>) -> i32 {
    let all_zeroes = sequence.iter().all(|x| *x == 0);
    if all_zeroes {
        return 0;
    }
    let mut diff_sequence: Vec<i32> = Vec::new();
    for i in 1..sequence.len() {
        let diff = sequence[i] - sequence[i - 1];
        diff_sequence.push(diff);
    }
    let extrapolated_diff: i32 = extrapolate_backwards(&diff_sequence);
    sequence.first().unwrap() - extrapolated_diff
}