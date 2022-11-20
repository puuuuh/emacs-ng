#![cfg_attr(feature = "strict", deny(warnings))]

mod attributes;

// concat_idents polyfill
pub use paste::paste;

// Used by remacs-macros and remacs-lib
pub use self::attributes::parse_lisp_fn;
