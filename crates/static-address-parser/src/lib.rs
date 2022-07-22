//! Parser used by the static-address macro crate.

#![deny(rustdoc::all)]
#![allow(rustdoc::missing_doc_code_examples)]

pub use account_address;
use account_address::AccountAddress;
use proc_macro2::Span;
use quote::quote;
use syn::{LitByte, LitStr};

/// Parses a static address literal to an [AccountAddress].
pub fn parse_static_address(id_literal: LitStr) -> proc_macro2::TokenStream {
    let bytes = parse_static_address_as_array(id_literal);
    quote! {
        ::static_address::account_address::AccountAddress::new(#bytes)
    }
}

/// Parses a static address literal to a byte array.
pub fn parse_static_address_as_array(id_literal: LitStr) -> proc_macro2::TokenStream {
    let id_vec = AccountAddress::from_hex_literal(&id_literal.value())
        .map_err(|err| {
            syn::Error::new_spanned(
                &id_literal,
                format!("failed to decode hex address string: {}", err),
            )
        })
        .unwrap()
        .to_vec();
    let id_array = <[u8; AccountAddress::LENGTH]>::try_from(<&[u8]>::clone(&&id_vec[..]))
        .map_err(|_| {
            syn::Error::new_spanned(
                &id_literal,
                format!(
                    "address array is not {} bytes long: len={}",
                    AccountAddress::LENGTH,
                    id_vec.len(),
                ),
            )
        })
        .unwrap();
    let bytes = id_array.iter().map(|b| LitByte::new(*b, Span::call_site()));
    quote! { [#(#bytes,)*] }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address() {
        let key = parse_static_address_as_array(syn::parse_str("\"0x12\"").unwrap());
        let length = AccountAddress::LENGTH;
        // 16 byte address ending in 0x12 (18u8)
        let expected = format!("[{}{} ,]", "0u8 , ".repeat(length - 1), "18u8");
        assert_eq!(key.to_string(), expected);
    }
}
