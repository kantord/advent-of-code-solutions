use crate::utils;

fn run_program(source_code: &str, noun: i32, verb: i32) -> i32 {
    let mut memory: Vec<i32> = source_code
        .split(",")
        .into_iter()
        .map(|x| x.parse().unwrap())
        .collect();

    memory[1] = noun;
    memory[2] = verb;
    let mut pointer = 0;

    loop {
        match memory[pointer] {
            1 => {
                let target_index = memory[pointer + 3] as usize;
                memory[target_index] =
                    memory[memory[pointer + 1] as usize] + memory[memory[pointer + 2] as usize];
            }
            2 => {
                let target_index = memory[pointer + 3] as usize;
                memory[target_index] =
                    memory[memory[pointer + 1] as usize] * memory[memory[pointer + 2] as usize];
            }
            99 => break,
            _ => panic!("Unknown opcode"),
        }
        pointer += 4;
    }

    memory[0]
}

pub fn run() {
    let raw_lines = utils::read_lines("input/2019/day02.txt");
    let first_line = &raw_lines[0];

    for noun in 0..100 {
        for verb in 0..100 {
            if run_program(first_line, noun, verb) == 19690720 {
                println!("{}", 100 * noun + verb);
                return;
            }
        }
    }
}
