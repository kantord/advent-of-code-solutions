use std::fs::read_to_string;

pub fn read_lines(filename: &str) -> Vec<String> {
    // naive implementation from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
    // this implementation should be more convenient because the input files are usually
    // small, and there would be little real benefit to loading them in a more efficient
    // way

    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
