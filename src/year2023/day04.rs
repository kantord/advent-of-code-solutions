use std::collections::{HashMap, HashSet};

use crate::utils;

#[derive(Debug)]
struct Card {
    id: i32,
    numbers: HashSet<i32>,
    winning_numbers: HashSet<i32>,
}

impl Card {
    fn get_winning_numbers(&self) -> HashSet<&i32> {
        self.numbers
            .intersection(&self.winning_numbers)
            .into_iter()
            .collect()
    }

    fn get_matching_numbers(&self) -> i32 {
        self.get_winning_numbers().len() as i32
    }

    fn get_score(&self) -> i32 {
        let size_of_intersection = self.get_matching_numbers() as u32;

        if size_of_intersection == 0 {
            return 0;
        }

        i32::pow(2, size_of_intersection - 1)
    }
}

#[derive(Debug)]
struct CardSet {
    cards: HashMap<i32, Card>,
}

impl CardSet {
    fn build_cardset(cards: Vec<Card>) -> CardSet {
        let mut results = HashMap::new();
        for card in cards {
            results.insert(card.id, card);
        }

        CardSet { cards: results }
    }
}

fn get_card_counts(cardset: &CardSet, card_id: i32) -> HashMap<i32, i32> {
    let mut card_counts: HashMap<i32, i32> = HashMap::new();
    let card = cardset.cards.get(&card_id).unwrap();

    for winning_card_id in card.get_winning_numbers() {
        *card_counts.entry(*winning_card_id).or_insert(0) += 1;

        for (key, value) in get_card_counts(cardset, *winning_card_id) {
            *card_counts.entry(key).or_insert(0) += value;
        }
    }

    card_counts
}

fn parse_numbers(raw_numbers: &str) -> HashSet<i32> {
    raw_numbers
        .trim()
        .split_whitespace()
        .map(|raw_number| raw_number.trim().parse().unwrap())
        .collect()
}

fn parse_card(raw_card: &str) -> Card {
    let raw_parts: Vec<_> = raw_card.split(": ").collect();
    let raw_id = raw_parts[0];
    let raw_card_data = raw_parts[1];
    let card_id: i32 = raw_id.replace("Card", "").trim().parse().unwrap();
    let raw_numbers_parts: Vec<_> = raw_card_data.split(" | ").collect();

    Card {
        id: card_id,
        numbers: parse_numbers(raw_numbers_parts[0]),
        winning_numbers: parse_numbers(raw_numbers_parts[1]),
    }
}

pub fn run() {
    let raw_lines = utils::read_lines("input/2023/day04.txt");
    let cards: Vec<_> = raw_lines.iter().map(|line| parse_card(line)).collect();

    for card in cards.iter() {
        println!("{:?}", card);
        println!("Score: {}", card.get_score());
    }
    println!("{:?}", cards.iter().map(|c| c.get_score()).sum::<i32>());

    let cardset = CardSet::build_cardset(cards);
    println!(
        "{:?}",
        cardset
            .cards
            .values()
            .into_iter()
            .map(|card| get_card_counts(&cardset, card.id).values().sum::<i32>())
            .sum::<i32>()
    );
}
