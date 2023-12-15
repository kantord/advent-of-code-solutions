use std::collections::HashMap;

use crate::utils;

fn gcd(a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut b = b;

    while a != b {
        if a > b {
            a = a - b;
        } else {
            b = b - a;
        }
    }

    a
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

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

    // println!("{:?}", instructions);
    // println!("{:?}", nodes);

    let mut positions: Vec<String> = nodes
        .keys()
        .filter(|node| node.ends_with("A"))
        .map(|s| s.to_string())
        .collect();
    let mut steps: Vec<u64> = vec![0; positions.len()];

    for i in 0..positions.len() {
        loop {
            if positions[i].ends_with("Z") {
                break;
            }

            let c = instructions
                .chars()
                .nth((steps[i] % (instructions.len() as u64)) as usize)
                .unwrap();

            steps[i] += 1;

            positions[i] = match c {
                'L' => nodes[positions[i].as_str()].0.clone(),
                'R' => nodes[positions[i].as_str()].1.clone(),
                _ => panic!("Unknown instruction: {}", c),
            }
        }
    }

    println!("{:?}", steps);
    println!("{:?}", steps[1..].iter().fold(steps[0], |a, b| lcm(a, *b)));
}
