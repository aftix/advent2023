use super::parse_int;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, space0, space1},
    combinator::eof,
    multi::many_till,
    sequence::{preceded, tuple},
    IResult,
};

pub fn parse_card_id(input: &str) -> IResult<&str, u32> {
    let (rest, (_, _, num, _)) = tuple((tag("Card"), space1, parse_int, char(':')))(input)?;
    Ok((rest, num as u32))
}

pub fn parse_numbers(input: &str) -> IResult<&str, Vec<i64>> {
    let p_int = preceded(space0, parse_int);
    let p_end = preceded(space0, alt((tag("|"), eof)));
    let (rest, (vec, _)) = many_till(p_int, p_end)(input)?;
    Ok((rest, vec))
}

pub fn parse_line(input: &str) -> IResult<&str, (u32, Vec<i64>, Vec<i64>)> {
    let (rest, id) = parse_card_id(input)?;
    let (rest, winners) = parse_numbers(rest)?;
    let (rest, card_nums) = parse_numbers(rest)?;
    Ok((rest, (id, winners, card_nums)))
}
