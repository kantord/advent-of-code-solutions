use std::collections::{HashMap, HashSet};

use crate::utils;

pub fn run() {
    let raw_lines = utils::read_lines("input/2023/day03.txt");
    let mut number_id = 0;
    let mut y = 0;
    let mut current_number = 0;
    let mut which_integer_at_position: HashMap<(i32, i32), i32> = HashMap::new();
    let mut number_values: HashMap<i32, u32> = HashMap::new();
    let mut symbol_positions: HashSet<(i32, i32)> = HashSet::new();

    for line in raw_lines {
        for (x, c) in line.chars().enumerate() {
            let position = (x as i32, y as i32);

            if c.is_digit(10) {
                current_number = current_number * 10 + c.to_digit(10).unwrap();
                which_integer_at_position.insert(position, number_id);
            } else if current_number != 0 {
                number_values.insert(number_id, current_number);
                number_id += 1;
                current_number = 0;
            }

            if !c.is_digit(10) && c != '.' {
                symbol_positions.insert(position);
            }
        }

        if current_number != 0 {
            number_values.insert(number_id, current_number);
            number_id += 1;
            current_number = 0;
        }

        y += 1;
    }

    let mut number_ids_adjacent_to_symbols: HashSet<i32> = HashSet::new();

    for (position, number_id) in which_integer_at_position.iter() {
        let (x, y) = position;
        for dx in -1..2 {
            for dy in -1..2 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                if symbol_positions.contains(&(x + dx, y + dy)) {
                    number_ids_adjacent_to_symbols.insert(*number_id);
                }
            }
        }
    }

    println!("{:?}", which_integer_at_position);
    println!("{:?}", number_values);
    println!("{:?}", symbol_positions);
    println!("{:?}", number_ids_adjacent_to_symbols);
    println!(
        "{:?}",
        number_ids_adjacent_to_symbols
            .iter()
            .map(|id| number_values.get(id).unwrap())
            .sum::<u32>()
    );
}
