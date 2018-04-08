#![deny(improper_ctypes)]
#![no_std]

extern crate x86_64;

pub use memory_map::*;

use x86_64::structures::paging::PageTable;

const VERSION: u64 = 2;

mod memory_map;

#[derive(Debug)]
#[repr(C)]
pub struct BootInfo {
    pub version: u64,
    pub p4_table: &'static mut PageTable,
    pub memory_map: MemoryMap,
}

impl BootInfo {
    pub fn new(p4_table: &'static mut PageTable, memory_map: MemoryMap) -> Self {
        BootInfo {
            version: VERSION,
            p4_table,
            memory_map
        }
    }

    pub fn check_version(&self) -> Result<(), ()> {
        if self.version == VERSION { Ok(()) } else { Err(()) }
    }
}

extern "C" {
    fn _improper_ctypes_check(_boot_info: BootInfo);
}
