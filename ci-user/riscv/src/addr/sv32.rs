use super::*;
use bit_field::BitField;
use core::convert::TryInto;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VirtAddrSv32(u32);
impl Address for VirtAddrSv32 {
    fn new(addr: usize) -> Self {
        VirtAddrSv32(addr.try_into().unwrap())
    }
    fn as_usize(&self) -> usize {
        self.0 as usize
    }
    fn page_number(&self) -> usize {
        self.0.get_bits(12..32) as usize
    }
    fn page_offset(&self) -> usize {
        self.0.get_bits(0..12) as usize
    }
    fn to_4k_aligned(&self) -> Self {
        VirtAddrSv32((self.0 >> 12) << 12)
    }
}
impl VirtualAddress for VirtAddrSv32 {
    unsafe fn as_mut<'a, 'b, T>(&'a self) -> &'b mut T {
        &mut *(self.0 as *mut T)
    }
}

impl AddressL2 for VirtAddrSv32 {
    fn p2_index(&self) -> usize {
        self.0.get_bits(22..32) as usize
    }

    fn p1_index(&self) -> usize {
        self.0.get_bits(12..22) as usize
    }
    fn from_page_table_indices(p2_index: usize, p1_index: usize, offset: usize) -> Self {
        assert!(p2_index.get_bits(10..) == 0, "p2_index exceeding 10 bits");
        assert!(p1_index.get_bits(10..) == 0, "p1_index exceeding 10 bits");
        assert!(offset.get_bits(12..) == 0, "offset exceeding 12 bits");
        VirtAddrSv32::new((p2_index << 22) | (p1_index << 12) | offset)
    }
}

impl AddressX32 for VirtAddrSv32 {
    fn new_u32(addr: u32) -> Self {
        VirtAddrSv32(addr)
    }
    fn as_u32(&self) -> u32 {
        self.0
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PhysAddrSv32(u64);
impl Address for PhysAddrSv32 {
    fn new(addr: usize) -> Self {
        Self::new_u64(addr as u64)
    }
    fn as_usize(&self) -> usize {
        assert!(
            self.0.get_bits(32..34) == 0,
            "Downcasting an Sv32 pa >4GB (32..34!=0) will cause address loss."
        );
        self.0 as usize
    }
    fn page_number(&self) -> usize {
        self.0.get_bits(12..34) as usize
    }
    fn page_offset(&self) -> usize {
        self.0.get_bits(0..12) as usize
    }
    fn to_4k_aligned(&self) -> Self {
        PhysAddrSv32((self.0 >> 12) << 12)
    }
}

impl AddressX64 for PhysAddrSv32 {
    fn new_u64(addr: u64) -> Self {
        assert!(
            addr.get_bits(34..64) == 0,
            "Sv32 does not allow pa 34..64!=0"
        );
        PhysAddrSv32(addr)
    }
    fn as_u64(&self) -> u64 {
        self.0
    }
}

impl PhysicalAddress for PhysAddrSv32 {}
