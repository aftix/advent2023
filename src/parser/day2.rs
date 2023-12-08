use super::parse_int;
use crate::types::{Game, GameSet};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, space0},
    multi::many0,
    sequence::{preceded, terminated, tuple},
    IResult,
};

pub fn parse_game_id(input: &str) -> IResult<&str, i64> {
    terminated(preceded(tag("Game "), parse_int), tag(":"))(input)
}

pub fn parse_game_cubes(input: &str) -> IResult<&str, GameSet> {
    let (rest, num) = terminated(parse_int, char(' '))(input)?;
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
    let (rest, mut sets) = many0(tuple((space0, parse_game_cubes, tag(","))))(input)?;
    let (rest, (_, last)) = tuple((space0, parse_game_cubes))(rest)?;
    sets.push(("", last, ""));

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

#[cfg(test)]
mod test {
    use test_case::test_case;

    #[test_case("Game 0:" => (0, ""))]
    #[test_case("Game 10:" => (10, ""))]
    #[test_case("Game 0:a" => (0, "a"))]
    #[test_case("Game 10:a" => (10, "a"))]
    fn game_id(input: &str) -> (i64, &str) {
        let (rest, num) = super::parse_game_id(input).unwrap();
        (num, rest)
    }

    #[test_case("Game 0" ; "when no colon")]
    #[test_case("Game " ; "when no number")]
    #[test_case("" ; "when empty")]
    #[should_panic]
    fn game_id_panics(input: &str) {
        super::parse_game_id(input).unwrap();
    }

    #[test_case("1 red" => ((1, 0, 0), "") ; "when 1 red")]
    #[test_case("1 green" => ((0, 1, 0), "") ; "when 1 green")]
    #[test_case("1 blue" => ((0, 0, 1), "") ; "when 1 blue")]
    #[test_case("10 red" => ((10, 0, 0), "") ; "when 10 red")]
    #[test_case("10 green" => ((0, 10, 0), "") ; "when 10 green")]
    #[test_case("10 blue" => ((0, 0, 10), "") ; "when 10 blue")]
    #[test_case("1 red, " => ((1, 0, 0), ", ") ; "when trailing comma")]
    fn game_cubes(input: &str) -> ((i64, i64, i64), &str) {
        let (rest, set) = super::parse_game_cubes(input).unwrap();
        ((set.red, set.green, set.blue), rest)
    }

    #[test_case("" ; "when empty")]
    #[test_case("a.eueou" ; "when nonsense")]
    #[should_panic]
    fn game_cubes_panics(input: &str) {
        super::parse_game_cubes(input).unwrap();
    }

    #[test_case("1 red" => ((1, 0, 0), "") ; "when 1 red")]
    #[test_case("1 green" => ((0, 1, 0), "") ; "when 1 green")]
    #[test_case("1 blue" => ((0, 0, 1), "") ; "when 1 blue")]
    #[test_case("10 red" => ((10, 0, 0), "") ; "when 10 red")]
    #[test_case("10 green" => ((0, 10, 0), "") ; "when 10 green")]
    #[test_case("10 blue" => ((0, 0, 10), "") ; "when 10 blue")]
    #[test_case("1 red, 1 blue" => ((1, 0, 1), "") ; "when red blue")]
    #[test_case("1 red, 1 green" => ((1, 1, 0), "") ; "when red green")]
    #[test_case("10 green, 3 blue" => ((0, 10, 3), "") ; "when green blue")]
    #[test_case("1 red;" => ((1, 0, 0), ";") ; "when trailing semicolon")]
    fn game_set(input: &str) -> ((i64, i64, i64), &str) {
        let (rest, set) = super::parse_game_set(input).unwrap();
        ((set.red, set.green, set.blue), rest)
    }

    #[test_case("" ; "when empty")]
    #[test_case("a.eueou" ; "when nonsense")]
    #[should_panic]
    fn game_set_panics(input: &str) {
        super::parse_game_set(input).unwrap();
    }

    use crate::types::GameSet;

    #[test_case("1 red" => vec![GameSet {red: 1, ..Default::default()}] ; "rounds 1 red")]
    #[test_case("1 green" => vec![GameSet {green: 1, ..Default::default()}] ; "rounds 1 green")]
    #[test_case("1 blue" => vec![GameSet {blue: 1, ..Default::default()}] ; "rounds 1 blue")]
    #[test_case("10 red" => vec![GameSet {red: 10, ..Default::default()}] ; "rounds 10 red")]
    #[test_case("10 green" => vec![GameSet {green: 10, ..Default::default()}] ; "rounds 10 green")]
    #[test_case("10 blue" => vec![GameSet {blue: 10, ..Default::default()}] ; "rounds 10 blue")]
    #[test_case("1 red, 1 blue" => vec![GameSet {red: 1, green: 0, blue: 1}] ; "rounds 1 red 1 blue")]
    #[test_case("1 red, 1 green" => vec![GameSet{red: 1, green: 1, blue: 0}] ; "rounds 1 red 1 green")]
    #[test_case("10 green, 3 blue" => vec![GameSet{red: 0, green: 10, blue: 3}] ; "ronuds 10 green 3 blue")]
    #[test_case("1 red; 1 blue" => vec![GameSet {red: 1, ..Default::default()}, GameSet {blue: 1, ..Default::default()}] ; "rounds 1 red then 1 blue")]
    #[test_case("1 red, 1 blue; 1 green, 1 blue" => vec![GameSet {red: 1, green: 0, blue: 1}, GameSet {red: 0, green: 1, blue: 1}] ; "rounds 1 red 1 blue then 1 green 1 blue")]
    fn game_rounds(input: &str) -> Vec<GameSet> {
        super::parse_game_rounds(input).unwrap().1
    }

    #[test_case("" ; "when empty")]
    #[test_case(";" ; "when no blocks")]
    #[test_case("," ; "when stray comma")]
    #[should_panic]
    fn game_rounds_panics(input: &str) {
        super::parse_game_rounds(input).unwrap();
    }
}
