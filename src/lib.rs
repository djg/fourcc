// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Syntax extension to generate FourCCs.
//!
//! Once loaded, fourcc!() is called with a single 4-character string,
//! and an optional ident that is either `big`, `little`, or `target`.
//! The ident represents endianness, and specifies in which direction
//! the characters should be read. If the ident is omitted, it is assumed
//! to be `big`, i.e. left-to-right order. It returns a u32.
//!
//! # Examples
//!
//! To load the extension and use it:
//!
//! ```rust,ignore
//! #[macro_use]
//! extern crate fourcc;
//!
//! fn main() {
//!     let val = fourcc!("\xC0\xFF\xEE!");
//!     assert_eq!(val, 0xC0FFEE21u32);
//!     let little_val = fourcc!("foo ", little);
//!     assert_eq!(little_val, 0x21EEFFC0u32);
//! }
//! ```
//!
//! # References
//!
//! * [Wikipedia: FourCC](http://en.wikipedia.org/wiki/FourCC)

#![doc(html_logo_url = "http://www.rust-lang.org/logos/rust-logo-128x128-blk-v2.png",
       html_favicon_url = "http://www.rust-lang.org/favicon.ico")]

#[macro_use]
extern crate proc_macro_hack;

#[allow(unused_imports)]
#[macro_use]
extern crate fourcc_impl;

#[doc(hidden)]
pub use fourcc_impl::*;

proc_macro_expr_decl! {
    fourcc! => fourcc_impl
}
