#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![warn(missing_debug_implementations, missing_copy_implementations, trivial_casts, trivial_numeric_casts, unused_import_braces, unused_qualifications)]
#![allow(non_camel_case_types)]

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

mod consts;
use consts::msgs;

#[cfg(test)]
mod unit_tests;

pub fn lib_main(_args: Vec<String>) -> Result<()>
{
    Ok(())
}
