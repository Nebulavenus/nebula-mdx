#![recursion_limit = "1024"]
extern crate proc_macro;
extern crate syn;

mod attr;
mod bts;
mod de;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(NMread, attributes(nebula))]
pub fn derive_nmread(input: TokenStream) -> TokenStream {
    de::derive(parse_macro_input!(input as DeriveInput))
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

#[proc_macro_derive(NMbts, attributes(nebula))]
pub fn derive_bts(input: TokenStream) -> TokenStream {
    bts::derive(parse_macro_input!(input as DeriveInput))
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
