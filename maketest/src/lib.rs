use std::collections::{HashMap, HashSet};

use proc_macro2::TokenStream;
use quote::format_ident;
use syn::{
    parse::{Parse, ParseStream, Result},
    parse2,
    token::{Colon, Comma, FatArrow, Semi},
    Ident, Lit, LitInt, LitStr,
};

// Make tests for advent of code 2023
// Example:
// make_tests! {
//   INPUT_PATH: str;
//   DAYS: [1, 1p2, ...];
//   INPUT_OVERRIDES: {
//       1p2 => "day1p2";
//   };
//   OUTPUTS: {
//     1 => 1;
//     1p2 => "test";
//   };
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

impl Parse for Day {
    fn parse(input: ParseStream) -> Result<Self> {
        let number: LitInt = input.parse()?;
        let part_two = number.suffix() == "p2";
        let number: u8 = number.base10_parse()?;
        Ok(Self { number, part_two })
    }
}

// Day override specifier: day => str;
#[derive(PartialEq, Eq, Clone, Debug, Hash)]
struct DayOverride {
    day: Day,
    replacement: String,
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
struct DayOverrides(Vec<DayOverride>);

impl DayOverrides {
    fn get_input_variable_name(&self, mut day: Day) -> String {
        let found = self.0.iter().rfind(|x| x.day == day);
        if let Some(day_override) = found {
            day_override.replacement.to_uppercase()
        } else {
            day.part_two = false;
            day.to_string().to_uppercase()
        }
    }

    fn get_input_path(&self, inp_path: &str, mut day: Day) -> String {
        let found = self.0.iter().rfind(|x| x.day == day);
        if let Some(day_override) = found {
            format!("{}/{}_ex.dat", inp_path, day_override.replacement)
        } else {
            day.part_two = false;
            format!("{}/{}_ex.dat", inp_path, day.to_string())
        }
    }
}

impl Parse for DayOverride {
    fn parse(input: ParseStream) -> Result<Self> {
        let day: Day = input.parse()?;
        input.parse::<FatArrow>()?;
        let replacement: LitStr = input.parse()?;

        Ok(Self {
            day,
            replacement: replacement.value(),
        })
    }
}

struct MakeTests {
    input_path: String,
    days: Vec<Day>,
    overrides: DayOverrides,
    expected: HashMap<Day, Lit>,
}

impl Parse for MakeTests {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut make_tests = Self {
            input_path: String::new(),
            days: vec![],
            overrides: DayOverrides(vec![]),
            expected: HashMap::new(),
        };

        let mut has_input_path = false;
        let mut has_days = false;
        let mut has_overrides = false;
        let mut has_expected = false;

        while !input.is_empty() {
            let ident: Ident = input.parse()?;
            input.parse::<Colon>()?;

            match ident.to_string().as_str() {
                "INPUT_PATH" => {
                    if has_input_path {
                        return Err(input.error("INPUT_PATH declared multiple times."));
                    }
                    has_input_path = true;

                    let string: LitStr = input.parse()?;
                    make_tests.input_path = string.value();
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

                    make_tests.days = days;
                    input.parse::<Semi>()?;
                }
                "INPUT_OVERRIDES" => {
                    if has_overrides {
                        return Err(input.error("INPUT_OVERRIDES declared multiple times."));
                    }
                    has_overrides = false;

                    let content;
                    syn::braced!(content in input);

                    let mut replacements: Vec<DayOverride> = vec![];
                    let mut semicolon = Ok(Semi::default()); // Start out with a phantom semi
                    while !content.is_empty() {
                        if let Err(e) = semicolon {
                            // If there's more input and the last token wasn't a semi
                            return Err(e);
                        }
                        let item = content.parse()?;
                        replacements.push(item);
                        semicolon = content.parse();
                    }

                    make_tests.overrides = DayOverrides(replacements);
                    input.parse::<Semi>()?;
                }
                "OUTPUTS" => {
                    if has_expected {
                        return Err(input.error("OUTPUTS declared multiple times."));
                    }
                    has_expected = true;

                    let content;
                    syn::braced!(content in input);
                    let mut semicolon = Ok(Semi::default()); // Start out with a phantom semi
                    while !content.is_empty() {
                        if let Err(e) = semicolon {
                            // If there's more input and the last token wasn't a semi
                            return Err(e);
                        }

                        let day: Day = content.parse()?;
                        if make_tests.expected.contains_key(&day) {
                            return Err(input.error(&format!(
                                "output declared multiple times for {}",
                                day.to_string()
                            )));
                        }

                        content.parse::<FatArrow>()?;

                        let lit: Lit = content.parse()?;
                        make_tests.expected.insert(day, lit);

                        semicolon = content.parse();
                    }

                    input.parse::<Semi>()?;
                }
                _ => return Err(input.error("Unknown identifier encountered.")),
            }
        }

