use super::parse_int;
use crate::types::Day5;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, space1},
    multi::many1,
    sequence::{preceded, tuple},
    IResult,
};
use std::ops::Range;

fn parse_seeds(input: &str) -> IResult<&str, Vec<i64>> {
    let number_parser = preceded(space1, parse_int);
    let (rest, vec) = preceded(tag("seeds:"), many1(number_parser))(input)?;
    Ok((rest, vec))
}

// (out.0-to-out.1 map)
fn parse_map_name(input: &str) -> IResult<&str, (String, String)> {
    let (rest, (first, _, second, _, _)) =
        tuple((alpha1, tag("-to-"), alpha1, space1, tag("map:")))(input)?;
    Ok((rest, (first.to_owned(), second.to_owned())))
}

// Source range, destination range
fn parse_range(input: &str) -> IResult<&str, (Range<i64>, i64)> {
    let parse_src = preceded(space1, parse_int);
    let parse_len = preceded(space1, parse_int);
    let (rest, (dest_start, src_start, len)) = tuple((parse_int, parse_src, parse_len))(input)?;
    Ok((rest, (src_start..src_start + len, dest_start)))
}

pub fn parse_line(input: &str) -> IResult<&str, Day5> {
    if let Ok((rest, vec)) = parse_seeds(input) {
        return Ok((rest, Day5::Seeds(vec)));
    }

    if let Ok((rest, titles)) = parse_map_name(input) {
        return Ok((rest, Day5::MapTitle(titles.0, titles.1)));
    }

    let (rest, ranges) = parse_range(input)?;
    Ok((rest, Day5::Maps(ranges.0, ranges.1)))
}

#[cfg(test)]
mod test {
    use std::ops::Range;
    use test_case::test_case;

    #[test_case("seeds: 1" => vec![1])]
    #[test_case("seeds: 1 2" => vec![1, 2])]
    #[test_case("seeds: 1 2 3" => vec![1, 2, 3])]
    fn parse_seeds(input: &str) -> Vec<i64> {
        super::parse_seeds(input).unwrap().1
    }

    #[test_case("" ; "when empty")]
    #[test_case("1" ; "when no prefix")]
    #[test_case("seeds 1 2 3" ; "when no colon")]
    #[test_case("seeds: " ; "when no numbers")]
    #[should_panic]
    fn parse_seeds_panics(input: &str) {
        super::parse_seeds(input).unwrap();
    }

    #[test_case("seed-to-soil map:" => (String::from("seed"), String::from("soil")))]
    #[test_case("soil-to-seed map:" => (String::from("soil"), String::from("seed")))]
    fn parse_map_name(input: &str) -> (String, String) {
        super::parse_map_name(input).unwrap().1
    }

    #[test_case("" ; "when empty")]
    #[test_case("seed to soil map:" ; "when no hyphen")]
    #[test_case("seed-to soil map:" ; "when one hyphen")]
    #[test_case("seed-to-soil" ; "no map")]
    #[test_case("seed-to-soil map" ; "no colon")]
    #[should_panic]
    fn parse_map_name_panics(input: &str) {
        super::parse_map_name(input).unwrap();
    }

    #[test_case("1 1 1" => (1..2, 1))]
    #[test_case("1 2 4" => (2..6, 1))]
    #[test_case("2 4 1" => (4..5, 2))]
    fn parse_range(input: &str) -> (Range<i64>, i64) {
        super::parse_range(input).unwrap().1
    }

    #[test_case("" ; "when empty")]
    #[test_case("aoue" ; "when alphabetic")]
    #[test_case("1" ; "When single number")]
    #[test_case("1 2" ; "When two numbers")]
    #[test_case("1a2a3" ; "When separator not space")]
    #[should_panic]
    fn parse_range_panics(input: &str) {
        super::parse_range(input).unwrap();
    }
}
