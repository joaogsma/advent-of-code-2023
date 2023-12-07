use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::fs;

const JOKER_VALUE: u8 = 0;

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents: String =
        fs::read_to_string(&args[1])
            .expect("should be able to read the file");
    let version: u64 = args[2].parse().expect("Should have a problem version");

    let joker_rule = match version { 1 => false, 2 => true, _ => panic!("Unknown version {version}") };

    let input: Vec<(Hand, u32)> =
        contents
            .trim()
            .split("\n")
            .map(|line| parse_line(line, joker_rule))
            .collect();

    let mut sorted_input = input;
    sorted_input.sort();

    let result: u32 =
        sorted_input.iter()
            .enumerate()
            .map(|(i, (_hand, bid))| bid * (i as u32 + 1))
            .sum();
    println!("Result is {result}");
}

fn parse_line(line: &str, joker_rule: bool) -> (Hand, u32) {
    let parts: Vec<&str> = line.trim().split(' ').collect();
    let mut cards: [u8; 5] = [0, 0, 0, 0, 0];
    let bid: u32 = parts[1].parse().expect("Bid should be number");
    for (i, c) in parts[0].bytes().enumerate() {
        if c >= b'2' && c <= b'9' {
            cards[i] = c - b'0';
            continue;
        }
        match c {
            b'T' => cards[i] = 10,
            b'J' => cards[i] = if !joker_rule { 11 } else { JOKER_VALUE },
            b'Q' => cards[i] = 12,
            b'K' => cards[i] = 13,
            b'A' => cards[i] = 14,
            _ => panic!("Invalid card")
        }
    }
    return (Hand::from(cards), bid);
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Hand {
    HighCard([u8; 5]),
    OnePair([u8; 5]),
    TwoPair([u8; 5]),
    ThreeOfAKind([u8; 5]),
    FullHouse([u8; 5]),
    FourOfAKind([u8; 5]),
    FiveOfAKind([u8; 5])
}

impl Hand {
    fn from(cards: [u8; 5]) -> Hand {
        let mut histogram: HashMap<u8, u8> = HashMap::new();
        for card in cards {
            match histogram.get(&card) {
                None => { histogram.insert(card, 1); }
                Some(count) => { histogram.insert(card, count + 1); }
            }
        }
        let joker_count = histogram.remove(&JOKER_VALUE).unwrap_or(0);
        let max_count = *histogram.values().max().unwrap_or(&0);
        let pair_count = histogram.values().filter(|x| **x == 2).count() as u8;
        if max_count == 5 || joker_count == 5 || histogram.values().any(|x| x + joker_count == 5) {
            return Hand::FiveOfAKind(cards);
        }
        if max_count == 4 || histogram.values().any(|x| x + joker_count == 4) {
            return Hand::FourOfAKind(cards);
        }
        if (max_count == 3 && pair_count == 1) || (pair_count == 2 && joker_count == 1) {
            return Hand::FullHouse(cards);
        }
        if max_count == 3 || histogram.values().any(|x| x + joker_count == 3) {
            return Hand::ThreeOfAKind(cards);
        }
        if pair_count == 2 {
            return Hand::TwoPair(cards);
        }
        if max_count == 2 || histogram.values().any(|x| x + joker_count == 2) {
            return Hand::OnePair(cards);
        }
        return Hand::HighCard(cards);
    }

    // Assigns a number value (1-7) to the hand type and concatenates its binary value it with the cards's score
    fn score_hand(hand: &Hand) -> u64 {
        match hand {
            Hand::HighCard(cards) => (1_u64 << 40) + Hand::score_cards(cards),
            Hand::OnePair(cards) => (2_u64 << 40) + Hand::score_cards(cards),
            Hand::TwoPair(cards) => (3_u64 << 40) + Hand::score_cards(cards),
            Hand::ThreeOfAKind(cards) => (4_u64 << 40) + Hand::score_cards(cards),
            Hand::FullHouse(cards) => (5_u64 << 40) + Hand::score_cards(cards),
            Hand::FourOfAKind(cards) => (6_u64 << 40) + Hand::score_cards(cards),
            Hand::FiveOfAKind(cards) => (7_u64 << 40) + Hand::score_cards(cards)
        }
    }

    // Concatenates the binary values of all numbers
    fn score_cards(cards: &[u8; 5]) -> u64 {
        ((cards[0] as u64) << 32)
            + ((cards[1] as u64) << 24)
            + ((cards[2] as u64) << 16)
            + ((cards[3] as u64) << 8)
            + cards[4] as u64
    }

    // fn to_string(&self) -> String {
    //     const MAPPING: [char; 15] = ['J', 'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];
    //     let mut result = String::new();
    //     let (hand_type, card_values) =
    //         match self {
    //             Hand::HighCard(cards) => ("HighCard", cards),
    //             Hand::OnePair(cards) => ("OnePair", cards),
    //             Hand::TwoPair(cards) => ("TwoPair", cards),
    //             Hand::ThreeOfAKind(cards) => ("ThreeOfAKind", cards),
    //             Hand::FullHouse(cards) => ("FullHouse", cards),
    //             Hand::FourOfAKind(cards) => ("FourOfAKind", cards),
    //             Hand::FiveOfAKind(cards) => ("FiveOfAKind", cards)
    //         };
    //     result += hand_type;
    //     result += "(";
    //     for value in card_values {
    //         result.push(MAPPING[*value as usize]);
    //     }
    //     result += ")";
    //     result
    // }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering { Hand::score_hand(self).cmp(&Hand::score_hand(other)) }
}