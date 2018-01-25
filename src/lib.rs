#![no_std]

extern crate x86_64;

use x86_64::PhysAddr;

#[repr(C)]
pub struct BootInfo {
    pub memory_map: &'static mut [E820MemoryRegion],
}

#[repr(C)]
pub struct E820MemoryRegion {
    pub start_addr: PhysAddr,
    pub length: u64,
    pub region_type: MemoryRegionType,
    pub acpi_extended_attributes: u32,
}

#[repr(u32)]
pub enum MemoryRegionType {
    /// (normal) RAM
    Usable = 1,
    /// unusable
    Reserved = 2,
    /// ACPI reclaimable memory
    AcpiReclaimable = 3,
    /// ACPI NVS memory
    AcpiNvs = 4,
    /// Area containing bad memory
    BadMemory = 5,
    /// used by bootloader (e.g. to create page tables)
    InUse = 0xb007,
}
