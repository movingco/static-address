//! Provides a macro used for compile-time parsing of Move account addresses into byte arrays for near 0-cost static addresses.

#![deny(rustdoc::all)]

use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn static_address(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    static_address_parser::parse_static_address(parse_macro_input!(input as LitStr)).into()
}
