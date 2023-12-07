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

fn day1_benchmark(c: &mut Criterion) {
    c.bench_function("day1 100", |b| {
        let input_file =
            File::open("bench_inputs/day1.dat").expect("Could not find file bench_inputs/day1.dat");
        let buf_read = BufReader::new(input_file);
        let lines: Vec<String> = buf_read.lines().flatten().collect();
        let str_lines: Vec<&str> = lines.iter().map(String::as_str).collect();
        b.iter(|| day1(black_box(&str_lines)))
    });
}

criterion_group!(benches, day1_benchmark);

criterion_main! {
    benches
}