use advent2023::parser::parse_line;
use std::io;

pub fn main() {
    let stdin = io::stdin();
    let sum: i64 = stdin
        .lines()
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .map(|line| parse_line(&line).ok().map(|(_, num)| num))
        .flatten()
        .sum();
    println!("Sum is {sum}");
}
