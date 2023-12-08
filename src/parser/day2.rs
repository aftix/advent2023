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
    use ntest::test_case;

    #[test_case("Game 0:", 0, "")]
    #[test_case("Game 10:", 10, "")]
    #[test_case("Game 0:a", 0, "a")]
    #[test_case("Game 10:a", 10, "a")]
    #[test_case("Game 0", 0, "")]
    #[should_panic]
    #[test_case("Game ", 0, "")]
    #[should_panic]
    #[test_case("", 0, "")]
    #[should_panic]
    fn game_id(input: &str, id: i64, remains: &str) {
        let (rest, num) = super::parse_game_id(input).unwrap();
        assert_eq!(id, num);
        assert_eq!(remains, rest);
    }

    #[test_case("1 red", 1, 0, 0, "")]
    #[test_case("1 green", 0, 1, 0, "")]
    #[test_case("1 blue", 0, 0, 1, "")]
    #[test_case("10 red", 10, 0, 0, "")]
    #[test_case("10 green", 0, 10, 0, "")]
    #[test_case("10 blue", 0, 0, 10, "")]
    #[test_case("1 red, ", 1, 0, 0, ", ")]
    #[test_case("", 0, 0, 0, "")]
    #[should_panic]
    #[test_case("a.eueou", 0, 0, 0, "")]
    #[should_panic]
    fn game_cubes(input: &str, red: i64, green: i64, blue: i64, remains: &str) {
        let (rest, set) = super::parse_game_cubes(input).unwrap();
        assert_eq!(red, set.red);
        assert_eq!(green, set.green);
        assert_eq!(blue, set.blue);
        assert_eq!(remains, rest);
    }

    #[test_case("1 red", 1, 0, 0, "")]
    #[test_case("1 green", 0, 1, 0, "")]
    #[test_case("1 blue", 0, 0, 1, "")]
    #[test_case("10 red", 10, 0, 0, "")]
    #[test_case("10 green", 0, 10, 0, "")]
    #[test_case("10 blue", 0, 0, 10, "")]
    #[test_case("1 red, 1 blue", 1, 0, 1, "")]
    #[test_case("1 red, 1 green", 1, 1, 0, "")]
    #[test_case("10 green, 3 blue", 0, 10, 3, "")]
    #[test_case("1 red;", 1, 0, 0, ";")]
    #[test_case("", 0, 0, 0, "")]
    #[should_panic]
    #[test_case("a.eueou", 0, 0, 0, "")]
    #[should_panic]
    fn game_set(input: &str, red: i64, green: i64, blue: i64, remains: &str) {
        let (rest, set) = super::parse_game_set(input).unwrap();
        assert_eq!(red, set.red);
        assert_eq!(green, set.green);
        assert_eq!(blue, set.blue);
        assert_eq!(remains, rest);
    }

    #[test]
    fn game_rounds() {
        use crate::types::GameSet;

        let inputs = vec![
            "1 red",
            "1 green",
            "1 blue",
            "10 red",
            "10 green",
            "10 blue",
            "1 red, 1 blue",
            "1 red, 1 green",
            "10 green, 3 blue",
            "1 red; 1 blue",
            "1 red, 1 blue; 1 green, 1 blue",
        ];

        let outputs = vec![
            vec![GameSet {
                red: 1,
                ..Default::default()
            }],
            vec![GameSet {
                green: 1,
                ..Default::default()
            }],
            vec![GameSet {
                blue: 1,
                ..Default::default()
            }],
            vec![GameSet {
                red: 10,
                ..Default::default()
            }],
            vec![GameSet {
                green: 10,
                ..Default::default()
            }],
            vec![GameSet {
                blue: 10,
                ..Default::default()
            }],
            vec![GameSet {
                red: 1,
                green: 0,
                blue: 1,
            }],
            vec![GameSet {
                red: 1,
                green: 1,
                blue: 0,
            }],
            vec![GameSet {
                red: 0,
                green: 10,
                blue: 3,
            }],
            vec![
                GameSet {
                    red: 1,
                    ..Default::default()
                },
                GameSet {
                    blue: 1,
                    ..Default::default()
                },
            ],
            vec![
                GameSet {
                    red: 1,
                    green: 0,
                    blue: 1,
                },
                GameSet {
                    red: 0,
                    green: 1,
                    blue: 1,
                },
            ],
        ];

        for (inp, out) in inputs.iter().zip(outputs.iter()) {
            let res = super::parse_game_rounds(inp);
            assert!(res.is_ok());
            let (_, vec) = res.unwrap();
            assert_eq!(out, &vec);
        }

        let errs = vec!["", ";", ",", "aeouaeu"];

        for err in errs {
            assert!(super::parse_game_rounds(err).is_err());
        }
    }
}
