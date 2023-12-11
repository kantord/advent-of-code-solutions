use std::iter::zip;

use crate::utils;

pub fn run() {
    let raw_lines = utils::read_lines("input/2023/day06.txt");

    let times: Vec<u32> = raw_lines[0]
        .replace("Time:", "")
        .trim()
        .split_whitespace()
        .map(|number| number.parse().unwrap())
        .collect();

    let distances: Vec<u32> = raw_lines[1]
        .replace("Distance:", "")
        .trim()
        .split_whitespace()
        .map(|number| number.parse().unwrap())
        .collect();

    println!("{:?}", times);
    println!("{:?}", distances);

    let mut ways_to_beat_the_record: Vec<i32> = Vec::new();

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

    println!("{:?}", ways_to_beat_the_record);
    println!("{:?}", ways_to_beat_the_record.iter().product::<i32>());
}
