mod makebenches;
mod makefunc;
mod maketests;
mod util;

use proc_macro::TokenStream;

#[proc_macro]
pub fn make_tests(input: TokenStream) -> TokenStream {
    maketests::make_tests(input.into()).into()
}

#[proc_macro]
pub fn make_benches(input: TokenStream) -> TokenStream {
    makebenches::make_benches(input.into()).into()
}

#[proc_macro]
pub fn make_func(input: TokenStream) -> TokenStream {
    makefunc::make_func(input.into()).into()
}
