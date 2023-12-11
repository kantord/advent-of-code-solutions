use crate::utils;
use std::{collections::HashMap, iter::zip};

fn count_items_in_hand(hand: Vec<i32>) -> Vec<i32> {
    let mut result = HashMap::new();

    for item in hand {
        let count = result.entry(item).or_insert(0);
        *count += 1;
    }

    let mut finalresult: Vec<i32> = result.values().map(|x| x.clone()).collect();

    finalresult.sort();
    finalresult.reverse();

    finalresult
}

pub fn run() {
    let raw_lines = utils::read_lines("input/2023/day07.txt");
    let value_scores: HashMap<char, i32> = [
        ('A', 12),
        ('K', 11),
        ('Q', 10),
        ('J', 9),
        ('T', 8),
        ('9', 7),
        ('8', 6),
        ('7', 5),
        ('6', 4),
        ('5', 3),
        ('4', 2),
        ('3', 1),
        ('2', 0),
    ]
    .into_iter()
    .collect();

    let mut parsed_hands: Vec<_> = raw_lines
        .iter()
        .map(|line| {
            let mut split_line = line.split_whitespace();

            let hand: Vec<i32> = split_line
                .next()
                .unwrap()
                .chars()
                .map(|c| *value_scores.get(&c).unwrap())
                .collect();
            let bid: i32 = split_line.next().unwrap().parse().unwrap();

            (hand, bid)
        })
        .collect();

    parsed_hands.sort_by_key(|(hand, _)| hand.clone());
    parsed_hands.sort_by_key(|(hand, _)| {
        let hand_counts = count_items_in_hand(hand.clone());

        if hand_counts.len() >= 2 {
            return hand_counts[1];
        } else {
            return 0;
        }
    });
    parsed_hands.sort_by_key(|(hand, _)| {
        let hand_counts = count_items_in_hand(hand.clone());

        if hand_counts.len() >= 1 {
            return hand_counts[0];
        } else {
            return 0;
        }
    });

    println!("{:?}", parsed_hands);
    println!(
        "{:?}",
        parsed_hands
            .iter()
            .enumerate()
            .map(|(index, (_, bid))| (index as i32 + 1) * bid)
            .sum::<i32>()
    );
}
