use super::parse_literal_digit;
use crate::types::{Game, GameSet};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, space0},
    multi::{many0, many1},
    sequence::{preceded, terminated, tuple},
    IResult,
};

pub fn parse_game_id(input: &str) -> IResult<&str, i64> {
    terminated(preceded(tag("Game "), parse_literal_digit), tag(":"))(input)
}

pub fn parse_game_cubes(input: &str) -> IResult<&str, GameSet> {
    let (rest, num) = terminated(parse_literal_digit, char(' '))(input)?;
    let (rest, color) = alt((tag("red"), tag("green"), tag("blue")))(rest)?;

    let mut set = GameSet::default();
    match color {
        "red" => set.red = num,
        "green" => set.green = num,
        "blue" => set.blue = num,
        _ => unreachable!(),
    };
    Ok((rest, set))
}

pub fn parse_game_set(input: &str) -> IResult<&str, GameSet> {
    let (rest, sets) = many1(tuple((space0, parse_game_cubes, tag(","))))(input)?;

    let set = sets
        .into_iter()
        .map(|(_, set, _)| set)
        .fold(GameSet::default(), |mut acc, set| {
            acc.red += set.red;
            acc.green += set.green;
            acc.blue += set.blue;
            acc
        });
    Ok((rest, set))
}

pub fn parse_game_rounds(input: &str) -> IResult<&str, Vec<GameSet>> {
    let (rest, mut vec) = many0(terminated(parse_game_set, tag(";")))(input)?;
    let (rest, last) = parse_game_set(rest)?;
    vec.push(last);
    Ok((rest, vec))
}

pub fn parse_line(input: &str) -> IResult<&str, Game> {
    let (rest, (id, sets)) = tuple((parse_game_id, parse_game_rounds))(input)?;
    Ok((rest, Game { id, sets }))
}
