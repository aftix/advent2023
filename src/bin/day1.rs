use std::io;

use nom::{
    character::complete::{alpha0, one_of},
    multi::many1,
    sequence::preceded,
    IResult,
};

fn parse_line(input: &str) -> IResult<&str, i64> {
    let (_, first_digit) = preceded(alpha0, one_of("0123456789"))(input)?;
    let (_, second_digit) = many1(preceded(alpha0, one_of("0123456789")))(input)?;
    let second_digit = second_digit.last().unwrap();
    Ok((
        "",
        (first_digit.to_digit(10).unwrap() * 10 + second_digit.to_digit(10).unwrap()) as i64,
    ))
}

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
