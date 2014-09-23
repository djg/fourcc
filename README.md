fourcc
======

A Rust syntax extension to generate FourCCs.

[![Build Status](https://travis-ci.org/rust-lang/fourcc.svg?branch=master)](https://travis-ci.org/rust-lang/fourcc)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies.fourcc]

git = "https://github.com/rust-lang/fourcc"
```

and this to your crate root:

```rust
#[phase(plugin)]
extern crate fourcc;
```
