use super::parse_digit;
use nom::{
    bytes::complete::take,
    multi::{many1, many_till},
    IResult,
};

pub fn parse_glob_then_digit(input: &str) -> IResult<&str, i64> {
    let (rest, (_, digit)) = many_till(take(1usize), parse_digit)(input)?;
    Ok((rest, digit))
}

pub fn parse_line(input: &str) -> IResult<&str, i64> {
    let (_, first_digit) = parse_glob_then_digit(input)?;
    let (_, second_digit) = many1(parse_glob_then_digit)(input)?;
    let second_digit = second_digit.last().unwrap();
    Ok(("", first_digit * 10 + second_digit))
}

#[cfg(test)]
mod test {
    use ntest::test_case;

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
