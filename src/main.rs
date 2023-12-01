mod utils;
mod year2019;
mod year2023;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args
        .get(1)
        .as_deref()
        .expect("specify a valid year and day!")
        .as_str()
    {
        "2019" => year2019::run(args.get(2)),
        "2023" => year2023::run(args.get(2)),
        _ => println!("Please specify a valid year and day (e.g., 2021 day01)"),
    }
}
