use std::env;
use std::fs::File;
use std::io::{read_to_string, Write};
use std::path::PathBuf;

use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take},
    character::complete::{char, digit1, multispace0},
    combinator::{map, value, verify},
    multi::{fold_many0, many_till},
    sequence::{delimited, pair, preceded, terminated},
    IResult,
};

fn parse_day(input: &str) -> IResult<&str, &str> {
    let parse_single_digit = verify(digit1, |s: &str| s.len() == 1 && s != "0");
    let parse_double_digit = verify(digit1, |s: &str| {
        if s.is_empty() {
            return false;
        }

        let first = s.chars().next().unwrap();
        (first == '1' || first == '2') && s.len() == 2
    });

    let (rest, day) = alt((parse_single_digit, parse_double_digit))(input)?;
    if matches!(
        tag_no_case::<&str, &str, nom::error::Error<&str>>("p2")(rest),
        Result::Ok(_)
    ) {
        let (day, rest) = input.split_at(day.len() + 2);
        Ok((rest, day))
    } else {
        Ok((rest, day))
    }
}

fn parse_days(input: &str) -> IResult<&str, Vec<&str>> {
    let empty_array = value(vec![], tag("[]"));
    let single_item = delimited(
        char('['),
        map(delimited(multispace0, parse_day, multispace0), |s| vec![s]),
        char(']'),
    );

    let many_days = fold_many0(
        terminated(delimited(multispace0, parse_day, multispace0), char(',')),
        Vec::new,
        |mut accum, s| {
            accum.push(s);
            accum
        },
    );
    let multi_item = delimited(
        char('['),
        map(
            pair(many_days, delimited(multispace0, parse_day, multispace0)),
            |(mut many, last)| {
                many.push(last);
                many
            },
        ),
        char(']'),
    );

    let (rest, vec) = alt((empty_array, single_item, multi_item))(input)?;
    Ok((rest, vec))
}

fn parse_benches(input: &str) -> Vec<&str> {
    let (rest, _) = many_till(
        take::<usize, &str, nom::error::Error<&str>>(1usize),
        tag("DAYS:"),
    )(input)
    .expect("Failed to find DAYS:");
    let (rest, _) = multispace0::<&str, nom::error::Error<&str>>(rest).unwrap();
    let (rest, vec) = parse_days(rest).expect("Could not parse_days");
    preceded(multispace0, char::<&str, nom::error::Error<&str>>(';'))(rest)
        .expect("DAYS not followed by ;");
    vec
}

fn main() {
    println!("cargo:rerun-if-changed=benches/bench_main.rs");

    let bench_main = File::open("benches/bench_main.rs").expect("benches/bench_main.rs not found");
    let bench_main = read_to_string(bench_main).expect("failed to read bench_main.rs to string");
    let days = parse_benches(bench_main.as_str());

    let out_dir = env::var("OUT_DIR").unwrap();
    let out_file = PathBuf::from(out_dir).join("generate_days.dat");

    let mut out_file = File::create(out_file).expect("Could not create output file");

    // write dispatch function
    writeln!(out_file, "fn dispatch(cmd: &str) {{").unwrap();
    writeln!(
        out_file,
        "    let lines: Vec<String> = std::io::stdin().lines().map_while(Result::ok).collect();"
    )
    .unwrap();
    writeln!(
        out_file,
        "    let input: Vec<&str>   = lines.iter().map(String::as_str).collect();"
    )
    .unwrap();
    writeln!(out_file, "    match cmd {{").unwrap();

    // Write match
    days.into_iter().for_each(|day| {
        writeln!(
            out_file,
            r#"        "{}" => println!("Result is {{}}", advent2023::day{0}(&input)),"#,
            day,
        )
        .unwrap();
    });

    // finish dispatch function
    writeln!(out_file, "        _ => unreachable!(),").unwrap();
    writeln!(out_file, "    }};").unwrap();
    writeln!(out_file, "}}").unwrap();
}
