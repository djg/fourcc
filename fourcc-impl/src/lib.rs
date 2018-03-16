// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate proc_macro_hack;
#[macro_use]
extern crate syn;

use syn::synom::Synom;

struct FourCC {
    pub lit: syn::LitStr,
    pub endian: Option<syn::Ident>,
}

impl Synom for FourCC {
    named!(parse -> Self, do_parse!(
        lit: syn!(syn::LitStr) >>
        endian: option!(do_parse!(punct!(,) >> id: syn!(syn::Ident) >> (id))) >>
            (FourCC { lit, endian })
    ));
}

fn convert(fcc: FourCC) -> String {
    let little = match fcc.endian {
        None => false,
        Some(id) => match id.as_ref() {
            "little" => true,
            "big" => false,
            "target" => target_little_endian(),
            x => panic!("invalid endian directive `{}` in fourcc!", x),
        },
    };

    if fcc.lit.value().chars().count() != 4 {
        panic!("string literal `{:?}` len != 4 in fourcc!", fcc.lit.value());
    }

    let mut val = 0u32;
    for codepoint in fcc.lit.value().chars().take(4) {
        let byte = if codepoint as u32 > 0xFF {
            panic!("fourcc! literal character out of range 0-255");
        } else {
            codepoint as u8
        };

        val = if little {
            (val >> 8) | ((byte as u32) << 24)
        } else {
            (val << 8) | (byte as u32)
        };
    }
    format!("{}", val)
}

fn target_little_endian() -> bool {
    if cfg!(target_endian = "little") {
        true
    } else {
        false
    }
}

proc_macro_expr_impl! {
    pub fn fourcc_impl(input: &str) -> String {
        match syn::parse_str(input) {
            Ok(fcc) => convert(fcc),
            Err(e) => panic!("fourcc! failed: {}", e),
        }
    }
}
