mod day01;

pub fn run(day: Option<&String>) {
    match day.as_deref().expect("Please specify a day!").as_str() {
        "day01" => day01::run(),
        _ => println!("Please specify a valid day (e.g., day01)"),
    }
}
