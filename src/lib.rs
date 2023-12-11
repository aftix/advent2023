#![feature(iterator_try_collect)]
// Advent of Code 2023 utility lib

use parser::day4;
use rayon::prelude::*;

pub mod parser;
pub mod types;

// Solutions

mod day1 {
    use crate::parser::parse_literal_digit;
    use nom::{
        character::complete::alpha0,
        multi::{many1, many_till},
        IResult,
    };

    fn parse_glob_then_digit(input: &str) -> IResult<&str, i64> {
        let (rest, (_, digit)) = many_till(alpha0, parse_literal_digit)(input)?;
        Ok((rest, digit))
    }

    pub(super) fn parse_line(input: &str) -> IResult<&str, i64> {
        let (_, first_digit) = parse_glob_then_digit(input)?;
        let (_, second_digit) = many1(parse_glob_then_digit)(input)?;
        let second_digit = second_digit.last().unwrap();
        Ok(("", first_digit * 10 + second_digit))
    }
}

pub fn day1(input: &[&str]) -> i64 {
    input
        .par_iter()
        .map(|line| day1::parse_line(line).ok().map(|(_, num)| num))
        .flatten()
        .sum()
}

pub fn day1p2(input: &[&str]) -> i64 {
    input
        .par_iter()
        .map(|line| parser::day1::parse_line(line).ok().map(|(_, num)| num))
        .flatten()
        .sum()
}

pub fn day2(input: &[&str]) -> i64 {
    const NUM_RED: i64 = 12;
    const NUM_GREEN: i64 = 13;
    const NUM_BLUE: i64 = 14;

    input
        .into_par_iter()
        .map(|line| parser::day2::parse_line(line).ok().map(|(_, game)| game))
        .flatten()
        .filter(|game| {
            game.sets
                .par_iter()
                .find_any(|set| set.red > NUM_RED || set.green > NUM_GREEN || set.blue > NUM_BLUE)
                .is_none()
        })
        .map(|game| game.id)
        .sum()
}

mod day2p2 {
    use crate::types::{Game, GameSet};
    use rayon::prelude::*;

    pub(super) fn get_power(game: Game) -> i64 {
        let maximums = game.sets.into_par_iter().reduce(
            || GameSet {
                red: 0,
                green: 0,
                blue: 0,
            },
            |mut acc, set| {
                acc.red = acc.red.max(set.red);
                acc.green = acc.green.max(set.green);
                acc.blue = acc.blue.max(set.blue);
                acc
            },
        );
        maximums.red * maximums.green * maximums.blue
    }
}

pub fn day2p2(input: &[&str]) -> i64 {
    input
        .into_par_iter()
        .map(|line| parser::day2::parse_line(line).ok().map(|(_, game)| game))
        .flatten()
        .map(day2p2::get_power)
        .sum()
}

mod day3 {
    use crate::types::Schematic;
    use std::collections::{HashMap, HashSet};

    fn generate_adjacency(x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
        [
            (x + 1, y),
            (x - 1, y),
            (x, y + 1),
            (x, y - 1),
            (x + 1, y + 1),
            (x + 1, y - 1),
            (x - 1, y + 1),
            (x - 1, y - 1),
        ]
        .into_iter()
    }

    pub(super) fn linear_to_rect(idx: usize, width: usize) -> (usize, usize) {
        (idx % width, idx / width)
    }

    pub(super) fn get_coordinate_map(
        items: &[Schematic],
        width: usize,
    ) -> HashMap<(usize, usize), Schematic> {
        items
            .iter()
            .flat_map(|schem| {
                let span = schem.span();
                (span.0..span.1).map(|idx| (linear_to_rect(idx, width), *schem))
            })
            .collect()
    }

    pub(super) fn get_adjacent_numbers_fn(
        width: usize,
        map: HashMap<(usize, usize), Schematic>,
    ) -> impl FnMut(usize) -> i64 {
        let mut already_added = HashSet::new();
        move |idx: usize| {
            let (x, y) = linear_to_rect(idx, width);
            generate_adjacency(x, y)
                .map(|coords| {
                    if let Some(&schem) = map.get(&coords) {
                        match schem {
                            Schematic::Number(num, _) => {
                                if already_added.insert(schem) {
                                    num
                                } else {
                                    0
                                }
                            }
                            _ => 0,
                        }
                    } else {
                        0
                    }
                })
                .sum()
        }
    }

    pub(super) fn get_gear_ratio(
        idx: usize,
        width: usize,
        map: &HashMap<(usize, usize), Schematic>,
    ) -> i64 {
        let (x, y) = linear_to_rect(idx, width);
        let mut already_found = vec![];

        let adjacent_numbers: Vec<_> = generate_adjacency(x, y)
            .filter_map(|coords| {
                if let Some(&schem) = map.get(&coords) {
                    match schem {
                        Schematic::Number(num, span) => {
                            if !already_found.contains(&span) {
                                already_found.push(span);
                                Some(num)
                            } else {
                                None
                            }
                        }
                        _ => None,
                    }
                } else {
                    None
                }
            })
            .collect();

        if adjacent_numbers.len() != 2 {
            0
        } else {
            adjacent_numbers[0] * adjacent_numbers[1]
        }
    }
}

pub fn day3(input: &[&str]) -> i64 {
    let (_, ((width, _), items)) = parser::day3::parse_input(input).expect("Failed to parse input");
    let coordinate_map = day3::get_coordinate_map(&items, width);
    let mut get_adjacent_numbers = day3::get_adjacent_numbers_fn(width, coordinate_map);
    items
        .iter()
        .filter(|schem| schem.is_symbol())
        .map(|symb| get_adjacent_numbers(symb.span().0))
        .sum()
}

pub fn day3p2(input: &[&str]) -> i64 {
    let (_, ((width, _), items)) = parser::day3::parse_input(input).expect("Failed to parse input");
    let coordinate_map = day3::get_coordinate_map(&items, width);

    items
        .iter()
        .filter(|schem| matches!(schem, crate::types::Schematic::Symbol('*', _)))
        .map(|symb| day3::get_gear_ratio(symb.span().0, width, &coordinate_map))
        .sum()
}

fn get_winners((_id, winners, cards): &(u32, Vec<i64>, Vec<i64>)) -> usize {
    cards.iter().filter(|num| winners.contains(num)).count()
}

pub fn day4(input: &[&str]) -> i64 {
    input
        .par_iter()
        .map(|&line| day4::parse_line(line).map(|(_, tuple)| tuple))
        .flatten()
        .map(|x| get_winners(&x))
        .map(|num_winners| {
            if num_winners == 0 {
                0
            } else {
                1 << (num_winners - 1)
            }
        })
        .sum()
}

mod day4p2;

pub fn day4p2(input: &[&str]) -> i64 {
    let cards: Vec<_> = input
        .par_iter()
        .map(|&line| day4::parse_line(line).map(|(_, tuple)| tuple))
        .flatten()
        .collect();

    day4p2::Day4p2::new(&cards).map(|(num, _)| num as i64).sum()
}

#[cfg(test)]
mod test {
    use maketest::make_tests;

    make_tests! {
        INPUT_PATH: "../inputs";
        DAYS: [1, 1p2, 2, 2p2, 3, 3p2, 4];
        INPUT_OVERRIDES: {
            1p2 => "day1p2";
        };
        OUTPUTS: {
            1 => 142;
            1p2 => 281;
            2 => 8;
            2p2 => 2286;
            3 => 4361;
            3p2 => 467835;
            4 => 13;
            4p2 => 30;
        };
    }
}
