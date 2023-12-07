use std::env;
use std::fs;

const TIME_PREFIX: &str = "Time:";
const DISTANCE_PREFIX: &str = "Distance:";

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents: String =
        fs::read_to_string(&args[1])
            .expect("should be able to read the file");
    let version: u64 = args[2].parse().expect("Should have a problem version");

    let lines: Vec<&str> =
        contents
            .trim()
            .split("\n")
            .collect();

    match version {
        1 => {
            let input = parse(&lines, false);
            let result: u64 =
                input.into_iter()
                    .map(|(total_time, record_distance)| count_winning_charging_times(total_time, record_distance))
                    .reduce(|a, b| a * b)
                    .unwrap();
            println!("Result is {result}");
        },
        2 => {
            let (total_time, record_distance) = parse(&lines, true)[0];
            let result: u64 = count_winning_charging_times(total_time, record_distance);
            println!("Result is {result}");
        }
        _ => panic!("Unknown version {version}")
    };
}

fn parse_number_line(line: &str) -> Vec<u64> {
    line
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().expect("Should be valid durations"))
        .collect()
}

fn parse(lines: &Vec<&str>, join_numbers: bool) -> Vec<(u64, u64)> {
    let durations = parse_number_line(&lines[0][TIME_PREFIX.len()..]);
    let distances = parse_number_line(&lines[1][DISTANCE_PREFIX.len()..]);
    if !join_numbers {
        return durations.into_iter().zip(distances).collect();
    }
    let duration: u64 = durations.iter().map(|n| n.to_string()).collect::<String>().parse().unwrap();
    let distance: u64 = distances.iter().map(|n| n.to_string()).collect::<String>().parse().unwrap();
    vec![(duration, distance)]
}

fn count_winning_charging_times(total_time: u64, record_distance: u64) -> u64 {
    (1..total_time)
        .map(|charging_time| run_race(charging_time, total_time))
        .filter(|distance| distance > &record_distance)
        .count() as u64
}

fn run_race(charging_time: u64, total_time: u64) -> u64 {
    let race_time: u64 = total_time - charging_time;
    race_time * charging_time
}