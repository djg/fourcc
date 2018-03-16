fourcc
======

A Rust proc macro to generate FourCCs.

[![Build Status](https://travis-ci.org/rust-lang/fourcc.svg?branch=master)](https://travis-ci.org/rust-lang/fourcc)


This crate provides a `fourcc!` macro for expressing FourCC values
`fourcc!("asys")` in the place of integer literals.

The `fourcc!` macro accepts a 4 character string literal which will be
converted into a `u32` treating each character as a `u8`.

The order of the resulting `u32` is controlled by an optional,
comma-separated keyword following the string literal. Keyword is one of:

 * `big` - Big Endian ordering - ie `"ABCD"` -> `0xaabbccdd`,
 * `little` - Little Endian ordering - ie `"ABCD"` -> `0xddccbbaa`,
 * `target` - Endian of target.

The default ordering is `big` endian.

# Example

```rust
#[macro_use]
extern crate fourcc;

fn main() {
    assert_eq!(fourcc!("asys"), 0x61737973);
    assert_eq!(fourcc!("asys", big), 0x61737973);
    assert_eq!(fourcc!("asys", little), 0x73797361);

    if cfg!(target_endian = "big") {
        assert_eq!(fourcc!("asys", target), 0x61737973);
    } else {
        assert_eq!(fourcc!("asys", target), 0x73797361);
    }
```

# Usage

Add this to you `Cargo.toml`

```toml
[dependencies]
fourcc = "0.1"
```

and this to your crate root:

```rust
#[macro_use]
extern crate fourcc;
```

# Compatability

This crate supports all versions of Rust (stable and nightly) starting
with Rust 1.15.

# License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opersource.org/licenses/MIT)
 
at your option.

# Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the
Apache-2.0 license, shall be dual licensed as about, without any
additional terms or conditions.