        Ok(make_tests)
    }
}

#[proc_macro]
pub fn make_tests(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: MakeTests = parse2(input.into()).expect("Failed to parse make_tests AST");

    let mut created_statics: HashSet<String> = HashSet::new();
    ast.days
        .into_iter()
        .map(|day| {
            let day_str = quote::format_ident!("{}", day.to_string());
            let day_static = ast.overrides.get_input_variable_name(day);

            let lit = ast
                .expected
                .get(&day)
                .expect("Did not find declared day in OUTPUTS");

            if created_statics.contains(&day_static) {
                let day_static = format_ident!("{}", day_static);
                quote::quote! {
                    #[test]
                    fn #day_str() {
                        let input: Vec<&str> = #day_static.lines().collect();
                        let output = super::#day_str(&input);
                        assert_eq!(#lit, output);
                    }
                }
            } else {
                created_statics.insert(day_static.clone());
                let day_static = format_ident!("{}", day_static);
                let day_path = ast.overrides.get_input_path(&ast.input_path, day);
                quote::quote! {
                    const #day_static: &str = include_str!(#day_path);

                    #[test]
                    fn #day_str() {
                        let input: Vec<&str> = #day_static.lines().collect();
                        let output = super::#day_str(&input);
                        assert_eq!(#lit, output);
                    }
                }
            }
        })
        .collect::<TokenStream>()
        .into()
}

#[cfg(test)]
mod test {
    use crate::{DayOverrides, MakeTests};

    use super::{Day, DayOverride};
    use proc_macro2::TokenStream;
    use std::str::FromStr;
    use syn::{parse2, Lit};
    use test_case::test_case;

    #[test_case("1" => Day {number: 1, part_two: false})]
    #[test_case("1p2" => Day {number: 1, part_two: true})]
    #[test_case("10" => Day {number: 10, part_two: false})]
    #[test_case("10p2" => Day {number: 10, part_two: true})]
    fn day(input: &str) -> Day {
        let ts = TokenStream::from_str(input).unwrap();
        parse2(ts).unwrap()
    }

    #[test_case("1 => \"day1p2\"" => DayOverride {day: Day {number: 1, part_two: false}, replacement: String::from("day1p2")})]
    #[test_case("1p2 => \"day1p2\"" => DayOverride {day: Day {number: 1, part_two: true}, replacement: String::from("day1p2")})]
    fn day_override(input: &str) -> DayOverride {
        let ts = TokenStream::from_str(input).unwrap();
        parse2(ts).unwrap()
    }

    #[test]
    fn full() {
        let input = r#"INPUT_PATH: "../inputs";
        DAYS: [1, 1p2, 2, 2p2];
        INPUT_OVERRIDES: {
            1p2 => "day1p2";
        };
        OUTPUTS: {
            1 => 1;
            1p2 => "hi";
            2 => true;
            2p2 => false;
        };"#;
        let ts = TokenStream::from_str(input).unwrap();
        let make_tests: MakeTests = parse2(ts).unwrap();

        assert_eq!("../inputs", make_tests.input_path);
        assert_eq!(
            vec![
                Day {
                    number: 1,
                    part_two: false,
                },
                Day {
                    number: 1,
                    part_two: true,
                },
                Day {
                    number: 2,
                    part_two: false,
                },
                Day {
                    number: 2,
                    part_two: true,
                },
            ],
            make_tests.days
        );
        assert_eq!(
            DayOverrides(vec![DayOverride {
                day: Day {
                    number: 1,
                    part_two: true
                },
                replacement: String::from("day1p2")
            }]),
            make_tests.overrides
        );

        let day1 = quote::quote!(1);
        let day1p2 = quote::quote!("hi");
        let day2 = quote::quote!(true);
        let day2p2 = quote::quote!(false);

        let day1: Lit = parse2(day1).unwrap();
        let day1p2: Lit = parse2(day1p2).unwrap();
        let day2: Lit = parse2(day2).unwrap();
        let day2p2: Lit = parse2(day2p2).unwrap();

        let mut outputs = vec![
            (
                Day {
                    number: 1,
                    part_two: false,
                },
                day1,
            ),
            (
                Day {
                    number: 1,
                    part_two: true,
                },
                day1p2,
            ),
            (
                Day {
                    number: 2,
                    part_two: false,
                },
                day2,
            ),
            (
                Day {
                    number: 2,
                    part_two: true,
                },
                day2p2,
            ),
        ];
        outputs.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());

        let mut my_outputs: Vec<(Day, Lit)> = make_tests.expected.into_iter().collect();
        my_outputs.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());

        assert_eq!(outputs, my_outputs);
    }
}
