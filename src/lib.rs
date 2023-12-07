// Advent of Code 2023 utility lib

pub mod parser {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::one_of,
        error::{self, ErrorKind},
        multi::{many1, many_till},
        IResult,
    };

    pub fn parse_spelled_digit(input: &str) -> IResult<&str, i64> {
        let (rest, num) = alt((
            tag("one"),
            tag("two"),
            tag("three"),
            tag("four"),
            tag("five"),
            tag("six"),
            tag("seven"),
            tag("eight"),
            tag("nine"),
        ))(input)?;

        let num = match num {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            _ => return Err(nom::Err::Failure(error::Error::new(rest, ErrorKind::Alt))),
        };

        Ok((rest, num))
    }

    pub fn parse_literal_digit(input: &str) -> IResult<&str, i64> {
        let (rest, num) = one_of("123456789")(input)?;
        match num.to_digit(10) {
            Some(num) => Ok((rest, num as i64)),
            None => Err(nom::Err::Failure(error::Error::new(rest, ErrorKind::Char))),
        }
    }

    pub fn parse_digit(input: &str) -> IResult<&str, i64> {
        alt((parse_spelled_digit, parse_literal_digit))(input)
    }

    pub fn parse_glob(input: &str) -> IResult<&str, &str> {
        let mut char_iter = input.char_indices();
        let mut ch = char_iter.next();
        let mut res = parse_digit(char_iter.as_str());
        while res.is_err() && ch.is_some() {
            ch = char_iter.next();
            res = parse_digit(char_iter.as_str());
        }

        if ch.is_some() {
            ch = char_iter.next();
        }

        if let Some((idx, _)) = ch {
            Ok((&input[idx..], &input[..idx]))
        } else {
            Ok(("", input))
        }
    }

    pub fn parse_glob_then_digit(input: &str) -> IResult<&str, i64> {
        let (rest, (_, digit)) = many_till(parse_glob, parse_digit)(input)?;
        Ok((rest, digit))
    }

    pub fn parse_line(input: &str) -> IResult<&str, i64> {
        let (_, first_digit) = parse_glob_then_digit(input)?;
        let (_, second_digit) = many1(parse_glob_then_digit)(input)?;
        let second_digit = second_digit.last().unwrap();
        println!("{input} {:?}", first_digit * 10 + second_digit);
        Ok(("", first_digit * 10 + second_digit))
    }
}
