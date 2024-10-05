use proc_macro::TokenStream;
use proc_macro_error2::proc_macro_error;

mod derive;
mod pattern;
mod predicate;

#[proc_macro_error]
#[proc_macro_derive(Classes, attributes(class))]
pub fn derive(input: TokenStream) -> TokenStream {
  derive::derive(input.into()).into()
}

#[doc(hidden)]
#[proc_macro_error]
#[proc_macro]
pub fn _pattern(input: TokenStream) -> TokenStream {
  pattern::pattern(input.into()).into()
}
