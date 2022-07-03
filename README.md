# static-address

[![Crates.io](https://img.shields.io/crates/v/static-address)](https://crates.io/crates/static-address)
[![License](https://img.shields.io/crates/l/static-address)](https://github.com/movingco/static-address/blob/master/LICENSE.txt)
[![Build Status](https://img.shields.io/github/workflow/status/movingco/static-address/Rust/master)](https://github.com/movingco/static-address/actions/workflows/rust.yml?query=branch%3Amaster)
[![Contributors](https://img.shields.io/github/contributors/movingco/static-address)](https://github.com/movingco/static-address/graphs/contributors)
[![Code Coverage](https://img.shields.io/codecov/c/github/movingco/static-address)](https://app.codecov.io/gh/movingco/static-address)

The `static-address` crate provides a macro `static_address!`, used for compile-time parsing of strings into a static address. This provides an efficient way of declaring Move account addresses in source code while incurring almost no runtime cost in programs, without having to declare the byte array yourself. The actual code is adapted from [here](https://github.com/project-serum/anchor/commit/96036e149173603926074c6dba445c47bd6575aa).

## Usage

First install the [`mv-core-types`](https://crates.io/crates/mv-core-types) crate.

Then, use the following code:

```rust
let key: AccountAddress =
    static_address!("0x80809acd8d3bc3d30aea82e5506f45951e3eb53e2fda39da6d772647e52c25cd");
assert_eq!(
    key.to_hex_literal(),
    "0x80809acd8d3bc3d30aea82e5506f45951e3eb53e2fda39da6d772647e52c25cd"
);
```

An example of using this for an Aptos project is available at [crates/static-address-example-aptos](./crates/static-address-example-aptos/).

## License

`static-address` is licensed under the Apache License, version 2.0.
