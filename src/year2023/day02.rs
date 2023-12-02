use std::collections::HashMap;

use crate::utils;

fn parse_game_id(raw_game_id: &str) -> i32 {
    raw_game_id.replace("Game ", "").parse().unwrap()
}

fn parse_draw(raw_draw: &str) -> HashMap<&str, i32> {
    let mut result = HashMap::new();
    raw_draw.split(", ").for_each(|raw_item| {
        let mut split_raw_item = raw_item.split(" ");
        let raw_number = split_raw_item.next().unwrap();
        let number: i32 = raw_number.parse().unwrap();
        let key = split_raw_item.next().unwrap();

        result.insert(key, number);
    });

    result
}

fn parse_game_draws(raw_draws: &str) -> Vec<HashMap<&str, i32>> {
    raw_draws.split("; ").map(parse_draw).collect()
}

fn parse_game(raw_game: &str) -> (i32, Vec<HashMap<&str, i32>>) {
    let mut raw_parts = raw_game.split(": ");
    let raw_game_id = raw_parts.next().unwrap();
    let raw_draws = raw_parts.next().unwrap();
    let parsed_game_id = parse_game_id(raw_game_id);
    let parsed_draws = parse_game_draws(raw_draws);

    (parsed_game_id, parsed_draws)
}

pub fn run() {
    let raw_lines = utils::read_lines("input/2023/day02.txt");
    let maximum_cubes = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let parsed_games: Vec<_> = raw_lines.iter().map(|line| parse_game(line)).collect();

    // star 1
    let possible_games_star_1 = parsed_games
        .iter()
        .filter(|(_, draws)| {
            let mut possible = true;

            for draw in draws {
                for (color, count) in draw.iter() {
                    let maximum_for_color = maximum_cubes.get(color).unwrap();

                    if count > maximum_for_color {
                        possible = false;
                    }
                }
            }

            possible
        })
        .collect::<Vec<_>>();

    println!(
        "{:?}",
        possible_games_star_1.iter().map(|(x, _)| x).sum::<i32>()
    );

    // star 2
    let powers_star_2: Vec<i32> = parsed_games
        .iter()
        .map(|(_, draws)| {
            let mut minimum_cubes = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

            for draw in draws {
                for (color, count) in draw.iter() {
                    let minimum_for_color = minimum_cubes.get(color).unwrap();

                    if count > minimum_for_color {
                        minimum_cubes.insert(color, *count);
                    }
                }
            }

            minimum_cubes.values().product()
        })
        .collect();

    println!("{:?}", powers_star_2.iter().sum::<i32>())
}
