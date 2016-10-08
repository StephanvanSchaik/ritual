#![forbid(unused_must_use)]

// #![allow(unknown_lints)]

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![cfg_attr(feature="clippy", warn(nonminimal_bool))]
#![cfg_attr(feature="clippy", warn(if_not_else))]
#![cfg_attr(feature="clippy", warn(shadow_same))]
#![cfg_attr(feature="clippy", warn(shadow_unrelated))]
#![cfg_attr(feature="clippy", warn(single_match_else))]

// sometime in the future...
// #![warn(option_unwrap_used)]
// #![warn(result_unwrap_used)]
// #![warn(print_stdout)]

mod cpp_ffi_generator;
mod cpp_code_generator;
mod caption_strategy;
mod cpp_data;
mod cpp_ffi_data;
mod cpp_method;
mod cpp_type;
mod cpp_operator;
mod dependency_info;
mod doc_formatter;
pub mod log;
mod qt_doc_parser;
mod qt_specific;
mod rust_generator;
mod rust_code_generator;
mod rust_info;
mod rust_type;
mod utils;
mod cpp_parser;
mod serializable;
pub mod launcher;

#[cfg(test)]
mod tests;
