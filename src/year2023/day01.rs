use crate::utils;

pub fn run() {
    let raw_lines = utils::read_lines("input/2023/day01.txt");
    let lines: Vec<Vec<char>> = raw_lines
        .into_iter()
        .map(|line| line.chars().filter(|c| c.is_digit(10)).collect())
        .collect();

    let numbers: Vec<i32> = lines
        .iter()
        .map(|line| {
            let first = line.first().unwrap();
            let last = line.last().unwrap();
            let mut result = first.to_string();
            result.push(*last);

            result.parse().unwrap()
        })
        .collect();

    let answer1: i32 = numbers.iter().sum();

    println!("{:?}", answer1);
}
