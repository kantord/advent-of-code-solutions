mod year2022;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args
        .get(1)
        .as_deref()
        .expect("specify a valid year and day!")
        .as_str()
    {
        "2022" => year2022::run(args.get(2)),
        _ => println!("Please specify a valid year and day (e.g., 2021 day01)"),
    }
}
