pub mod msgs;
pub use self::msgs::*;

//TODO: Use dependency injection instead of conditional compilation
#[cfg(not(test))]
pub const MEMORY_MAP_DEVICE_FILENAME_VALUE: &'static str = "/dev/mem";
#[cfg(test)]
pub const MEMORY_MAP_DEVICE_FILENAME_VALUE: &'static str = "./fake_mmap_file.test";

pub const MEMORY_MAP_DEVICE_FILENAME: &'static str = MEMORY_MAP_DEVICE_FILENAME_VALUE;
pub const GPIO_BASE: usize = 0x3F20_0000;
pub const GPIO_SIZE: usize = 0x1000;
