#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![allow(non_camel_case_types)]
#![warn(missing_debug_implementations, missing_copy_implementations, trivial_casts, trivial_numeric_casts, unused_import_braces, unused_qualifications)]
#![deny(unused_must_use, overflowing_literals)]

extern crate memmap;
extern crate byteorder;
extern crate num;

type GeneralError = Box<std::error::Error>;
type GeneralResult<T> = Result<T, GeneralError>;

mod consts;
use consts::*;
use memmap::{Mmap, Protection};
use std::fs::OpenOptions;
use std::ptr;
use byteorder::{ByteOrder, LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::Write;

#[cfg(test)]
mod unit_tests;

pub fn lib_main(_args: Vec<String>) -> GeneralResult<()> {

    //Set up GPIO
    let mut gpio = Mapping::new(GPIO_BASE, GPIO_SIZE)?;
    println!("GPIO has been set up");

    //Set GPIO5 (pin 29) as output
    let gpclr = 0x28;
    gpio.write(gpclr, 0b1 << 5)?;

    let gpfsel0 = 0x00;
    let gpio5_mask = 0b111 << 15;
    let register = gpio.read(gpfsel0)? & !gpio5_mask;
    gpio.write(gpfsel0, register | 0b1 << 15)?;
    println!("GPIO5 set as output");

    //flush?  hope not

    //toggle
    let gplev0 = 0x34;
    println!("GPIO5: {}", gpio.read(gplev0)? & 0b1 << 5 >> 5);
    let gpset = 0x1c;
    gpio.write(gpset, 0b1 << 5)?;
    println!("GPIO5 enabled");
    println!("GPIO5: {}", gpio.read(gplev0)? & 0b1 << 5 >> 5);

    Ok(())
}

struct Mapping {
    map: Mmap,
}

impl Mapping {
    fn new(base: usize, size: usize) -> GeneralResult<Mapping> {
        //Map GPIO into user space
        let fd = OpenOptions::new().read(true)
                                   .write(true)
                                   .open(MEMORY_MAP_DEVICE_FILENAME)?;
        Ok(Mapping { map: Mmap::open_with_offset(&fd, Protection::ReadWrite,
                                                   base,
                                                   size)? })
    }

    fn read(&self, offset: usize) -> Result<u32, std::io::Error> {
        Ok(unsafe { &(self.map.as_slice()[offset..offset + 4]) }
        .read_u32::<LittleEndian>()?)
    }

    //extern crate memmap, extern crate byteorder are in effect
    fn write(&mut self, offset: usize, value: u32) -> Result<(), std::io::Error> {
        unsafe { &mut self.map.as_mut_slice()[offset..offset + 4] }
            .write_u32::<LittleEndian>(value)
    }
}
