use advent2023::parser::day2::parse_line;
use rayon::prelude::*;
use std::io;

const NUM_RED: i64 = 12;
const NUM_GREEN: i64 = 13;
const NUM_BLUE: i64 = 14;

pub fn main() {
    let lines: Vec<String> = io::stdin().lines().map_while(Result::ok).collect();
    let sum: i64 = lines
        .into_par_iter()
        .map(|line| parse_line(&line).ok().map(|(_, game)| game))
        .flatten()
        .filter(|game| {
            game.sets
                .par_iter()
                .find_any(|set| set.red > NUM_RED || set.green > NUM_GREEN || set.blue > NUM_BLUE)
                .is_none()
        })
        .map(|game| game.id)
        .sum();
    println!("Sum is {sum}");
}
