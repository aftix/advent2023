use phf::Map;
use syn::{
    parse::{Parse, ParseStream, Result},
    token::{Colon, Comma, Semi},
    Ident, LitInt, LitStr,
};

pub type IdentMap<T> = Map<&'static str, fn(&mut ParseStream, &mut T) -> Result<()>>;

// Day specifier: [12]?[0-9](p2)?
#[derive(PartialEq, PartialOrd, Eq, Copy, Clone, Debug, Hash)]
pub struct Day {
    pub number: u8,
    pub part_two: bool,
}

impl std::fmt::Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.part_two {
            write!(f, "day{}p2", self.number)
        } else {
            write!(f, "day{}", self.number)
        }
    }
}

impl Day {
    pub fn to_bench_name(self) -> String {
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

pub trait AccessDays {
    fn access_days(&mut self) -> &mut Option<Vec<Day>>;
}

pub trait AccessPath {
    fn access_path(&mut self) -> &mut Option<String>;
}

pub fn parse_input_path<T: AccessPath>(input: &mut ParseStream, state: &mut T) -> Result<()> {
    let path = state.access_path();
    if path.is_some() {
        Err(input.error("INPUT_PATH declared multiple times."))
    } else {
        let litstr: LitStr = input.parse()?;
        *path = Some(litstr.value());
        Ok(())
    }
}

pub fn parse_days<T: AccessDays>(input: &mut ParseStream, state: &mut T) -> Result<()> {
    let state_days = state.access_days();
    if state_days.is_some() {
        Err(input.error("DAYS declared multiple times."))
    } else {
        let content;
        syn::bracketed!(content in input);

        let mut days: Vec<Day> = vec![];
        let mut comma = Ok(Comma::default()); // Start out with a phantom comma
        while !content.is_empty() {
            // If there's more input and the last token wasn't a comma
            comma?;

            let day = content.parse()?;
            days.push(day);
            comma = content.parse();
        }

        *state_days = Some(days);
        Ok(())
    }
}

// Parse a block of IDENT: <anything>; using a map of IDENT -> parsers
// parses the following semicolon as well, parser shouldn't
pub fn parse_block<T>(input: &mut ParseStream, map: &IdentMap<T>, mut init: T) -> Result<T> {
    while !input.is_empty() {
        let ident: Ident = input.parse()?;
        input.parse::<Colon>()?;

        if let Some(func) = map.get(&ident.to_string()) {
            func(input, &mut init)?;
        } else {
            return Err(input.error(format!("Unknown identifier encountered: {}", ident)));
        }

        input.parse::<Semi>()?;
    }

    Ok(init)
}
