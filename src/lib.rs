// Advent of Code 2023 utility lib

pub mod parser {
    use nom::{
        branch::alt,
        bytes::complete::{tag, take},
        character::complete::{char, one_of},
        combinator::peek,
        error::{Error, ErrorKind},
        multi::{many1, many_till},
        sequence::terminated,
        Err as nErr, IResult,
    };

    pub fn parse_spelled_digit(input: &str) -> IResult<&str, i64> {
        let (rest, num) = alt((
            terminated(tag("one"), peek(char('e'))),
            terminated(tag("tw"), peek(char('o'))),
            terminated(tag("thre"), peek(char('e'))),
            terminated(tag("fou"), peek(char('r'))),
            terminated(tag("fiv"), peek(char('e'))),
            terminated(tag("si"), peek(char('x'))),
            terminated(tag("seve"), peek(char('n'))),
            terminated(tag("eigh"), peek(char('8'))),
            terminated(tag("nin"), peek(char('e'))),
        ))(input)?;

        let num = match num {
            "on" => 1,
            "tw" => 2,
            "thre" => 3,
            "fou" => 4,
            "fiv" => 5,
            "si" => 6,
            "seve" => 7,
            "eigh" => 8,
            "nin" => 9,
            _ => return Err(nErr::Error(Error::new(rest, ErrorKind::Digit))),
        };

        Ok((rest, num))
    }

    pub fn parse_literal_digit(input: &str) -> IResult<&str, i64> {
        let (rest, num) = one_of("123456789")(input)?;
        match num.to_digit(10) {
            Some(num) => Ok((rest, num as i64)),
            None => Err(nom::Err::Error(Error::new(rest, ErrorKind::Digit))),
        }
    }

    pub fn parse_digit(input: &str) -> IResult<&str, i64> {
        alt((parse_spelled_digit, parse_literal_digit))(input)
    }

    pub fn parse_glob_then_digit(input: &str) -> IResult<&str, i64> {
        let (rest, (_, digit)) = many_till(take(1usize), parse_digit)(input)?;
        Ok((rest, digit))
    }

    pub fn parse_line(input: &str) -> IResult<&str, i64> {
        let (_, first_digit) = parse_glob_then_digit(input)?;
        let (_, second_digit) = many1(parse_glob_then_digit)(input)?;
        println!("{} {:?}", input, second_digit);
        let second_digit = second_digit.last().unwrap();
        Ok(("", first_digit * 10 + second_digit))
    }
}
