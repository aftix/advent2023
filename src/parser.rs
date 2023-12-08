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

pub fn parse_digit(input: &str) -> IResult<&str, i64> {
    alt((parse_spelled_digit, parse_literal_digit))(input)
}

pub fn parse_glob_then_digit(input: &str) -> IResult<&str, i64> {
    println!("Parsing glob and digit from {input}");
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

#[cfg(test)]
mod test {
    use nom::character::is_alphabetic;
    use ntest::test_case;

    #[test_case("1", 1)]
    #[test_case("2", 2)]
    #[test_case("3", 3)]
    #[test_case("4", 4)]
    #[test_case("5", 5)]
    #[test_case("6", 6)]
    #[test_case("7", 7)]
    #[test_case("8", 8)]
    #[test_case("9", 9)]
    #[test_case("", 0)]
    #[should_panic]
    #[test_case("one", 0)]
    #[should_panic]
    #[test_case("!@#$@#", 0)]
    #[should_panic]
    fn parse_literal_digit(input: &str, output: i64) {
        let (_, num) = super::parse_literal_digit(input).unwrap();
        assert_eq!(output, num);
    }

    #[test_case("one", 1)]
    #[test_case("two", 2)]
    #[test_case("three", 3)]
    #[test_case("four", 4)]
    #[test_case("five", 5)]
    #[test_case("six", 6)]
    #[test_case("seven", 7)]
    #[test_case("eight", 8)]
    #[test_case("nine", 9)]
    #[test_case("", 0)]
    #[should_panic]
    #[test_case("1", 0)]
    #[should_panic]
    #[test_case("!@#$@#", 0)]
    #[should_panic]
    fn parse_spelled_digit(input: &str, output: i64) {
        let (rest, num) = super::parse_spelled_digit(input).unwrap();
        assert_eq!(input.chars().last(), rest.chars().next());
        assert_eq!(output, num);
    }

    #[test_case("one", 1)]
    #[test_case("two", 2)]
    #[test_case("three", 3)]
    #[test_case("four", 4)]
    #[test_case("five", 5)]
    #[test_case("six", 6)]
    #[test_case("seven", 7)]
    #[test_case("eight", 8)]
    #[test_case("nine", 9)]
    #[test_case("1", 1)]
    #[test_case("2", 2)]
    #[test_case("3", 3)]
    #[test_case("4", 4)]
    #[test_case("5", 5)]
    #[test_case("6", 6)]
    #[test_case("7", 7)]
    #[test_case("8", 8)]
    #[test_case("9", 9)]
    #[test_case("", 0)]
    #[should_panic]
    #[test_case("!@#$@#", 0)]
    #[should_panic]
    fn parse_digit(input: &str, output: i64) {
        let (rest, num) = super::parse_digit(input).unwrap();
        if is_alphabetic(input.as_bytes()[0]) {
            assert_eq!(input.chars().last(), rest.chars().next());
        }
        assert_eq!(output, num);
    }

    #[test_case("one", 1, true)]
    #[test_case("two", 2, true)]
    #[test_case("three", 3, true)]
    #[test_case("four", 4, true)]
    #[test_case("five", 5, true)]
    #[test_case("six", 6, true)]
    #[test_case("seven", 7, true)]
    #[test_case("eight", 8, true)]
    #[test_case("nine", 9, true)]
    #[test_case("1", 1, false)]
    #[test_case("2", 2, false)]
    #[test_case("3", 3, false)]
    #[test_case("4", 4, false)]
    #[test_case("5", 5, false)]
    #[test_case("6", 6, false)]
    #[test_case("7", 7, false)]
    #[test_case("8", 8, false)]
    #[test_case("9", 9, false)]
    #[test_case("aeouaueone", 1, true)]
    #[test_case("aeuaeou3", 3, false)]
    #[test_case("", 0, false)]
    #[should_panic]
    #[test_case("!@#$@#", 0, false)]
    #[should_panic]
    fn parse_glob_then_digit(input: &str, output: i64, spelled: bool) {
        let (rest, num) = super::parse_glob_then_digit(input).unwrap();
        if spelled {
            assert_eq!(input.chars().last(), rest.chars().next());
        }
        assert_eq!(output, num);
    }

    #[test_case("12", 1, 2)]
    #[test_case("onetwo", 1, 2)]
    #[test_case("one2", 1, 2)]
    #[test_case("1two", 1, 2)]
    #[test_case("twone", 2, 1)]
    #[test_case(" 12", 1, 2)]
    #[test_case(" onetwo", 1, 2)]
    #[test_case(" one2", 1, 2)]
    #[test_case(" 1two", 1, 2)]
    #[test_case(" twone", 2, 1)]
    #[test_case("aaeaou12", 1, 2)]
    #[test_case("aaeaouonetwo", 1, 2)]
    #[test_case("aaeaouone2", 1, 2)]
    #[test_case("aaeaou1two", 1, 2)]
    #[test_case("aaeaoutwone", 2, 1)]
    #[test_case("12 ", 1, 2)]
    #[test_case("onetwo ", 1, 2)]
    #[test_case("one2 ", 1, 2)]
    #[test_case("1two ", 1, 2)]
    #[test_case("twone ", 2, 1)]
    #[test_case("12eaouaoeu", 1, 2)]
    #[test_case("onetwoaeoue", 1, 2)]
    #[test_case("one2aeoue", 1, 2)]
    #[test_case("1twoaeoue", 1, 2)]
    #[test_case("twoneaeoue", 2, 1)]
    #[test_case("1 2", 1, 2)]
    #[test_case("one two", 1, 2)]
    #[test_case("one 2", 1, 2)]
    #[test_case("1 two", 1, 2)]
    #[test_case("1aoeueoau2", 1, 2)]
    #[test_case("oneaoeueoautwo", 1, 2)]
    #[test_case("oneaoeueoau2", 1, 2)]
    #[test_case("1aoeueoautwo", 1, 2)]
    #[test_case(" 1 2 ", 1, 2)]
    #[test_case(" one two ", 1, 2)]
    #[test_case(" one 2 ", 1, 2)]
    #[test_case(" 1 two ", 1, 2)]
    #[test_case("", 0, 0)]
    #[should_panic]
    #[test_case("abc", 0, 0)]
    #[should_panic]
    #[test_case("one", 0, 0)]
    #[should_panic]
    fn multiple_digits(input: &str, first: i64, second: i64) {
        let (rest, one) = super::parse_glob_then_digit(input).unwrap();
        let (_, two) = super::parse_glob_then_digit(rest).unwrap();
        assert_eq!(first, one);
        assert_eq!(second, two);
    }
}
