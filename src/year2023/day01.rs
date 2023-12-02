use crate::utils;

pub fn run() {
    let raw_lines = utils::read_lines("input/2023/day01.txt");
    // let lines: Vec<Vec<char>> = raw_lines
    //     .iter()
    //     .map(|line| line.chars().filter(|c| c.is_digit(10)).collect())
    //     .collect();

    // let numbers: Vec<i32> = lines
    //     .iter()
    //     .map(|line| {
    //         let first = line.first().unwrap();
    //         let last = line.last().unwrap();
    //         let mut result = first.to_string();
    //         result.push(*last);

    //         result.parse().unwrap()
    //     })
    //     .collect();

    // let answer1: i32 = numbers.iter().sum();

    // println!("{:?}", answer1);

    let numbers_with_numbers_spelled_out: Vec<i32> = raw_lines
        .iter()
        .map(|line| {
            let mut numbers: Vec<i32> = vec![];
            let replace_rules = [
                ("1", 1),
                ("2", 2),
                ("3", 3),
                ("4", 4),
                ("5", 5),
                ("6", 6),
                ("7", 7),
                ("8", 8),
                ("9", 9),
                ("eight", 8),
                ("seven", 7),
                ("three", 3),
                ("four", 4),
                ("five", 5),
                ("nine", 9),
                ("one", 1),
                ("two", 2),
                ("six", 6),
            ];

            for i in 0..line.len() {
                for (source, target) in replace_rules {
                    if line[i..].starts_with(source) {
                        numbers.push(target);
                        break;
                    }
                }
            }

            return numbers.first().unwrap() * 10 + numbers.last().unwrap();
        })
        .collect();

    let answer2: i32 = numbers_with_numbers_spelled_out.iter().sum();

    println!("{:?}", answer2);
}
