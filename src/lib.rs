// Advent of Code 2023 utility lib

pub mod parser {
    use nom::{
        character::complete::{alpha0, one_of},
        multi::many1,
        sequence::preceded,
        IResult,
    };

    pub fn parse_line(input: &str) -> IResult<&str, i64> {
        let (_, first_digit) = preceded(alpha0, one_of("0123456789"))(input)?;
        let (_, second_digit) = many1(preceded(alpha0, one_of("0123456789")))(input)?;
        let second_digit = second_digit.last().unwrap();
        Ok((
            "",
            (first_digit.to_digit(10).unwrap() * 10 + second_digit.to_digit(10).unwrap()) as i64,
        ))
    }
}
