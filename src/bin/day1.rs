use std::io;

use nom::{
    bytes::complete::is_a, character::complete::alpha0, multi::many1, sequence::preceded, IResult,
};

fn parse_line(input: &str) -> IResult<&str, i64> {
    let (_, first_digit) = preceded(alpha0, is_a("0123456789"))(input)?;
    let (_, second_digit) = many1(preceded(alpha0, is_a("0123456789")))(input)?;
    let second_digit = second_digit.last().unwrap();

    Ok((
        "",
        first_digit
            .chars()
            .chain(second_digit.chars())
            .map(|c| c.to_digit(10).unwrap() as i64)
            .fold(0 as i64, |acc, elem| acc * 10 + elem),
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
