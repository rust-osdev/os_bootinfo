#![no_std]

extern crate x86_64;
extern crate arrayvec;

use x86_64::PhysAddr;
use arrayvec::ArrayVec;

#[derive(Debug, Clone)]
pub struct BootInfo {
    pub memory_map: ArrayVec<[MemoryRegion; 32]>,
}

impl BootInfo {
    pub fn new() -> Self {
        BootInfo {
            memory_map: ArrayVec::new(),
        }
    }

    pub fn sort_memory_map(&mut self) {
        self.memory_map.sort_unstable_by_key(|r| r.start_addr);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemoryRegion {
    pub start_addr: PhysAddr,
    pub len: u64,
    pub region_type: MemoryRegionType
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryRegionType {
    /// free RAM
    Usable,
    /// used RAM
    InUse,
    /// unusable
    Reserved,
    /// ACPI reclaimable memory
    AcpiReclaimable,
    /// ACPI NVS memory
    AcpiNvs,
    /// Area containing bad memory
    BadMemory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct E820MemoryRegion {
    pub start_addr: u64,
    pub len: u64,
    pub region_type: u32,
    pub acpi_extended_attributes: u32,
}

impl From<E820MemoryRegion> for MemoryRegion {
    fn from(region: E820MemoryRegion) -> MemoryRegion {
        let region_type = match region.region_type {
            1 => MemoryRegionType::Usable,
            2 => MemoryRegionType::Reserved,
            3 => MemoryRegionType::AcpiReclaimable,
            4 => MemoryRegionType::AcpiNvs,
            5 => MemoryRegionType::BadMemory,
            t => panic!("invalid region type {}", t),
        };
        MemoryRegion {
            start_addr: PhysAddr::new(region.start_addr),
            len: region.len,
            region_type
        }
    }
}
