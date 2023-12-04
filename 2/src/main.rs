use core::panic;
use std::env;
use std::fs;
use std::cmp;

const PREFIX: usize = "Game ".len();
const MAX_HAND: Hand = Hand(12, 13, 14);

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents: String =
        fs::read_to_string(&args[1])
            .expect("should be able to read the file");
    let version: u32 = args[2].parse().expect("Should have a problem version");

    let parsed_lines =
        contents
            .split("\n")
            .filter(|e| !e.is_empty())
            .map(|x| parse_line(x));

    match version {
        1 => {
            let result =
                parsed_lines
                    .filter(is_allowed)
                    .fold(0_u32, |acc, game| acc + game.id);
            println!("Result is {result}");
        }
        2 => {
            let result: u32 =
                parsed_lines
                    .map(|game| {
                        let (max_red, max_green, max_blue) =
                            game.hands.iter()
                                .fold(
                                    (0_u32, 0_u32, 0_u32),
                                    |acc, hand|
                                        (cmp::max(acc.0, hand.0), cmp::max(acc.1, hand.1), cmp::max(acc.2, hand.2)));
                        max_red * max_green * max_blue
                    })
                    .sum();
            println!("Result is {result}");
        }
        _ => panic!("Unknown problem version")
    }
}

fn parse_line(line: &str) -> Game {
    let line = line.trim();
    let line = &line[PREFIX..];
    let id_end = line.find(':').expect("Should contain : after id");
    let id: u32 = line[..id_end].parse().expect("Id should be integer");

    let line = &line[id_end + 2..];
    let hands: Vec<Hand> = line
        .split(';')
        .map(parse_hand)
        .collect();

    Game { id, hands }
}

fn parse_hand(hand: &str) -> Hand {
    hand
        .split(',')
        .map(|x| x.trim())
        .map(|x| {
            let whitespace = x.find(' ').expect("Should contain whitespace before color");
            let cubes: u32 = x[..whitespace].parse().expect("Should have integer number of cubes");
            let color: &str = &x[whitespace + 1..];
            match color {
                "red" => Hand(cubes, 0, 0),
                "green" => Hand(0, cubes, 0),
                "blue" => Hand(0, 0, cubes),
                _ => panic!("Unknown color {color}")
            }
        })
        .reduce(|a, b| Hand(a.0 + b.0, a.1 + b.1, a.2 + b.2))
        .expect("Should have some cubes shown in a hand")
}

fn is_allowed(game: &Game) -> bool {
    game.hands.iter().all(|hand| hand.0 <= MAX_HAND.0 && hand.1 <= MAX_HAND.1 && hand.2 <= MAX_HAND.2)
}

struct Hand(u32, u32, u32);
struct Game { id: u32, hands: Vec<Hand> }