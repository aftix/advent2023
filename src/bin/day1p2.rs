use advent2023::parser::day1::parse_line;
use rayon::prelude::*;
use std::io;

pub fn main() {
    let lines: Vec<String> = io::stdin().lines().flatten().collect();
    let sum: i64 = lines
        .par_iter()
        .map(|line| parse_line(&line).ok().map(|(_, num)| num))
        .flatten()
        .sum();
    println!("Sum is {sum}");
}
