use super::parse_int;
use crate::types::RaceLabel;
use nom::{
    branch::alt,
    bytes::complete::tag_no_case,
    character::complete::{char, space0},
    multi::many1,
    sequence::{preceded, terminated},
    IResult,
};

fn parse_label(input: &str) -> IResult<&str, RaceLabel> {
    let alt_parser = alt((tag_no_case("Time"), tag_no_case("Distance")));
    let (rest, label) = terminated(alt_parser, char(':'))(input)?;

    match label.to_lowercase().as_str() {
        "time" => Ok((rest, RaceLabel::Time)),
        "distance" => Ok((rest, RaceLabel::Distance)),
        _ => unreachable!(),
    }
}

pub fn parse_line(input: &str) -> IResult<&str, (RaceLabel, Vec<i64>)> {
    let (rest, label) = parse_label(input)?;
    let (rest, vec) = many1(preceded(space0, parse_int))(rest)?;
    Ok((rest, (label, vec)))
}

#[cfg(test)]
mod test {
    use crate::types::RaceLabel;
    use test_case::test_case;

    #[test_case("Time:" => RaceLabel::Time)]
    #[test_case("Distance:" => RaceLabel::Distance)]
    #[test_case("TIME:" => RaceLabel::Time ; "when time capitalized")]
    #[test_case("DISTANCE:" => RaceLabel::Distance ; "when distance capitalized")]
    #[test_case("time:" => RaceLabel::Time ; "when time lowercase")]
    #[test_case("distance:" => RaceLabel::Distance ; "when distance lowercase")]
    fn parse_label(input: &str) -> RaceLabel {
        super::parse_label(input).unwrap().1
    }

    #[test_case("" ; "when empty")]
    #[test_case("Time" ; "when time no colon")]
    #[test_case("Distance" ; "when distance no colon")]
    #[test_case(" Time:" ; "when leading space")]
    #[test_case(":" ; "when no label")]
    #[should_panic]
    fn parse_label_panics(input: &str) {
        super::parse_label(input).unwrap();
    }
}
