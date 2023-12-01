use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents: String =
        fs::read_to_string(&args[1])
            .expect("should be able to read the file");
    let lines: Vec<&str> =
        contents
        .split("\n")
        .filter(|e| !e.is_empty())
        .collect();
    let include_words: bool =
        if args[2] == "1" { false } else if args[2] == "2" { true } else { panic!("Unknown variation {}", args[2]) };

    let mut sum: u32 = 0;
    for line in lines {
        sum += extract_digits(line, include_words) as u32;
    }
    println!("Result2: {}", sum);
}

fn extract_digits(line: &str, include_words: bool) -> u8 {
    let mut digits: Vec<(usize, u8)> = Vec::new();
    for digit in 0..10 {
        let pattern = digit.to_string();
        find_first_and_last(line, pattern.as_str(), digit, &mut digits);
    }
    if include_words {
        find_first_and_last(line, "one", 1, &mut digits);
        find_first_and_last(line, "two", 2, &mut digits);
        find_first_and_last(line, "three", 3, &mut digits);
        find_first_and_last(line, "four", 4, &mut digits);
        find_first_and_last(line, "five", 5, &mut digits);
        find_first_and_last(line, "six", 6, &mut digits);
        find_first_and_last(line, "seven", 7, &mut digits);
        find_first_and_last(line, "eight", 8, &mut digits);
        find_first_and_last(line, "nine", 9, &mut digits);
    }

    let first_digit: u8 = digits.iter().min_by(|a, b| a.0.cmp(&b.0)).unwrap().1;
    let last_digit: u8 = digits.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap().1;
    first_digit * 10 + last_digit
}

fn find_first_and_last(line: &str, pattern: &str, value: u8, dst: &mut Vec<(usize, u8)>) {
    let first_match = line.find(pattern);
    let last_match = line.rfind(pattern);
    if first_match.is_some() {
        dst.push((first_match.unwrap(), value));
    }
    if last_match.is_some() {
        dst.push((last_match.unwrap(), value));
    }
}
