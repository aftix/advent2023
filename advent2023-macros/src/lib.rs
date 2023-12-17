mod makebenches;
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
