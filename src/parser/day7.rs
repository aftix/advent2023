use super::parse_int;

use nom::{
    bytes::complete::take_while_m_n, character::complete::space1, sequence::preceded, IResult,
};

fn parse_hand(input: &str) -> IResult<&str, &str> {
    let predicate = |c: char| (c.is_alphabetic() && c.is_uppercase()) || c.is_ascii_digit();
    let (rest, hand) = take_while_m_n(5, 5, predicate)(input)?;
    Ok((rest, hand))
}

pub fn parse_line(input: &str) -> IResult<&str, (&str, i64)> {
    let (rest, hand) = parse_hand(input)?;
    let (rest, bid) = preceded(space1, parse_int)(rest)?;
    Ok((rest, (hand, bid)))
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    #[test_case("AAAAA" => "AAAAA")]
    #[test_case("12345" => "12345")]
    #[test_case("AAAAAA" => "AAAAA")]
    #[test_case("123456" => "12345")]
    #[test_case("123AA" => "123AA")]
    #[test_case("123AAA" => "123AA")]
    fn parse_hand(hand: &str) -> &str {
        super::parse_hand(hand).unwrap().1
    }

    #[test_case("" ; "when empty")]
    #[test_case("ABC" ; "when too little")]
    #[test_case(" 12345" ; "when starting space")]
    #[test_case("_1234" ; "when non alphanum")]
    #[should_panic]
    fn parse_hand_panics(hand: &str) {
        super::parse_hand(hand).unwrap();
    }
}
