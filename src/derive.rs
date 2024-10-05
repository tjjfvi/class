use std::{
  collections::{BTreeMap, BTreeSet},
  hash::{DefaultHasher, Hasher},
  io::{self, Write},
};

use proc_macro2::TokenStream;
use proc_macro_error2::emit_error;
use quote::{format_ident, quote};
use syn::{
  parse::{Parse, ParseStream},
  punctuated::Punctuated,
  token::Comma,
  Ident, ItemEnum, Visibility,
};

pub fn derive(input: TokenStream) -> TokenStream {
  let hash = hash_tokens(&input);
  let input = match syn::parse2::<ItemEnum>(input) {
    Ok(data) => data,
    Err(err) => return TokenStream::from(err.to_compile_error()),
  };

  let variant_classes = input
    .variants
    .iter()
    .map(|variant| {
      (
        variant.ident.clone(),
        variant
          .attrs
          .iter()
          .filter(|attr| attr.meta.path().is_ident("class"))
          .map(|attr| syn::parse2::<ClassList>(attr.meta.require_list()?.tokens.clone()))
          .filter_map(|x| match x {
            Ok(c) => Some(c.0),
            Err(e) => {
              emit_error!(e);
              None
            }
          })
          .flatten()
          .collect::<BTreeSet<_>>(),
      )
    })
    .collect::<BTreeMap<_, _>>();

  let vis = input.vis;
  let export = if matches!(vis, Visibility::Public(..)) {
    quote!(#[macro_export])
  } else {
    quote!()
  };

  let macro_name = format_ident!("__class_{}_{:016x}", input.ident, hash);
  let enum_name = input.ident;

  let variant_classes = variant_classes
    .into_iter()
    .map(|(variant, classes)| quote!(#variant [#(#classes)*]));

  quote![
    #export
    #[doc(hidden)]
    macro_rules! #macro_name {
      ($($x:tt)*) => {
        ::class::_pattern!(#enum_name [#(#variant_classes)*]; $($x)*)
      }
    }
    #vis use #macro_name as #enum_name;
  ]
}

struct ClassList(Punctuated<Ident, Comma>);

impl Parse for ClassList {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    Ok(ClassList(Punctuated::parse_terminated(input)?))
  }
}

fn hash_tokens(tokens: &TokenStream) -> u64 {
  let mut hasher = DefaultHasher::new();
  _ = write!(WriteHasher(&mut hasher), "{}", tokens);
  hasher.finish()
}

struct WriteHasher<'a, H: Hasher>(&'a mut H);

impl<'a, H: Hasher> Write for WriteHasher<'a, H> {
  fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
    self.0.write(buf);
    Ok(buf.len())
  }

  fn flush(&mut self) -> io::Result<()> {
    Ok(())
  }
}
