use crate::utils;

fn calculate_fuel_requirement(mass: i32) -> i32 {
    let fuel_for_mass = mass / 3 - 2;
    if fuel_for_mass < 0 {
        return 0;
    }

    fuel_for_mass + calculate_fuel_requirement(fuel_for_mass)
}

pub fn run() {
    let raw_lines = utils::read_lines("input/2019/day01.txt");
    let lines: Vec<i32> = raw_lines
        .into_iter()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();

    let result: i32 = lines.into_iter().map(calculate_fuel_requirement).sum();
    println!("{:?}", result);
}
