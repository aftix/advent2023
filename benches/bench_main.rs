use std::fs::File;
use std::io::{BufRead, BufReader};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn day1_benchmark(c: &mut Criterion) {
    c.bench_function("day 1", |b| {
        let input_file =
            File::open("bench_inputs/day1.dat").expect("Could not find file bench_inputs/day1.dat");
        let buf_read = BufReader::new(input_file);
        let lines: Vec<String> = buf_read.lines().flatten().collect();
        let str_lines: Vec<&str> = lines.iter().map(String::as_str).collect();
        b.iter(|| advent2023::day1(black_box(&str_lines)))
    });
}

fn day1p2_benchmark(c: &mut Criterion) {
    c.bench_function("day 1 p2", |b| {
        let input_file =
            File::open("bench_inputs/day1.dat").expect("Could not find file bench_inputs/day1.dat");
        let buf_read = BufReader::new(input_file);
        let lines: Vec<String> = buf_read.lines().flatten().collect();
        let str_lines: Vec<&str> = lines.iter().map(String::as_str).collect();
        b.iter(|| advent2023::day1p2(black_box(&str_lines)))
    });
}

fn day2_benchmark(c: &mut Criterion) {
    c.bench_function("day 2", |b| {
        let input_file =
            File::open("bench_inputs/day2.dat").expect("Could not find file bench_inputs/day2.dat");
        let buf_read = BufReader::new(input_file);
        let lines: Vec<String> = buf_read.lines().flatten().collect();
        let str_lines: Vec<&str> = lines.iter().map(String::as_str).collect();
        b.iter(|| advent2023::day2(black_box(&str_lines)))
    });
}

fn day2p2_benchmark(c: &mut Criterion) {
    c.bench_function("day 2 p2", |b| {
        let input_file =
            File::open("bench_inputs/day2.dat").expect("Could not find file bench_inputs/day2.dat");
        let buf_read = BufReader::new(input_file);
        let lines: Vec<String> = buf_read.lines().flatten().collect();
        let str_lines: Vec<&str> = lines.iter().map(String::as_str).collect();
        b.iter(|| advent2023::day2p2(black_box(&str_lines)))
    });
}

fn day3_benchmark(c: &mut Criterion) {
    c.bench_function("day 3", |b| {
        let input_file =
            File::open("bench_inputs/day3.dat").expect("Could not find file bench_inputs/day3.dat");
        let buf_read = BufReader::new(input_file);
        let lines: Vec<String> = buf_read.lines().flatten().collect();
        let str_lines: Vec<&str> = lines.iter().map(String::as_str).collect();
        b.iter(|| advent2023::day3(black_box(&str_lines)))
    });
}

fn day3p2_benchmark(c: &mut Criterion) {
    c.bench_function("day 3 p2", |b| {
        let input_file =
            File::open("bench_inputs/day3.dat").expect("Could not find file bench_inputs/day3.dat");
        let buf_read = BufReader::new(input_file);
        let lines: Vec<String> = buf_read.lines().flatten().collect();
        let str_lines: Vec<&str> = lines.iter().map(String::as_str).collect();
        b.iter(|| advent2023::day3p2(black_box(&str_lines)))
    });
}

fn day4_benchmark(c: &mut Criterion) {
    c.bench_function("day 4", |b| {
        let input_file =
            File::open("bench_inputs/day3.dat").expect("Could not find file bench_inputs/day3.dat");
        let buf_read = BufReader::new(input_file);
        let lines: Vec<String> = buf_read.lines().flatten().collect();
        let str_lines: Vec<&str> = lines.iter().map(String::as_str).collect();
        b.iter(|| advent2023::day4(black_box(&str_lines)))
    });
}

criterion_group!(
    benches,
    day1_benchmark,
    day1p2_benchmark,
    day2_benchmark,
    day2p2_benchmark,
    day3_benchmark,
    day3p2_benchmark,
    day4_benchmark,
);

criterion_main! {
    benches
}
