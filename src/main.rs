#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![warn(missing_debug_implementations, missing_copy_implementations, trivial_casts, trivial_numeric_casts, unused_import_braces, unused_qualifications)]
#![allow(non_camel_case_types)]

extern crate hello_led;
mod consts;
use consts::*;

pub fn main()
{
    let args = std::env::args_os().map(|arg| arg.into_string().expect(INVALID_UTF8_ARG))
                                  .collect::<Vec<_>>();
    let app_name = match args.first()
    {
        Some(name) => name.to_string(),
        _ => UNKNOWN_APP_NAME.to_string(),
    };

    match hello_led::lib_main(args)
    {
        Err(e) => panic!(format!("{}: {}", app_name, e.description())),
        _ => (),
    }
}
