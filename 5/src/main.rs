use std::env;
use std::fs;
use std::iter;
use std::ops::Range;

const SEEDS_PREFIX: &str = "seeds: ";

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

    let (seeds, map_chain) = parse(&lines);

    match version {
        1 => {
            let result: u64 =
                seeds.iter()
                    .flat_map(|seed| run_map_chain(&(*seed..(*seed + 1)), &map_chain))
                    .map(|r| r.start)
                    .min()
                    .unwrap();
            println!("Result1: {result}");
        },
        2 => {
            let mut range_seeds: Vec<Range<u64>> = Vec::new();
            for i in (0..seeds.len()).step_by(2) {
                let start: u64 = seeds[i];
                let length: u64 = seeds[i + 1];
                range_seeds.push(start..(start + length));
            }
            let result: u64 =
                range_seeds.iter()
                    .flat_map(|range| run_map_chain(range, &map_chain))
                    .map(|r| r.start)
                    .min()
                    .unwrap();
            println!("Result1: {result}");
        },
        _ => panic!("Unknown version {version}")
    };
}

fn parse(lines: &Vec<&str>) -> (Vec<u64>, Vec<RangeMap>) {
    let seeds: Vec<u64> = parse_seeds(lines[0]);
    let mut range_maps: Vec<RangeMap> = Vec::new();

    // The first 3 lines are seeds, whitespace and map header. The map data starts at line 4
    let mut begin: usize = 3;
    for i in begin..lines.len() {
        let line: &str = &lines[i];
        if !line.is_empty() { continue; }
        let value = parse_map(&lines, begin, i);
        range_maps.push(value);
        // Line i is empty and line i+1 is the next map's header.
        // So the next map's data starts at like i + 2
        begin = i + 2;
    }

    range_maps.push(parse_map(&lines, begin, lines.len()));
    (seeds, range_maps)
}

fn parse_seeds(line: &str) -> Vec<u64> {
    let line = line[SEEDS_PREFIX.len()..].trim();
    line
        .split(' ')
        .map(|s| s.parse().expect("Seed should be a number"))
        .collect()
}

fn parse_map(lines: &Vec<&str>, begin: usize, end: usize) -> RangeMap {
    let mut ranges: Vec<RangeMapEntry> = Vec::new();
    for line in lines[begin..end].iter() {
        let numbers: Vec<u64> =
            line
                .split(' ')
                .map(|s| s.parse().expect("Map line should contain only numbers"))
                .collect();
        let dst_begin: u64 = numbers[0];
        let src_begin: u64 = numbers[1];
        let length: u64 = numbers[2];
        ranges.push(RangeMapEntry::from(src_begin, dst_begin, length));
    }
    RangeMap { ranges }
}

fn run_map_chain(range: &Range<u64>, chain: &Vec<RangeMap>) -> Vec<Range<u64>> {
    chain.iter()
        .fold(
            vec![range.clone()],
            |acc, rm| acc.iter().flat_map(|r| rm.map_range(&r)).collect())
}

#[derive(Debug)]
struct RangeMap {
    ranges: Vec<RangeMapEntry>
}

impl RangeMap {
    fn map_range(&self, range: &Range<u64>) -> Vec<Range<u64>> {
        let adjusted_ranges: Vec<Range<u64>> = self.break_range(range);
        let mut result: Vec<Range<u64>> = Vec::new();
        for adjusted_range in adjusted_ranges {
            let rme_opt: Option<&RangeMapEntry> = self.ranges.iter().find(|e| e.intersects(&adjusted_range));
            match rme_opt {
                None => result.push(adjusted_range),
                Some(rme) => {
                    let intersection = rme.intersect(&adjusted_range).unwrap();
                    result.push(rme.map_range(&intersection));
                }
            }
        }

        result.sort_by(|a, b| a.start.cmp(&b.start));
        return result;
    }

    fn break_range(&self, range: &Range<u64>) -> Vec<Range<u64>> {
        let range_edges: Vec<u64> = self.get_sorted_range_edges();
        let mut dst: Vec<Range<u64>> = Vec::new();

        let mut remaining_range: Range<u64> = range.clone();
        let mut start: u64 = u64::MIN;
        for edge in range_edges.into_iter().chain(iter::once(u64::MAX)) {
            let current_range = Range { start, end: edge };
            if remaining_range.is_empty() {
                break;
            }
            if current_range.intersects(range) {
                let piece: Range<u64> = current_range.intersect(&remaining_range).unwrap();
                remaining_range =
                    remaining_range.minus(&piece)
                        .iter()
                        .find(|r| !r.is_empty())
                        .unwrap_or(&Range { start: piece.end, end: piece.end })
                        .clone();
                dst.push(piece);
            }
            start = edge;
        }

        return dst;
    }

    fn get_sorted_range_edges(&self) -> Vec<u64> {
        let mut sorted_ranges: Vec<RangeMapEntry> = self.ranges.clone();
        sorted_ranges.sort_by(|a, b| a.src.start.cmp(&b.src.start));
        return sorted_ranges.iter()
            .flat_map(|rme| vec![rme.as_range().start, rme.as_range().end])
            .collect();
    }
}

