use advent2023::parser::parse_literal_digit;
use nom::{
    character::complete::alpha0,
    multi::{many1, many_till},
    IResult,
};
use rayon::prelude::*;
use std::io;

fn parse_glob_then_digit(input: &str) -> IResult<&str, i64> {
    let (rest, (_, digit)) = many_till(alpha0, parse_literal_digit)(input)?;
    Ok((rest, digit))
}

fn parse_line(input: &str) -> IResult<&str, i64> {
    let (_, first_digit) = parse_glob_then_digit(input)?;
    let (_, second_digit) = many1(parse_glob_then_digit)(input)?;
    let second_digit = second_digit.last().unwrap();
    Ok(("", first_digit * 10 + second_digit))
}

pub fn main() {
    let lines: Vec<String> = io::stdin().lines().map_while(Result::ok).collect();
    let sum: i64 = lines
        .par_iter()
        .map(|line| parse_line(line).ok().map(|(_, num)| num))
        .flatten()
        .sum();
    println!("Sum is {sum}");
}
