use crate::util::{parse_block, parse_days, parse_input_path, AccessDays, AccessPath, Day};

use proc_macro2::TokenStream;
use quote::format_ident;
use std::collections::{HashMap, HashSet};
use syn::{
    parse::{Parse, ParseStream, Result},
    parse2,
    token::{FatArrow, Semi},
    Lit, LitStr,
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

// Day override specifier: day => str;
#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct DayOverride {
    pub day: Day,
    pub replacement: String,
}

#[derive(PartialEq, Eq, Clone, Debug, Hash, Default)]
pub struct DayOverrides(pub Vec<DayOverride>);

impl DayOverrides {
    pub fn get_input_variable_name(&self, mut day: Day) -> String {
        let found = self.0.iter().rfind(|x| x.day == day);
        if let Some(day_override) = found {
            day_override.replacement.to_uppercase()
        } else {
            day.part_two = false;
            day.to_string().to_uppercase()
        }
    }

    pub fn get_input_path(&self, inp_path: &str, mut day: Day) -> String {
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

#[derive(Default)]
struct ParseData {
    input_path: Option<String>,
    days: Option<Vec<Day>>,
    overrides: Option<DayOverrides>,
    expected: Option<HashMap<Day, Lit>>,
}

impl AccessDays for ParseData {
    fn access_days(&mut self) -> &mut Option<Vec<Day>> {
        &mut self.days
    }
}

impl AccessPath for ParseData {
    fn access_path(&mut self) -> &mut Option<String> {
        &mut self.input_path
    }
}

fn parse_overrides(input: &mut ParseStream, state: &mut ParseData) -> Result<()> {
    if state.overrides.is_some() {
        Err(input.error("INPUT_OVERRIDES declared multiple times."))
    } else {
        let content;
        syn::braced!(content in input);

        let mut replacements: Vec<DayOverride> = vec![];
        let mut semicolon = Ok(Semi::default()); // Start out with a phantom semi
        while !content.is_empty() {
            // If there's more input and the last token wasn't a semi
            semicolon?;

            let item = content.parse()?;
            replacements.push(item);
            semicolon = content.parse();
        }

        state.overrides = Some(DayOverrides(replacements));
        Ok(())
    }
}

fn parse_expected(input: &mut ParseStream, state: &mut ParseData) -> Result<()> {
    if state.expected.is_some() {
        Err(input.error("OUTPUTS declared multiple times."))
    } else {
        state.expected = Some(HashMap::new());
        let content;
        syn::braced!(content in input);
        let mut semicolon = Ok(Semi::default()); // Start out with a phantom semi
        while !content.is_empty() {
            // If there's more input and the last token wasn't a semi
            semicolon?;

            let day: Day = content.parse()?;
            if state
                .expected
                .as_ref()
                .is_some_and(|map| map.contains_key(&day))
            {
                return Err(input.error(format!(
                    "output declared multiple times for {}",
                    day.to_string()
                )));
            }

            content.parse::<FatArrow>()?;

            let lit: Lit = content.parse()?;
            if let Some(map) = state.expected.as_mut() {
                map.insert(day, lit);
            }

            semicolon = content.parse();
        }

        Ok(())
    }
}

impl Parse for MakeTests {
    fn parse(mut input: ParseStream) -> Result<Self> {
        type MyFn = dyn Fn(&mut ParseStream, &mut ParseData) -> Result<()>;
        let map: HashMap<_, _> = [
            ("INPUT_PATH", Box::new(parse_input_path) as Box<MyFn>),
            ("DAYS", Box::new(parse_days) as Box<MyFn>),
            ("INPUT_OVERRIDES", Box::new(parse_overrides) as Box<MyFn>),
            ("OUTPUTS", Box::new(parse_expected) as Box<MyFn>),
        ]
        .into_iter()
        .map(|(s, f)| (s.into(), f))
        .collect();

        let parse_state = parse_block(&mut input, map, ParseData::default())?;

        if let ParseData {
            input_path: Some(input_path),
            days: Some(days),
            overrides,
            expected: Some(expected),
        } = parse_state
        {
            Ok(MakeTests {
                input_path,
                days,
                overrides: overrides.unwrap_or_else(Default::default),
                expected,
            })
        } else {
            Err(input.error("Missing required fields in make_tests block."))
        }
    }
}

pub fn make_tests(input: TokenStream) -> TokenStream {
    let ast: MakeTests = parse2(input).expect("Failed to parse make_tests AST");

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
}

#[cfg(test)]
mod test {
    use super::{DayOverrides, MakeTests};

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
