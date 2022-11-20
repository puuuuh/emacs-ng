#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate emacs;
#[macro_use]
extern crate lisp_util;

pub mod parsing;

#[cfg(not(test))]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/out/c_exports.rs"));
