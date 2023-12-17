use crate::util::{parse_block, parse_days, parse_input_path, AccessDays, AccessPath, Day};

use proc_macro2::TokenStream;
use quote::format_ident;
use std::collections::HashMap;
use syn::{
    parse::{Parse, ParseStream, Result},
    parse2,
};

// Make benchmarks for advent of code 2023
// Example:
// make_benches! {
//   INPUT_PATH: str;
//   DAYS: [1, 1p2, ...];
// }

pub struct MakeBenches {
    input_path: String,
    days: Vec<Day>,
}

#[derive(Default)]
struct ParseData {
    path: Option<String>,
    days: Option<Vec<Day>>,
}

impl AccessDays for ParseData {
    fn access_days(&mut self) -> &mut Option<Vec<Day>> {
        &mut self.days
    }
}

impl AccessPath for ParseData {
    fn access_path(&mut self) -> &mut Option<String> {
        &mut self.path
    }
}

impl Parse for MakeBenches {
    fn parse(mut input: ParseStream) -> Result<Self> {
        type MyFn = dyn Fn(&mut ParseStream, &mut ParseData) -> Result<()>;
        let map: HashMap<_, _> = [
            ("INPUT_PATH", Box::new(parse_input_path) as Box<MyFn>),
            ("DAYS", Box::new(parse_days) as Box<MyFn>),
        ]
        .into_iter()
        .map(|(s, f)| (s.into(), f))
        .collect();

        let parse_state = parse_block(&mut input, map, ParseData::default())?;

        if let ParseData {
            path: Some(input_path),
            days: Some(days),
        } = parse_state
        {
            Ok(Self { input_path, days })
        } else {
            Err(input.error("Missing required fields in make_tests block."))
        }
    }
}

pub fn make_benches(input: TokenStream) -> TokenStream {
    let ast: MakeBenches = parse2(input).expect("Failed to parse make_benches AST");

    let setup = [quote::quote! {
        use std::fs::File;
        use std::io::{BufRead, BufReader};
        use criterion::{black_box, criterion_group, criterion_main, Criterion};
    }];
    let ast_iter = ast.days.iter().map(|day| {
        let fn_ident = format_ident!("{}", day.to_string());
        let day_txt = day.to_bench_name();
        let input_file = format!("{}/day{}.dat", ast.input_path, day.number);
        let expect_txt = format!("Could not find file {}", input_file);

        quote::quote! {
            fn #fn_ident(c: &mut Criterion) {
                let input_file =
                    File::open(#input_file).expect(#expect_txt);
                let buf_read = BufReader::new(input_file);
                let lines: Vec<String> = buf_read.lines().flatten().collect();
                let str_lines: Vec<&str> = lines.iter().map(String::as_str).collect();
                c.bench_function(#day_txt, |b| {
                    b.iter(|| advent2023::#fn_ident(black_box(&str_lines)))
                });
            }
        }
    });

    let names: Vec<_> = ast
        .days
        .iter()
        .map(|day| format_ident!("{}", day.to_string()))
        .collect();
    let ending = [quote::quote! {
        criterion_group!(
            benches,
            #(#names),*
        );

        criterion_main! { benches }
    }];

    setup.into_iter().chain(ast_iter).chain(ending).collect()
}
