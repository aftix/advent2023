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
    use test_case::test_case;

    #[test_case("one", true => 1)]
    #[test_case("two", true => 2)]
    #[test_case("three", true => 3)]
    #[test_case("four", true => 4)]
    #[test_case("five", true => 5)]
    #[test_case("six", true => 6)]
    #[test_case("seven", true => 7)]
    #[test_case("eight", true => 8)]
    #[test_case("nine", true => 9)]
    #[test_case("1", false => 1)]
    #[test_case("2", false => 2)]
    #[test_case("3", false => 3)]
    #[test_case("4", false => 4)]
    #[test_case("5", false => 5)]
    #[test_case("6", false => 6)]
    #[test_case("7", false => 7)]
    #[test_case("8", false => 8)]
    #[test_case("9", false => 9)]
    #[test_case("aeouaueone", true => 1)]
    #[test_case("aeuaeou3", false => 3)]
    fn parse_glob_then_digit(input: &str, spelled: bool) -> i64 {
        let (rest, num) = super::parse_glob_then_digit(input).unwrap();
        if spelled {
            assert_eq!(input.chars().last(), rest.chars().next());
        }
        num
    }

    #[test_case("" ; "when empty")]
    #[test_case("!@#$@#" ; "when symbols")]
    #[should_panic]
    fn parse_glob_then_digit_panics(input: &str) {
        super::parse_glob_then_digit(input).unwrap();
    }

    #[test_case("12" => (1, 2) ; "when numeric")]
    #[test_case("onetwo" => (1, 2) ; "when alphabetic")]
    #[test_case("one2" => (1, 2) ; "when alphabetic numeric")]
    #[test_case("1two" => (1, 2) ; "when numeric alphabetic")]
    #[test_case("twone" => (2, 1) ; "when ovelapping")]
    #[test_case(" 12" => (1, 2) ; "when numeric prefix space")]
    #[test_case(" onetwo" => (1, 2) ; "when alphabetic prefix space")]
    #[test_case(" one2" => (1, 2) ; "when alphabetic numeric prefix space")]
    #[test_case(" 1two" => (1, 2) ; "when numeric alphabetic prefix space")]
    #[test_case(" twone" => (2, 1) ; "when overlapping prefix space")]
    #[test_case("aaeaou12" => (1, 2) ; "when numeric prefix")]
    #[test_case("aaeaouonetwo" => (1, 2) ; "when alphabetic prefix")]
    #[test_case("aaeaouone2" => (1, 2) ; "when alphabetic numeric prefix")]
    #[test_case("aaeaou1two" => (1, 2) ; "when numeric alphabetic prefix")]
    #[test_case("aaeaoutwone" => (2, 1) ; "when overlapping prefix")]
    #[test_case("12 " => (1, 2) ; "when numeric suffix space")]
    #[test_case("onetwo " => (1, 2) ; "when alphabetic suffix space")]
    #[test_case("one2 " => (1, 2) ; "when alphabetic numeric suffix space")]
    #[test_case("1two " => (1, 2) ; "when numeric alphabetic suffix space")]
    #[test_case("twone " => (2, 1) ; "when overlapping suffix space")]
    #[test_case("12eaouaoeu" => (1, 2) ; "when numeric suffix")]
    #[test_case("onetwoaeoue" => (1, 2) ; "when alphabetic suffix")]
    #[test_case("one2aeoue" => (1, 2) ; "when alphabetic numeric suffix")]
    #[test_case("1twoaeoue" => (1, 2) ; "when numeric alphabetic suffix")]
    #[test_case("twoneaeoue" => (2, 1) ; "when overlapping suffix")]
    #[test_case("1 2" => (1, 2) ; "when numeric spaced")]
    #[test_case("one two" => (1, 2) ; "when alphabetic spaced")]
    #[test_case("one 2" => (1, 2) ; "when alphabetic numeric spaced")]
    #[test_case("1 two" => (1, 2) ; "when numeric alphabetic spaced")]
    #[test_case("1aoeueoau2" => (1, 2) ; "when numeric with inbetween")]
    #[test_case("oneaoeueoautwo" => (1, 2) ; "when alphabetic with inbetween")]
    #[test_case("oneaoeueoau2" => (1, 2) ; "when alphabetic numeric with inbetween")]
    #[test_case("1aoeueoautwo" => (1, 2) ; "when numeric alphabetic with inbetween")]
    #[test_case(" 1 2 " => (1, 2) ; "when numeric fully spaced")]
    #[test_case(" one two " => (1, 2) ; "when alphabetic fully spaced")]
    #[test_case(" one 2 " => (1, 2) ; "when alphabetic numeric fully spaced")]
    #[test_case(" 1 two " => (1, 2) ; "when numeric alphabetic fully spaced")]
    fn multiple_digits(input: &str) -> (i64, i64) {
        let (rest, one) = super::parse_glob_then_digit(input).unwrap();
        let (_, two) = super::parse_glob_then_digit(rest).unwrap();
        (one, two)
    }

    #[test_case("" ; "when empty")]
    #[test_case("abc" ; "when letters")]
    #[should_panic]
    fn multiple_digits_panics(input: &str) {
        let (rest, _) = super::parse_glob_then_digit(input).unwrap();
        super::parse_glob_then_digit(rest).unwrap();
    }
}
