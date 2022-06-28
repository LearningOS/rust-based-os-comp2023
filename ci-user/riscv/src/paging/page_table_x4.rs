/// This file is for Hypervisor-related x4 page tables, including Sv32x4, Sv39x4 and Sv48x4.
/// In fact, these x4 page tables are Phys-to-Phys page tables from GPAs to real PAs.
use super::page_table::{
    PTEIterableSlice, PageTableEntryX32, PageTableEntryX64, PageTableWith, RV32_ENTRY_COUNT,
    RV64_ENTRY_COUNT,
};

// The root page table is 4 times larger.
pub const RV32_X4_ENTRY_COUNT: usize = RV32_ENTRY_COUNT << 2;
pub const RV64_X4_ENTRY_COUNT: usize = RV64_ENTRY_COUNT << 2;

pub type Entries32X4 = [PageTableEntryX32; RV32_X4_ENTRY_COUNT];
pub type Entries64X4 = [PageTableEntryX64; RV64_X4_ENTRY_COUNT];

impl PTEIterableSlice<PageTableEntryX32> for Entries32X4 {
    fn to_pte_slice(&self) -> &[PageTableEntryX32] {
        self
    }
    fn to_pte_slice_mut(&mut self) -> &mut [PageTableEntryX32] {
        self
    }
    fn pte_index(&self, index: usize) -> &PageTableEntryX32 {
        &self[index]
    }
    fn pte_index_mut(&mut self, index: usize) -> &mut PageTableEntryX32 {
        &mut self[index]
    }
}
impl PTEIterableSlice<PageTableEntryX64> for Entries64X4 {
    fn to_pte_slice(&self) -> &[PageTableEntryX64] {
        self
    }
    fn to_pte_slice_mut(&mut self) -> &mut [PageTableEntryX64] {
        self
    }
    fn pte_index(&self, index: usize) -> &PageTableEntryX64 {
        &self[index]
    }
    fn pte_index_mut(&mut self, index: usize) -> &mut PageTableEntryX64 {
        &mut self[index]
    }
}
pub type PageTable32X4 = PageTableWith<Entries32X4, PageTableEntryX32>;
pub type PageTable64X4 = PageTableWith<Entries64X4, PageTableEntryX64>;
