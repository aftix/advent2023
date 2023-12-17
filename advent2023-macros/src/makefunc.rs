use crate::util::Day;

use proc_macro2::TokenStream;
use quote::format_ident;
use syn::{
    parse::{Parse, ParseStream},
    parse2,
    token::{Colon, Semi},
    Block, Ident, Result, Type,
};

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
struct Parser {
    parser: Day,
}

impl Parse for Parser {
    fn parse(input: ParseStream) -> Result<Self> {
        let id: Ident = input.parse()?;
        if id.to_string().as_str() != "PARSER" {
            return Err(input.error("Wrong identifier"));
        }

        input.parse::<Colon>()?;
        let parser: Day = input.parse()?;
        input.parse::<Semi>()?;
        Ok(Self { parser })
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
struct Output {
    output: Type,
}

impl Parse for Output {
    fn parse(input: ParseStream) -> Result<Self> {
        let id: Ident = input.parse()?;
        if id.to_string().as_str() != "OUTPUT" {
            return Err(input.error("Wrong identifier"));
        }

        input.parse::<Colon>()?;
        let output: Type = input.parse()?;
        input.parse::<Semi>()?;
        Ok(Self { output })
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
struct NoOkay;

impl Parse for NoOkay {
    fn parse(input: ParseStream) -> Result<Self> {
        let id: Ident = input.parse()?;
        if id.to_string().as_str() != "NO_OKAY" {
            return Err(input.error("Wrong identifier"));
        }

        input.parse::<Semi>()?;
        Ok(Self)
    }
}

// Make a day solving function for advent of code 2023
// Example:
// make_func! {
//     <DaySpecifier>;
//     PARSER: <DaySpecifier>; // optional
//     OUTPUT: i64; // optional
//     NO_OKAY; // optional
//     { function body }
// }
#[derive(PartialEq, Eq, Clone, Debug, Hash)]
struct MakeFunc {
    day: Day,
    parser: Day,
    output: Type,
    no_ok: bool,
    block: Block,
}

impl Parse for MakeFunc {
    fn parse(input: ParseStream) -> Result<Self> {
        let day: Day = input.parse()?;
        input.parse::<Semi>()?;

        let mut parser = day;
        parser.part_two = false;
        let mut output = Type::Verbatim(quote::quote!(i64));
        let mut no_ok = false;

        let mut has_parser = false;
        let mut has_output = false;

        while input.peek(Ident) {
            if let Ok(p) = input.fork().parse::<Parser>() {
                input.parse::<Parser>().unwrap();
                if has_parser {
                    return Err(input.error("PARSER specfied multiple times"));
                }
                has_parser = true;
                parser = p.parser;
            } else if let Ok(o) = input.fork().parse::<Output>() {
                input.parse::<Output>().unwrap();
                if has_output {
                    return Err(input.error("OUTPUT specified multiple times"));
                }
                has_output = true;
                output = o.output;
            } else if matches!(input.fork().parse::<NoOkay>(), Ok(_)) {
                input.parse::<NoOkay>().unwrap();
                if no_ok {
                    return Err(input.error("NO_OKAY specified multiple times"));
                }
                no_ok = true;
            } else {
                return Err(input.error("Unexpected identifier encountered"));
            }
        }
        let block: Block = input.parse()?;
        if !input.is_empty() {
            return Err(input.error("Unexpected trailing characters"));
        }

        Ok(Self {
            day,
            parser,
            output,
            block,
            no_ok,
        })
    }
}

pub fn make_func(input: TokenStream) -> TokenStream {
    let make_func: MakeFunc = parse2(input).expect("Failed to parse MakeFunc");

    let day_name = format_ident!("{}", make_func.day.to_string());
    let parser_name = format_ident!("{}", make_func.parser.to_string());
    let output_type = make_func.output;
    let stmts = make_func.block.stmts;

    let func_proto = quote::quote! {
        pub fn #day_name(input: &[&str]) -> #output_type
    };

    let input_line = if make_func.no_ok {
        quote::quote! {
            let input = input.into_par_iter().map(|line| parser::#parser_name::parse_line(line));
        }
    } else {
        quote::quote! {
            let input = input.into_par_iter().map(|line| parser::#parser_name::parse_line(line).ok()).flatten();
        }
    };

    quote::quote! {
        #func_proto {
            #input_line
            #(#stmts)*
        }
    }
}
