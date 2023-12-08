use std::fs::File;
use std::io::{BufRead, BufReader};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rayon::prelude::*;

mod day1 {
    use advent2023::parser::parse_literal_digit;
    use nom::{
        character::complete::alpha0,
        multi::{many1, many_till},
        IResult,
    };

    pub(super) fn parse_line(input: &str) -> IResult<&str, i64> {
        let (_, first_digit) = parse_glob_then_digit(input)?;
        let (_, second_digit) = many1(parse_glob_then_digit)(input)?;
        let second_digit = second_digit.last().unwrap();
        Ok(("", first_digit * 10 + second_digit))
    }

    fn parse_glob_then_digit(input: &str) -> IResult<&str, i64> {
        let (rest, (_, digit)) = many_till(alpha0, parse_literal_digit)(input)?;
        Ok((rest, digit))
    }
}

fn day1(r: &[&str]) -> i64 {
    r.par_iter()
        .map(|line| day1::parse_line(&line).ok().map(|(_, num)| num))
        .flatten()
        .sum()
}

fn day1p2(r: &[&str]) -> i64 {
    use advent2023::parser::day1::parse_line;
    r.par_iter()
        .map(|line| parse_line(&line).ok().map(|(_, num)| num))
        .flatten()
        .sum()
}

fn day1_benchmark(c: &mut Criterion) {
    c.bench_function("day1", |b| {
        let input_file =
            File::open("bench_inputs/day1.dat").expect("Could not find file bench_inputs/day1.dat");
        let buf_read = BufReader::new(input_file);
        let lines: Vec<String> = buf_read.lines().flatten().collect();
        let str_lines: Vec<&str> = lines.iter().map(String::as_str).collect();
        b.iter(|| day1(black_box(&str_lines)))
    });
}

fn day1p2_benchmark(c: &mut Criterion) {
    c.bench_function("day1p2", |b| {
        let input_file =
            File::open("bench_inputs/day1.dat").expect("Could not find file bench_inputs/day1.dat");
        let buf_read = BufReader::new(input_file);
        let lines: Vec<String> = buf_read.lines().flatten().collect();
        let str_lines: Vec<&str> = lines.iter().map(String::as_str).collect();
        b.iter(|| day1p2(black_box(&str_lines)))
    });
}

fn get_power(game: advent2023::types::Game) -> i64 {
    use advent2023::types::GameSet;
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

fn day2(r: &[&str]) -> i64 {
    use advent2023::parser::day2::parse_line;
    r.par_iter()
        .map(|&line| parse_line(line).ok().map(|(_, game)| game))
        .flatten()
        .filter(|game| {
            game.sets
                .par_iter()
                .find_any(|set| set.red > 12 || set.green > 13 || set.blue > 14)
                .is_none()
        })
        .map(|game| game.id)
        .sum()
}

fn day2p2(r: &[&str]) -> i64 {
    use advent2023::parser::day2::parse_line;
    r.par_iter()
        .map(|&line| parse_line(line).ok().map(|(_, game)| game))
        .flatten()
        .map(get_power)
        .sum()
}

fn day2_benchmark(c: &mut Criterion) {
    c.bench_function("day 2", |b| {
        let input_file =
            File::open("bench_inputs/day2.dat").expect("Could not find file bench_inputs/day2.dat");
        let buf_read = BufReader::new(input_file);
        let lines: Vec<String> = buf_read.lines().flatten().collect();
        let str_lines: Vec<&str> = lines.iter().map(String::as_str).collect();
        b.iter(|| day2(black_box(&str_lines)))
    });
}

fn day2p2_benchmark(c: &mut Criterion) {
    c.bench_function("day2 p2", |b| {
        let input_file =
            File::open("bench_inputs/day2.dat").expect("Could not find file bench_inputs/day2.dat");
        let buf_read = BufReader::new(input_file);
        let lines: Vec<String> = buf_read.lines().flatten().collect();
        let str_lines: Vec<&str> = lines.iter().map(String::as_str).collect();
        b.iter(|| day2p2(black_box(&str_lines)))
    });
}

criterion_group!(
    benches,
    day1_benchmark,
    day1p2_benchmark,
    day2_benchmark,
    day2p2_benchmark
);

criterion_main! {
    benches
}
