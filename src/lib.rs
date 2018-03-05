#![deny(improper_ctypes)]
#![no_std]

extern crate x86_64;

pub use memory_map::*;

use x86_64::structures::paging::PageTable;

mod memory_map;

#[derive(Debug)]
#[repr(C)]
pub struct BootInfo {
    pub memory_map: MemoryMap,
    pub p4_table: &'static mut PageTable,
}

impl BootInfo {
    pub fn new(p4_table: &'static mut PageTable) -> Self {
        BootInfo {
            memory_map: ArrayVec::new(),
            p4_table
        }
    }

    pub fn sort_memory_map(&mut self) {
        self.memory_map.sort_unstable_by_key(|r| r.start_addr);
    }
}

extern "C" {
    fn _improper_ctypes_check(_boot_info: BootInfo);
}
