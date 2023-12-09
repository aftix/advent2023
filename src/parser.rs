use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, one_of},
    combinator::peek,
    error::{Error, ErrorKind},
    sequence::terminated,
    Err as nErr, IResult,
};

pub mod day1;
pub mod day2;
pub mod day3;

pub fn parse_spelled_digit(input: &str) -> IResult<&str, i64> {
    let (rest, num) = alt((
        terminated(tag("on"), peek(char('e'))),
        terminated(tag("tw"), peek(char('o'))),
        terminated(tag("thre"), peek(char('e'))),
        terminated(tag("fou"), peek(char('r'))),
        terminated(tag("fiv"), peek(char('e'))),
        terminated(tag("si"), peek(char('x'))),
        terminated(tag("seve"), peek(char('n'))),
        terminated(tag("eigh"), peek(char('t'))),
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

pub fn parse_int(input: &str) -> IResult<&str, i64> {
    let (rest, num) = digit1(input)?;

    match num.parse() {
        Ok(num) => Ok((rest, num)),
        Err(_) => Err(nErr::Error(Error::new(rest, ErrorKind::Digit))),
    }
}

pub fn parse_digit(input: &str) -> IResult<&str, i64> {
    alt((parse_spelled_digit, parse_literal_digit))(input)
}

#[cfg(test)]
mod test {
    use nom::character::is_alphabetic;
    use test_case::test_case;

    #[test_case("1", 1)]
    #[test_case("2", 2)]
    #[test_case("3", 3)]
    #[test_case("4", 4)]
    #[test_case("5", 5)]
    #[test_case("6", 6)]
    #[test_case("7", 7)]
    #[test_case("8", 8)]
    #[test_case("9", 9)]
    fn parse_literal_digit(input: &str, output: i64) {
        let (_, num) = super::parse_literal_digit(input).unwrap();
        assert_eq!(output, num);
    }

    #[test_case("" ; "when empty")]
    #[test_case("one" ; "when written in english")]
    #[test_case("!@#$@#" ; "when symbols")]
    #[should_panic]
    fn parse_literal_digit_panics(input: &str) {
        super::parse_literal_digit(input).unwrap();
    }

    #[test_case("1", 1 => "")]
    #[test_case("2", 2 => "")]
    #[test_case("3", 3 => "")]
    #[test_case("4", 4 => "")]
    #[test_case("5", 5 => "")]
    #[test_case("6", 6 => "")]
    #[test_case("7", 7 => "")]
    #[test_case("8", 8 => "")]
    #[test_case("9", 9 => "")]
    #[test_case("9a", 9 => "a")]
    #[test_case("134", 134 => "")]
    fn parse_int(input: &str, output: i64) -> &str {
        let (rest, num) = super::parse_int(input).unwrap();
        assert_eq!(output, num);
        rest
    }

    #[test_case("" ; "when empty")]
    #[test_case("one" ; "when written in english")]
    #[test_case("!@#$@#" ; "when symbols")]
    #[should_panic]
    fn parse_int_panics(input: &str) {
        super::parse_int(input).unwrap();
    }

    #[test_case("one" => 1)]
    #[test_case("two" => 2)]
    #[test_case("three" => 3)]
    #[test_case("four" => 4)]
    #[test_case("five" => 5)]
    #[test_case("six" => 6)]
    #[test_case("seven" => 7)]
    #[test_case("eight" => 8)]
    #[test_case("nine" => 9)]
    fn parse_spelled_digit(input: &str) -> i64 {
        let (rest, num) = super::parse_spelled_digit(input).unwrap();
        assert_eq!(input.chars().last(), rest.chars().next());
        num
    }

    #[test_case("" ; "when empty")]
    #[test_case("1" ; "when numeric")]
    #[test_case("!@#$@#" ; "when symbols")]
    #[should_panic]
    fn parse_spelled_digit_panics(input: &str) {
        super::parse_spelled_digit(input).unwrap();
    }

    #[test_case("one" => 1)]
    #[test_case("two" => 2)]
    #[test_case("three" => 3)]
    #[test_case("four" => 4)]
    #[test_case("five" => 5)]
    #[test_case("six" => 6)]
    #[test_case("seven" => 7)]
    #[test_case("eight" => 8)]
    #[test_case("nine" => 9)]
    #[test_case("1" => 1)]
    #[test_case("2" => 2)]
    #[test_case("3" => 3)]
    #[test_case("4" => 4)]
    #[test_case("5" => 5)]
    #[test_case("6" => 6)]
    #[test_case("7" => 7)]
    #[test_case("8" => 8)]
    #[test_case("9" => 9)]
    fn parse_digit(input: &str) -> i64 {
        let (rest, num) = super::parse_digit(input).unwrap();
        if is_alphabetic(input.as_bytes()[0]) {
            assert_eq!(input.chars().last(), rest.chars().next());
        }
        num
    }

    #[test_case("" ; "when empty")]
    #[test_case("!@#$@#" ; "when symbols")]
    #[should_panic]
    fn parse_digit_panics(input: &str) {
        super::parse_digit(input).unwrap();
    }
}
