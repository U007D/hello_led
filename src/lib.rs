#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![warn(missing_debug_implementations, missing_copy_implementations, trivial_casts, trivial_numeric_casts, unused_import_braces, unused_qualifications)]
#![allow(non_camel_case_types)]

extern crate memmap;
extern crate owning_ref;

type GeneralError = Box<std::error::Error>;
type GeneralResult<T> = Result<T, GeneralError>;

mod consts;
use consts::*;
use memmap::{Mmap, Protection};
use owning_ref::OwningRef;
use std::fs::OpenOptions;

#[cfg(test)]
mod unit_tests;

pub fn lib_main(_args: Vec<String>) -> GeneralResult<()> {
    let mmap = map_gpio()?;
    let gpio = &*mmap;
    println!("GPIO has been set up");

    //Set GPIO4 (pin 7) as output
    Ok(())
}

fn map_gpio() -> GeneralResult<OwningRef<Box<Mmap>, [u8]>> {
    let fd = OpenOptions::new().read(true)
                               .write(true)
                               .open(consts::MEMORY_MAP_DEVICE_FILENAME)?;
    let gpio_map = OwningRef::new(Box::new(Mmap::open_with_offset(&fd, Protection::ReadWrite,
                                                                  GPIO_BASE,
                                                                  GPIO_SIZE)?));
    Ok(gpio_map.map(|owner| unsafe { owner.as_slice() }))
}
