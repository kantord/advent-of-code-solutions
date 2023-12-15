use crate::utils;

fn calculate_next_step(sequence: &Vec<i64>) -> Vec<i64> {
    let mut result = vec![];
    for i in 0..(sequence.len() - 1) {
        result.push(sequence[i + 1] - sequence[i]);
    }

    result
}

fn calculate_until_final_row(sequence: &Vec<i64>) -> Vec<Vec<i64>> {
    let mut result: Vec<Vec<i64>> = vec![sequence.clone()];

    loop {
        let last_row = result.last().unwrap();
        if last_row.iter().all(|v| v == &0) {
            break;
        }
        let next_row = calculate_next_step(&last_row);

        result.push(next_row);
    }

    result
}

fn add_last_column(sequences: Vec<Vec<i64>>) -> Vec<i64> {
    let mut results: Vec<i64> = vec![0];

    for sequence in sequences.iter().rev().skip(1) {
        println!(
            "sequence {:?} results {:?}",
            sequence.last().unwrap(),
            results.last().unwrap()
        );
        let next_row = sequence.last().unwrap() + results.last().unwrap();
        results.push(next_row);
    }

    results
}

pub fn run() {
    let raw_lines = utils::read_lines("input/2023/day09.txt");
    let initial_sequences: Vec<Vec<i64>> = raw_lines
        .into_iter()
        .map(|line| line.split(" ").map(|c| c.parse().unwrap()).collect())
        .collect();

    let mut _final = 0;
    for line in initial_sequences.iter() {
        let until_final_row = calculate_until_final_row(line);
        println!("until_final_row {:?}", until_final_row);
        let results = add_last_column(until_final_row);
        _final += results.last().unwrap();
        println!("results {:?}", results);
        println!("---")
    }

    println!("{:?}", _final);
}
