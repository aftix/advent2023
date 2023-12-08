use advent2023::parser::day2::parse_line;
use advent2023::types::{Game, GameSet};
use rayon::prelude::*;
use std::io;

fn get_power(game: Game) -> i64 {
    let maximums = game.sets.into_par_iter().reduce(
        || GameSet {
            red: 0,
            green: 0,
            blue: 0,
        },
        |mut acc, set| {
            acc.red = acc.red.max(set.red);
            acc.green = acc.green.max(set.green);
            acc.blue = acc.blue.max(set.blue);
            acc
        },
    );
    maximums.red * maximums.green * maximums.blue
}

pub fn main() {
    let lines: Vec<String> = io::stdin().lines().flatten().collect();
    let sum: i64 = lines
        .into_par_iter()
        .map(|line| parse_line(&line).ok().map(|(_, game)| game))
        .flatten()
        .map(get_power)
        .sum();
    println!("Sum of powers is {sum}");
}
