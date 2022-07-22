//! Example of using the static-address crate with Aptos code.

#![cfg(test)]
#![deny(rustdoc::all)]
#![allow(rustdoc::missing_doc_code_examples)]

use static_address::static_address;

#[test]
fn test_aptos_address() {
    let key = static_address!("0x80809acd8d3bc3d30aea82e5506f45951e3eb53e2fda39da6d772647e52c25cd");
    assert_eq!(
        key.to_hex_literal(),
        "0x80809acd8d3bc3d30aea82e5506f45951e3eb53e2fda39da6d772647e52c25cd"
    );
}

#[test]
fn test_aptos_address_short() {
    let key = static_address!("0x1");
    assert_eq!(key.to_hex_literal(), "0x1");
}