trait RangeOps {
    fn is_empty(&self) -> bool;
    fn contains(&self, value: &u64) -> bool;
    fn contains_range(&self, other: &Range<u64>) -> bool;
    fn intersects(&self, other: &Range<u64>) -> bool;
    fn is_to_the_right_of(&self, other: &Range<u64>) -> bool;
    fn is_to_the_left_of(&self, other: &Range<u64>) -> bool;
    fn intersect(&self, other: &Range<u64>) -> Option<Range<u64>>;
    fn minus(&self, other: &Range<u64>) -> Vec<Range<u64>>;
}

impl RangeOps for Range<u64> {
    fn is_empty(&self) -> bool { self.start == self.end }

    fn contains(&self, value: &u64) -> bool {
        self.contains(&value)
    }

    fn contains_range(&self, other: &Range<u64>) -> bool {
        let inclusive_end = other.end - 1;
        self.contains(&other.start) && self.contains(&inclusive_end)
    }

    fn intersects(&self, other: &Range<u64>) -> bool {
        // Empty ranges don't intersect anything
        if self.is_empty() || other.is_empty() { return false; }
        !self.is_to_the_left_of(other) && !self.is_to_the_right_of(other)
    }

    fn is_to_the_right_of(&self, other: &Range<u64>) -> bool {
        let left_complement: Range<u64> = u64::MIN..self.start;
        // Handles empty ranges and underflows gracefully
        if other.is_empty() {
            return left_complement.contains(&other.start);
        }
        let inclusive_end: u64 = other.end - 1;
        left_complement.contains(&other.start) && left_complement.contains(&inclusive_end)
    }

    fn is_to_the_left_of(&self, other: &Range<u64>) -> bool {
        let right_complement: Range<u64> = self.end..u64::MAX;
        if other.is_empty() {
            return right_complement.contains(&other.start);
        }
        let inclusive_end = other.end - 1;
        right_complement.contains(&other.start) && right_complement.contains(&inclusive_end)
    }

    fn intersect(&self, other: &Range<u64>) -> Option<Range<u64>> {
        if !self.intersects(other) {
            return None;
        }

        let inclusive_end = other.end - 1;
        if !self.contains(&other.start) && self.contains(&inclusive_end) {
            return Some(Range { start: self.start, end: other.end });
        }
        if self.contains(&other.start) && !self.contains(&inclusive_end) {
            return Some(Range { start: other.start, end: self.end });
        }

        if self.contains_range(other) {
            return Some(other.clone());
        }
        Some(Range { start: self.start, end: self.end })
    }

    fn minus(&self, other: &Range<u64>) -> Vec<Range<u64>> {
        if !self.intersects(other) {
            return vec![self.clone()];
        }

        if self.contains_range(other) {
            let left_part: Range<u64> = Range { start: self.start, end: other.start };
            let right_part: Range<u64> = Range { start: other.end, end: self.end };
            return vec![left_part, right_part];
        }
        if other.contains_range(self) {
            return Vec::new();
        }

        let intersection = self.intersect(other);
        self.minus(&intersection.unwrap())
    }
}

#[derive(Debug, Clone)]
struct RangeMapEntry {
    src: Range<u64>,
    dst: Range<u64>
}

impl RangeMapEntry {
    fn from(src_begin: u64, dst_begin: u64, length: u64) -> RangeMapEntry {
        RangeMapEntry {
            src: src_begin..src_begin + length,
            dst: dst_begin..dst_begin + length
        }
    }

    fn as_range(&self) -> &Range<u64> { &self.src }

    fn map(&self, src_value: u64) -> Option<u64> {
        if !self.contains(&src_value) {
            return None;
        }
        let offset: u64 = src_value - self.src.start;
        Some(self.dst.start + offset)
    }

    fn map_range(&self, range: &Range<u64>) -> Range<u64>{
        if !self.contains_range(range) {
            panic!("Cannot map a range that is not fully contained");
        }
        let start: u64 = self.map(range.start).unwrap();
        let end: u64 =
            if range.is_empty() { start } else { self.map(range.end - 1).unwrap() + 1 };
        return start..end
    }
}

impl RangeOps for RangeMapEntry {
    fn is_empty(&self) -> bool { self.as_range().is_empty() }
    fn contains(&self, value: &u64) -> bool { self.as_range().contains(value) }
    fn contains_range(&self, other: &Range<u64>) -> bool { self.as_range().contains_range(other) }
    fn intersects(&self, other: &Range<u64>) -> bool { self.as_range().intersects(other) }
    fn is_to_the_right_of(&self, other: &Range<u64>) -> bool { self.as_range().is_to_the_right_of(other) }
    fn is_to_the_left_of(&self, other: &Range<u64>) -> bool { self.as_range().is_to_the_left_of(other) }
    fn intersect(&self, other: &Range<u64>) -> Option<Range<u64>> { self.as_range().intersect(other) }
    fn minus(&self, other: &Range<u64>) -> Vec<Range<u64>> { self.as_range().minus(other) }
}