use std::iter::zip;

use crate::utils;

fn get_ways_to_beat_the_record(times: Vec<i64>, distances: Vec<i64>) -> Vec<i64> {
    let mut ways_to_beat_the_record: Vec<i64> = Vec::new();

    for (time, record_distance) in zip(times, distances) {
        let mut ways = 0;
        for hold_time in 0..=time {
            let speed = hold_time;
            let remaining_time = time - hold_time;
            let distance = speed * remaining_time;

            if distance > record_distance {
                ways += 1;
            }
        }
        ways_to_beat_the_record.push(ways)
    }

    return ways_to_beat_the_record;
}

pub fn run() {
    let raw_lines = utils::read_lines("input/2023/day06.txt");

    // star 1
    let times: Vec<i64> = raw_lines[0]
        .replace("Time:", "")
        .trim()
        .split_whitespace()
        .map(|number| number.parse().unwrap())
        .collect();

    let distances: Vec<i64> = raw_lines[1]
        .replace("Distance:", "")
        .trim()
        .split_whitespace()
        .map(|number| number.parse().unwrap())
        .collect();

    println!("{:?}", times);
    println!("{:?}", distances);

    println!(
        "{:?}",
        get_ways_to_beat_the_record(times, distances)
            .iter()
            .product::<i64>()
    );

    // star 2
    let times2 = vec![raw_lines[0]
        .replace("Time:", "")
        .replace(" ", "")
        .trim()
        .parse::<i64>()
        .unwrap()];

    println!("{:?}", times2);

    let distances2 = vec![raw_lines[1]
        .replace("Distance:", "")
        .replace(" ", "")
        .trim()
        .parse::<i64>()
        .unwrap()];

    println!("{:?}", distances2);

    println!("{:?}", get_ways_to_beat_the_record(times2, distances2));
}
