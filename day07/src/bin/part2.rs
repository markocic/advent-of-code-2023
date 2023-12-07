use std::iter::zip;
use std::{fs, collections::HashMap, u8};
use std::cmp::Ordering;

fn main() {
    // let contents = fs::read_to_string("src/input.txt").expect("Could not read from file.");
    let contents = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483".to_string();

    let result = process(contents);
    println!("{}", result);
}

fn process(contents: String) -> usize {
    let mut hands = parse(contents);
    hands.sort();

    for hand in &hands {
        println!("{:?} {:?} {}", hand.r#type, hand.cards, hand.bid);
    }

    hands.iter()
        .enumerate()
        .map(|(i, h)| h.bid * (i + 1))
        .sum()
}

fn parse(contents: String) -> Vec<Hand> {
    let mut result = Vec::new();

    for line in contents.split("\n") {
        if line.is_empty() { continue; }
        
        result.push(construct(line.split(" ")
            .collect::<Vec<&str>>()));
        

    }

    result
}

fn construct(pair: Vec<&str>) -> Hand {
    Hand {
        r#type: get_type(pair[0].to_string()),
        cards: get_cards(pair[0].to_string()),
        bid: pair[1].parse().ok().unwrap()
    }

}

fn get_type(cards: String) -> Type {

    let mut counter: HashMap<char, u8> = HashMap::new();

    cards.chars().for_each(|card| *counter.entry(card.to_owned()).or_default() += 1);

    /* This is a bit agressive approach so I'm hoping I didn't just shoot myself in the foot here. */
    if counter.keys().len() == 1 { return Type::K5 };
    if counter.keys().len() == 2 { 
        if *counter.values().max().unwrap() == 4_u8 { return Type::K4 };
        return Type::FH;
    };

    if counter.keys().len() == 3 { 
        if *counter.values().max().unwrap() >= 3_u8 { return Type::K3 }
        return Type::P2;
    };

    if counter.keys().len() == 4 { return Type::P1 };
    if counter.keys().len() == 5 { return Type::HC };

    panic!("Could not parse card type. {}", cards);
}

fn get_cards(cards: String) -> Vec<Card> {
    cards.chars()
        .map(|c| {
            match c {
                '2' => Card::TWO,
                '3' => Card::THREE,
                '4' => Card::FOUR,
                '5' => Card::FIVE,
                '6' => Card::SIX,
                '7' => Card::SEVEN,
                '8' => Card::EIGHT,
                '9' => Card::NINE,
                'T' => Card::T,
                'J' => Card::J,
                'Q' => Card::Q,
                'K' => Card::K,
                'A' => Card::A,
                _   => panic!("Found illegal character: {}", c)
            }
        })
        .collect::<Vec<Card>>()
}

#[derive(Eq)]
struct Hand {
    r#type: Type,
    cards: Vec<Card>,
    bid: usize
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.r#type > other.r#type { return Ordering::Greater; }
        if self.r#type < other.r#type { return Ordering::Less; }

        for (i, j) in zip(&self.cards, &other.cards) {
            if *i == *j { continue; }
            if *i > *j { return Ordering::Greater; }
            if *i < *j { return Ordering::Less; }
        }
        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        for (i, j) in zip(&self.cards, &other.cards) {
            if *i != *j { return false; }
        }
        return true;
    }
}

#[derive(PartialEq, PartialOrd, Debug, Eq)]
enum Type {
    HC, P1, P2, K3, FH, K4, K5
}

#[derive(PartialEq, PartialOrd, Debug, Eq)]
enum Card {
    J, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE, T, Q, K, A
}
