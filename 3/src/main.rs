use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents: String =
        fs::read_to_string(&args[1])
            .expect("should be able to read the file");
    let version: u32 = args[2].parse().expect("Should have a problem version");

    let lines: Vec<&str> =
        contents
            .split("\n")
            .filter(|e| !e.is_empty())
            .collect();
    let mut grid = Grid::empty(lines.len() as i32, lines[0].len() as i32);

    for (i, line) in lines.iter().enumerate() {
        parse_line(line, i as i32, &mut grid);
    }
    
    match version {
        1 =>  println!("Result: {}", find_sum_of_part_numbers(&grid)),
        2 => println!("Result: {}", find_sum_of_gear_ratios(&grid)),
        _ => panic!("Unknown version {version}")
    }
}

fn parse_line(line: &str, line_number: i32, grid: &mut Grid) {
    let row = line_number;
    let mut current_number: String = String::new();

    for (col, c) in line.chars().enumerate() {
        let col = col as i32;
        if c >= '0' && c <= '9' {
            current_number += &c.to_string();
            continue;
        }
        if c != '.' {
            grid.add_symbol(Symbol { position: Point::from(col, row), value: c });
        }
        // Current char is not a digit, so maybe we just finished a number
        if !current_number.is_empty() {
            grid.add_number(build_number(&current_number, row, col - 1));
            current_number.clear();
        }
    }

    // If there's a number in the buffer the the last position of the grid finished a number
    if !current_number.is_empty() {
        grid.add_number(build_number(&current_number, row, (line.len() as i32) - 1));
        current_number.clear();
    }
}

fn build_number(raw: &String, row: i32, col: i32) -> Number {
    let value: i32 = raw.parse().expect("Should parse to a number");
    let positions: Vec<Point> =
        (0..raw.len())
            .map(|col_offset| Point::from(col - col_offset as i32, row))
            .collect();
    Number { value, positions }
}

fn find_sum_of_part_numbers(grid: &Grid) -> i32{
    let mut result: Vec<i32> = Vec::new();
    for number in grid.numbers.iter() {
        let value = number.value;
        let has_adjacent_symbol =
            number.get_adjacent().iter()
                .filter(|p| grid.contains(p))
                .map(|p| grid.get(p))
                .any(|o| o.as_ref().is_some_and(GridElement::is_symbol));
        if !has_adjacent_symbol {
            continue;
        }
        result.push(value);
    }
    result.iter().sum()
}

fn find_sum_of_gear_ratios(grid: &Grid) -> i32 {
    let mut sum: i32 = 0;
    for Symbol { value, position } in grid.symbols.iter() {
        if *value != '*' {
            continue;
        }
        let adjacent_numbers: HashSet<Number> =
            position.get_adjacent().iter()
                .filter(|p| grid.contains(p))
                .map(|p| grid.get(p))
                .filter(|o| o.as_ref().is_some_and(GridElement::is_number))
                .map(|o| o.unwrap().get_number())
                .collect();
        if adjacent_numbers.len() != 2 {
            continue;
        }
        let gear_ratio: i32 = adjacent_numbers.iter().map(|n| n.value).reduce(|a, b| a * b).unwrap();
        sum += gear_ratio;
    }
    sum
}

struct Grid {
    rows: i32,
    cols: i32,
    data: Vec<Option<GridElement>>,
    numbers: Vec<Number>,
    symbols: Vec<Symbol>
}

impl Grid {
    fn empty(rows: i32, cols: i32)-> Grid {
        let mut data = Vec::new();
        for _i in 0..rows * cols {
            data.push(Option::None);
        }
        Grid { rows, cols, data, numbers: Vec::new(), symbols: Vec::new() }
    }

    fn get(&self, p: &Point) -> Option<GridElement> {
        match &self.data[self.to_index(p)] {
            None => None,
            Some(value) => Some(value.clone())
        }
    }

    fn contains(&self, p: &Point) -> bool {
        p.x >= 0 && p.x < self.cols && p.y >= 0 && p.y < self.rows
    }

    fn add_symbol(&mut self, elem: Symbol) {
        let index = self.to_index(&elem.position);
        self.symbols.push(elem.clone());
        self.data[index] = Some(GridElement::Symbol(elem));
    }

    fn add_number(&mut self, elem: Number) {
        self.numbers.push(elem.clone());
        for p in elem.positions.iter() {
            let index = self.to_index(p);
            self.data[index] = Some(GridElement::Number(elem.clone()));
        }
    }

    fn to_index(&self, p: &Point) -> usize {
        let row = p.y as usize;
        let col = p.x as usize;
        row * self.cols as usize + col
    }

    // fn to_string(&self) -> String{
    //     let mut result = String::new();
    //     let mut row = 0;
    //     let mut col = 0;
    //     while row < self.rows {
    //         while col < self.cols {
    //             match self.get(&Point { x: col, y: row }) {
    //                 Option::None => {
    //                     result += ".";
    //                 }
    //                 Option::Some(elem) => {
    //                     match elem {
    //                         GridElement::Symbol(s) => {
    //                             result += &s.value.to_string();
    //                         }
    //                         GridElement::Number(n) => {
    //                             let value_str = n.value.to_string();
    //                             result += &value_str;
    //                             col += value_str.len() as i32;
    //                             continue;
    //                         }
    //                     }
    //                 }
    //             }
    //             col += 1;
    //         }
    //         result += "\n";
    //         col = 0;
    //         row += 1;
    //     }
    //     result
    // }
}

#[derive(Clone)]
enum GridElement {
    Symbol(Symbol),
    Number(Number)
}

impl GridElement {
    fn is_symbol(&self) -> bool {
        match self {
            GridElement::Symbol(_) => true,
            _ => false
        }
    }

    fn is_number(&self) -> bool {
        match self {
            GridElement::Number(_) => true,
            _ => false
        }
    }

    fn get_number(&self) -> Number {
        match self {
            GridElement::Number(value) => value.clone(),
            _ => panic!("Expected a number")
        }
    }
}

#[derive(Clone, Debug)]
struct Symbol { position: Point, value: char }

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct Number { value: i32, positions: Vec<Point> }

impl Number {
    fn get_adjacent(&self) -> HashSet<Point> {
        let mut result: HashSet<Point> = HashSet::new();
        self.positions.iter()
            .flat_map(Point::get_adjacent)
            .for_each(|p| { result.insert(p); });
        self.positions.iter().for_each(|p| { result.remove(p); });
        result
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Point { x: i32, y: i32 }

impl Point {
    fn from(x: i32, y: i32) -> Point { Point { x, y } }

    fn get_adjacent(&self) -> HashSet<Point> {
        let mut result = HashSet::new();
        for x in -1..2 {
            for y in -1..2 {
                if x == 0 && y == 0 {
                    continue;
                }
                result.insert(Point { x: self.x + x, y: self.y + y });
            }
        }
        result
    }
}
