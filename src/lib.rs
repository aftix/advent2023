#![feature(iterator_try_collect)]
#![feature(iter_array_chunks)]
#![feature(isqrt)]
#![feature(ascii_char)]
// Advent of Code 2023 utility lib

use parser::day4;
use rayon::prelude::*;
use std::{collections::HashSet, ops::Range};

pub mod parser;
pub mod types;

use types::Day5;

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

pub fn day5(input: &[&str]) -> i64 {
    let mut lines: Vec<_> = input
        .par_iter()
        .enumerate()
        .map(|(idx, &line)| (idx, parser::day5::parse_line(line).map(|(_, day)| day)))
        .filter_map(|(idx, res)| {
            if let Ok(day) = res {
                Some((idx, day))
            } else {
                None
            }
        })
        .collect();
    lines.sort_by_key(|(idx, _)| *idx);
    let mut lines: Vec<_> = lines.into_iter().map(|(_, day)| day).collect();

    let mut seeds = lines.remove(0).seeds();
    let mut already_mapped = vec![];
    lines.into_iter().for_each(|day| {
        if let Day5::Maps(src, dest) = day {
            seeds.iter_mut().for_each(|seed| {
                if src.contains(seed) && !already_mapped.contains(seed) {
                    *seed += dest - src.start;
                    already_mapped.push(*seed);
                }
            });
        } else {
            already_mapped.clear();
        }
    });

    seeds.into_iter().min().unwrap()
}

pub fn day5p2(input: &[&str]) -> i64 {
    let mut lines: Vec<_> = input
        .par_iter()
        .enumerate()
        .map(|(idx, &line)| (idx, parser::day5::parse_line(line).map(|(_, day)| day)))
        .filter_map(|(idx, res)| {
            if let Ok(day) = res {
                Some((idx, day))
            } else {
                None
            }
        })
        .collect();
    lines.sort_by_key(|(idx, _)| *idx);
    let mut lines: Vec<_> = lines.into_iter().map(|(_, day)| day).collect();

    let seeds = lines.remove(0).seeds();
    let mut seeds: Vec<_> = seeds
        .into_iter()
        .array_chunks::<2usize>()
        .map(|array| (array[0]..array[0] + array[1]))
        .collect();

    let mut already_mapped: HashSet<Range<i64>> = HashSet::new();
    lines.into_iter().for_each(|day| {
        if let Day5::Maps(src, dest) = day {
            let to_add: Vec<_> = seeds
                .iter_mut()
                .filter_map(|seed| {
                    if already_mapped.contains(seed) {
                        return None;
                    }

                    match (src.contains(&seed.start), src.contains(&(seed.end - 1))) {
                        (true, true) => {
                            *seed = (dest + seed.start - src.start)..(dest + seed.end - src.start);
                            already_mapped.insert(seed.clone());
                            None
                        }
                        (true, false) => {
                            let new_range = src.end..seed.end;
                            *seed =
                                (dest + seed.start - src.start)..(dest + src.end - src.start - 1);
                            already_mapped.insert(seed.clone());
                            Some(new_range)
                        }
                        (false, true) => {
                            let new_range = dest..(dest + seed.end - src.start);
                            *seed = seed.start..src.start;
                            already_mapped.insert(new_range.clone());
                            Some(new_range)
                        }
                        (false, false) => None,
                    }
                })
                .collect();
            seeds.extend(to_add);
        } else {
            already_mapped.clear();
            let range_cmp = |left: &Range<i64>, right: &Range<i64>| match left.start.cmp(&right.end)
            {
                std::cmp::Ordering::Equal => left.end.cmp(&right.end),
                ord => ord,
            };
            seeds.sort_by(range_cmp);
            seeds.dedup_by(|check, acc| {
                if check.end < acc.start || check.start > acc.end {
                    false
                } else {
                    *acc = check.start.min(acc.start)..check.end.max(acc.end);
                    true
                }
            });
        }
    });

    seeds.into_iter().map(|r| r.start).min().unwrap()
}

mod day6;

pub fn day6(input: &[&str]) -> i64 {
    let (times, records) = parser::day6::parse_line(input);

    times
        .into_iter()
        .zip(records)
        .map(|(time, record)| day6::ways_to_win(time, record))
        .product()
}

pub fn day6p2(input: &[&str]) -> i64 {
    let (times, records) = parser::day6::parse_line(input);
    let fold_fn = |acc: i64, x: i64| {
        if acc == 0 {
            x
        } else {
            acc * i64::pow(10, x.ilog10() + 1) + x
        }
    };
    let time = times.into_iter().fold(0, fold_fn);
    let records = records.into_iter().fold(0, fold_fn);

    day6::ways_to_win(time, records)
}

#[cfg(test)]
mod test {
    use maketest::make_tests;

    make_tests! {
        INPUT_PATH: "../inputs";
        DAYS: [1, 1p2, 2, 2p2, 3, 3p2, 4, 4p2, 5, 5p2, 6, 6p2];
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
            5 => 35;
            5p2 => 46;
            6 => 288;
            6p2 => 71503;
        };
    }
}
