#![deny(improper_ctypes)]
#![no_std]

pub use memory_map::*;

const VERSION: u64 = 4;

mod memory_map;

#[derive(Debug)]
#[repr(C)]
pub struct BootInfo {
    pub version: u64,
    pub p4_table_addr: u64,
    pub memory_map: MemoryMap,
}

impl BootInfo {
    pub fn new(p4_table_addr: u64, memory_map: MemoryMap) -> Self {
        BootInfo {
            version: VERSION,
            p4_table_addr,
            memory_map,
        }
    }

    pub fn check_version(&self) -> Result<(), ()> {
        if self.version == VERSION {
            Ok(())
        } else {
            Err(())
        }
    }
}

extern "C" {
    fn _improper_ctypes_check(_boot_info: BootInfo);
}
