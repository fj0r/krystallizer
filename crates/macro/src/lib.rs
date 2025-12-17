mod attrs;
mod config;
use attrs::Attrs;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::quote;
use std::collections::HashMap;
use std::fs::read_to_string;
use syn::{Error, parse_macro_input};

macro_rules! bail {
    ($($x: tt)*) => {
        return Error::new(Span::call_site(), format!($($x)*))
            .into_compile_error()
            .into()
    };
}

#[proc_macro]
pub fn example(input: TokenStream) -> TokenStream {
    let config = parse_macro_input!(input as Attrs)
        .0
        .into_iter()
        .collect::<HashMap<_, _>>();

    let Some(file) = config.get("file") else {
        bail!("must provide file");
    };

    quote! {
        const _: &[u8] = include_bytes!(#file);
    }
    .into()
}
