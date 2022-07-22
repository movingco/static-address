//! Provides a macro used for compile-time parsing of Move account addresses into byte arrays for near 0-cost static addresses.

#![deny(rustdoc::all)]

pub use account_address;

/// Parses a string literal account address into a byte array account address.
///
/// # Arguments
///
/// * `input` - A public key string
///
/// # Examples
///
/// ```
/// use static_address::static_address;
/// let key = static_address!("0x1");
/// assert_eq!(key.to_hex_literal(), "0x1");
/// ```
pub use static_address_macro::static_address;
