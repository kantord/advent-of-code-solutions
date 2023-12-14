use std::collections::HashMap;

use crate::utils;

pub fn run() {
    let raw_lines = utils::read_lines("input/2023/day08.txt");
    let instructions = &raw_lines[0];

    let nodes: HashMap<&str, (String, String)> = raw_lines[2..]
        .iter()
        .map(|line| {
            let mut parts = line.split(" = ");
            let node_name = parts.next().unwrap();
            let raw_children = parts.next().unwrap();

            let children: Vec<_> = raw_children
                .split(", ")
                .map(|child| child.split(", ").next().unwrap())
                .collect();

            let left = children[0].replace("(", "").clone();
            let right = children[1].replace(")", "").clone();

            (node_name, (left, right))
        })
        .collect();

    println!("{:?}", instructions);
    println!("{:?}", nodes);

    let mut steps = 0;
    let mut position = "AAA";

    loop {
        println!("{}: {}", steps, position);
        if position == "ZZZ" {
            break;
        }

        let c = instructions
            .chars()
            .nth(steps % instructions.len())
            .unwrap();

        steps += 1;

        match c {
            'L' => position = &nodes[position].0,
            'R' => position = &nodes[position].1,
            _ => panic!("Unknown instruction: {}", c),
        }
    }

    println!("{:?}", steps);
}
