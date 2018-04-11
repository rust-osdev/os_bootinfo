use core::ops::{Deref, DerefMut};
use core::fmt::{self, Debug};
use x86_64::PhysAddr;
use x86_64::structures::paging::{PhysFrame, PhysFrameRange};

#[repr(C)]
pub struct MemoryMap {
    entries: [MemoryRegion; 32],
    // u64 instead of usize so that the structure layout is platform
    // independent
    next_entry_index: u64,
}

impl MemoryMap {
    pub fn new() -> Self {
        MemoryMap {
            entries: [MemoryRegion::empty(); 32],
            next_entry_index: 0,
        }
    }

    pub fn add_region(&mut self, region: MemoryRegion) {
        self.entries[self.next_entry_index()] = region;
        self.next_entry_index += 1;
        self.sort();
    }

    pub fn sort(&mut self) {
        use core::cmp::Ordering;

        self.entries.sort_unstable_by(|r1, r2|
            if r1.range.is_empty() {
                Ordering::Greater
            } else if r2.range.is_empty() {
                Ordering::Less
            } else {
                r1.range.start.cmp(&r2.range.start)
            }
        );
        if let Some(first_zero_index) = self.entries.iter()
            .position(|r| r.range.is_empty())
        {
            self.next_entry_index = first_zero_index as u64;
        }
    }

    fn next_entry_index(&self) -> usize {
        self.next_entry_index as usize
    }
}

impl Deref for MemoryMap {
    type Target = [MemoryRegion];

    fn deref(&self) -> &Self::Target {
        &self.entries[0..self.next_entry_index()]
    }
}

impl DerefMut for MemoryMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let next_index = self.next_entry_index();
        &mut self.entries[0..next_index]
    }
}

impl Debug for MemoryMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct MemoryRegion {
    pub range: PhysFrameRange,
    pub region_type: MemoryRegionType
}

impl MemoryRegion {
    pub fn empty() -> Self {
        let zero = PhysFrame::containing_address(PhysAddr::new(0));
        MemoryRegion {
            range: PhysFrame::range(zero, zero),
            region_type: MemoryRegionType::Empty,
        }
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
    /// kernel stack memory
    KernelStack,
    /// memory used by page tables
    PageTable,
    /// memory used by the bootloader
    Bootloader,
    /// frame at address zero
    ///
    /// (shouldn't be used because it's easy to make mistakes related to null pointers)
    FrameZero,
    /// an empty region with size 0
    Empty,
    /// used for storing the boot information
    BootInfo,
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
        let start_addr = PhysAddr::new(region.start_addr);
        let start_frame = PhysFrame::containing_address(start_addr);
        let end_addr = (start_addr + region.len).align_up(start_frame.size());
        let end_frame = PhysFrame::containing_address(end_addr);
        MemoryRegion {
            range: PhysFrame::range(start_frame, end_frame),
            region_type
        }
    }
}

extern "C" {
    fn _improper_ctypes_check(_boot_info: MemoryMap);
}
