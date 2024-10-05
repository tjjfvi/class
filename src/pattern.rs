use std::collections::{BTreeMap, BTreeSet};

use proc_macro2::TokenStream;
use quote::quote_spanned;
use syn::{
  bracketed,
  parse::{Parse, ParseStream},
  spanned::Spanned,
  token::Semi,
  Expr, Ident,
};

use crate::predicate::{eval_predicate, validate_predicate};

pub fn pattern(input: TokenStream) -> TokenStream {
  let Input {
    mut enum_name,
    variant_classes,
    predicate,
  } = match syn::parse2::<Input>(input) {
    Ok(data) => data,
    Err(err) => return TokenStream::from(err.to_compile_error()),
  };

  let all_classes = variant_classes
    .values()
    .flatten()
    .cloned()
    .collect::<BTreeSet<_>>();

  validate_predicate(&predicate, &all_classes);

  let span = predicate.span();
  enum_name.set_span(span);

  let variants = variant_classes
    .into_iter()
    .filter(|(_, classes)| eval_predicate(&predicate, &classes))
    .map(|(mut variant, _)| {
      variant.set_span(span);
      quote_spanned![span=> #enum_name::#variant {..}]
    });

  quote_spanned![span=> (#(#variants)|*)].into()
}

#[derive(Debug)]
struct Input {
  enum_name: Ident,
  variant_classes: BTreeMap<Ident, BTreeSet<Ident>>,
  predicate: Expr,
}

impl Parse for Input {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let enum_name = input.parse()?;
    let inner;
    bracketed!(inner in input);
    let mut variants = BTreeMap::new();
    while !inner.is_empty() {
      let variant = inner.parse()?;
      let mut classes = BTreeSet::new();
      let list;
      bracketed!(list in inner);
      while !list.is_empty() {
        classes.insert(list.parse()?);
      }
      variants.insert(variant, classes);
    }
    input.parse::<Semi>()?;
    let expr = input.parse()?;
    Ok(Input {
      enum_name,
      variant_classes: variants,
      predicate: expr,
    })
  }
}
