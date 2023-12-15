use std::collections::HashMap;
use std::env;
use std::fs;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents: String =
        fs::read_to_string(&args[1])
            .expect("should be able to read the file");
    let version: u64 = args[2].parse().expect("Should have a problem version");
    let parsed_lines: Vec<(String, Vec<u32>)> = contents.trim().split("\n").map(parse_line).collect();

    match version {
        1 => {
            let result: u64 =
                parsed_lines.iter()
                    .map(|(row, groups)| count_matching_variations_dyn_prog(row, groups))
                    .sum();
            println!("Result is {result}");
        },
        2 => {
            let result: u64 =
            parsed_lines.iter()
                .map(|(row, groups)| {
                    let mut row_times5 = row.clone();
                    let mut groups_times5 = groups.clone();
                    for _ in 0..4 {
                        row_times5.push('?');
                        row_times5.push_str(row);
                        groups_times5.extend(groups);
                    }
                    count_matching_variations_dyn_prog(&row_times5, &groups_times5)
                })
                .sum();
            println!("Result is {result}");
        },
        _ => panic!("Unknown version {version}")
    };
}

fn parse_line(line: &str) -> (String, Vec<u32>) {
    let parts: Vec<&str> = line.split(' ').collect();
    let broken_spring_groups: Vec<u32> =
        parts[1].split(',').map(|e| e.parse::<u32>().expect("Should be a number")).collect();
    (String::from(parts[0]), broken_spring_groups)
}

fn count_matching_variations_dyn_prog(row: &str, groups: &[u32]) -> u64 {
    count_matching_variations_dyn_prog_rec(row, groups, &mut HashMap::new())
}

fn count_matching_variations_dyn_prog_rec(row: &str, groups: &[u32], cache: &mut HashMap<(String, Vec<u32>), u64>) -> u64 {
    let target_group_len: usize = match groups.first() {
        None => return if row.contains('#') { 0 } else { 1 },
        Some(&val) => val as usize
    };
    let cache_key: (String, Vec<u32>) = (String::from(row), Vec::from(groups));
    if cache.contains_key(&cache_key) {
        return cache.get(&cache_key).cloned().unwrap();
    }

    if row.is_empty() || row.len() < target_group_len {
        return 0;
    }
    if &row[0..1] == "." {
        let cache_value: u64 = count_matching_variations_dyn_prog_rec(&row[1..], groups, cache);
        cache.insert(cache_key, cache_value);
        return cache_value;
    }
    if &row[0..1] == "?" {
        let fork1 =
            count_matching_variations_dyn_prog_rec( &(String::from("#") + &row[1..]), groups, cache);
        let fork2 =
            count_matching_variations_dyn_prog_rec(&(String::from(".") + &row[1..]), groups, cache);
        return fork2 + fork1;
    }

    // Here the current character must be a #
    let fits_group = row[..target_group_len].chars().all(|c| c == '#' || c == '?');
    let is_followed_by_delimiter =
        row.chars().nth(target_group_len).map(|c| c == '.' || c == '?').unwrap_or(true);
    if !fits_group || !is_followed_by_delimiter {
        return 0;
    }

    let mut next_row = &row[target_group_len..];
    if !next_row.is_empty() {
        next_row = &next_row[1..];
    }
    let variations_keeping = count_matching_variations_dyn_prog_rec(next_row, &groups[1..], cache);
    variations_keeping
}

fn count_matching_variations_brute_force(row: &String, groups: &Vec<u32>) -> u32 {
    if !row.matches_groups(groups) {
        return 0;
    }
    row.fork()
        .map(|(var1, var2)| count_matching_variations_brute_force(&var1, groups) + count_matching_variations_brute_force(&var2, groups))
        .unwrap_or(1)
}

trait SpringRow {
    fn is_filled(&self) -> bool;
    fn fork(&self) -> Option<(Self, Self)> where Self: Sized;
    fn matches_groups(&self, broken_groups: &[u32]) -> bool;
}

impl SpringRow for String {
    fn is_filled(&self) -> bool { !self.contains('?') }

    fn fork(&self) -> Option<(Self, Self)> where Self: Sized {
        self
            .find(|c| c == '?')
            .map(|index| {
                let left = &self[..index];
                let right = &self[index + 1..];
                (left.to_string() + "." + right, left.to_string() + "#" + right)
            })
    }

    fn matches_groups(&self, broken_groups: &[u32]) -> bool {
        let prefix_pattern = "^[\\.|?]*".to_string();
        let middle_pattern: String = 
            broken_groups.iter()
                .map(|group_size| format!("[#|?]{{{group_size}}}"))
                .collect::<Vec<String>>()
                .join("[\\.|?]+");
        let suffix_pattern: &str = "[\\.|?]*$";
        let row_pattern = prefix_pattern + &middle_pattern + suffix_pattern;
        let regex = Regex::new(&row_pattern).unwrap();
        regex.is_match(self)
    }
}