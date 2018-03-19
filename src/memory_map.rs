use core::ops::{Deref, DerefMut};
use x86_64::PhysAddr;

#[derive(Debug)]
#[repr(C)]
pub struct MemoryMap {
    entries: [MemoryRegion; 32],
    last_used_entry: u64,
}

impl MemoryMap {
    pub fn new() -> Self {
        MemoryMap {
            entries: [MemoryRegion::empty(); 32],
            last_used_entry: 0,
        }
    }

    pub fn add_region(&mut self, region: MemoryRegion) {
        self.entries[self.last_used_entry as usize] = region;
        self.last_used_entry += 1;
    }
}

impl Deref for MemoryMap {
    type Target = [MemoryRegion];

    fn deref(&self) -> &Self::Target {
        &self.entries
    }
}

impl DerefMut for MemoryMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entries
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct MemoryRegion {
    pub start_addr: PhysAddr,
    pub len: u64,
    pub region_type: MemoryRegionType
}

impl MemoryRegion {
    pub fn empty() -> Self {
        MemoryRegion {
            start_addr: PhysAddr::new(0),
            len: 0,
            region_type: MemoryRegionType::Reserved,
        }
    }

    pub fn start_addr(&self) -> PhysAddr {
        self.start_addr
    }

    pub fn end_addr(&self) -> PhysAddr {
        self.start_addr + self.len
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
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
    /// kernel memory
    Kernel,
    /// memory used by page tables
    PageTable,
    /// memory used by the bootloader
    Bootloader,
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

extern "C" {
    fn _improper_ctypes_check(_boot_info: MemoryMap);
}
