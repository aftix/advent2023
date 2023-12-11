use quote::format_ident;
use syn::{
    parse::{Parse, ParseStream, Result},
    parse2,
    token::{Colon, Comma, Semi},
    Ident, LitInt, LitStr,
};

// Make benchmarks for advent of code 2023
// Example:
// make_benches! {
//   INPUT_PATH: str;
//   DAYS: [1, 1p2, ...];
// }

// Day specifier: [12]?[0-9](p2)?
#[derive(PartialEq, PartialOrd, Eq, Copy, Clone, Debug, Hash)]
struct Day {
    number: u8,
    part_two: bool,
}

impl ToString for Day {
    fn to_string(&self) -> String {
        if self.part_two {
            format!("day{}p2", self.number)
        } else {
            format!("day{}", self.number)
        }
    }
}

impl Day {
    fn to_bench_name(&self) -> String {
        if self.part_two {
            format!("day {} p2", self.number)
        } else {
            format!("day {}", self.number)
        }
    }
}

impl Parse for Day {
    fn parse(input: ParseStream) -> Result<Self> {
        let number: LitInt = input.parse()?;
        let part_two = number.suffix() == "p2";
        let number: u8 = number.base10_parse()?;
        Ok(Self { number, part_two })
    }
}

struct MakeBenches {
    input_path: String,
    days: Vec<Day>,
}

impl Parse for MakeBenches {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut make_benches = Self {
            input_path: String::new(),
            days: vec![],
        };

        let mut has_path = false;
        let mut has_days = false;

        while !input.is_empty() {
            let ident: Ident = input.parse()?;
            input.parse::<Colon>()?;

            match ident.to_string().as_str() {
                "INPUT_PATH" => {
                    if has_path {
                        return Err(input.error("INPUT_PATH declared multiple times."));
                    }
                    has_path = true;

                    let string: LitStr = input.parse()?;
                    make_benches.input_path = string.value();
                    input.parse::<Semi>()?;
                }
                "DAYS" => {
                    if has_days {
                        return Err(input.error("DAYS declared multiple times."));
                    }
                    has_days = true;

                    let content;
                    syn::bracketed!(content in input);

                    let mut days: Vec<Day> = vec![];
                    let mut comma = Ok(Comma::default()); // Start out with a phantom comma
                    while !content.is_empty() {
                        if let Err(e) = comma {
                            // If there's more input and the last token wasn't a comma
                            return Err(e);
                        }
                        let day = content.parse()?;
                        days.push(day);
                        comma = content.parse();
                    }

                    make_benches.days = days;
                    input.parse::<Semi>()?;
                }
                _ => {
                    return Err(input.error("Uknown identifier encountered."));
                }
            }
        }

        Ok(make_benches)
    }
}

#[proc_macro]
pub fn make_benches(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: MakeBenches = parse2(input.into()).expect("Failed to parse make_benches AST");

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
                c.bench_function(#day_txt, |b| {
                    let input_file =
                        File::open(#input_file).expect(#expect_txt);
                    let buf_read = BufReader::new(input_file);
                    let lines: Vec<String> = buf_read.lines().flatten().collect();
                    let str_lines: Vec<&str> = lines.iter().map(String::as_str).collect();
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

    setup
        .into_iter()
        .chain(ast_iter.into_iter())
        .chain(ending.into_iter())
        .map(Into::<proc_macro::TokenStream>::into)
        .collect()
}
