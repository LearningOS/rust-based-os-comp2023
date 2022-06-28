use super::*;
use bit_field::BitField;
use core::convert::TryInto;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct GPAddrSv32X4(u64);

impl Address for GPAddrSv32X4 {
    fn new(addr: usize) -> Self {
        Self::new_u64(addr as u64)
    }
    fn as_usize(&self) -> usize {
        self.0 as usize
    }
    fn page_number(&self) -> usize {
        self.0.get_bits(12..34) as usize
    }
    fn page_offset(&self) -> usize {
        self.0.get_bits(0..12) as usize
    }
    fn to_4k_aligned(&self) -> Self {
        GPAddrSv32X4((self.0 >> 12) << 12)
    }
}

impl VirtualAddress for GPAddrSv32X4 {
    unsafe fn as_mut<'a, 'b, T>(&'a self) -> &'b mut T {
        &mut *(self.0 as *mut T)
    }
}

impl AddressL2 for GPAddrSv32X4 {
    fn p2_index(&self) -> usize {
        self.0.get_bits(22..34) as usize
    }
    fn p1_index(&self) -> usize {
        self.0.get_bits(12..22) as usize
    }
    fn from_page_table_indices(p2_index: usize, p1_index: usize, offset: usize) -> Self {
        let p2_index = p2_index as u64;
        let p1_index = p1_index as u64;
        let offset = offset as u64;
        assert!(p2_index.get_bits(12..) == 0, "p2_index exceeding 12 bits");
        assert!(p1_index.get_bits(10..) == 0, "p1_index exceeding 10 bits");
        assert!(offset.get_bits(12..) == 0, "offset exceeding 12 bits");
        GPAddrSv32X4::new_u64((p2_index << 22) | (p1_index << 12) | offset)
    }
}

impl AddressX64 for GPAddrSv32X4 {
    fn new_u64(addr: u64) -> Self {
        assert!(
            addr.get_bits(34..64) == 0,
            "Sv32x4 does not allow pa 34..64!=0"
        );
        GPAddrSv32X4(addr)
    }
    fn as_u64(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct GPAddrSv39X4(u64);

impl Address for GPAddrSv39X4 {
    fn new(addr: usize) -> Self {
        GPAddrSv39X4(addr.try_into().unwrap())
    }
    fn as_usize(&self) -> usize {
        self.0 as usize
    }
    fn page_number(&self) -> usize {
        self.0.get_bits(12..41) as usize
    }
    fn page_offset(&self) -> usize {
        self.0.get_bits(0..12) as usize
    }
    fn to_4k_aligned(&self) -> Self {
        GPAddrSv39X4((self.0 >> 12) << 12)
    }
}

impl VirtualAddress for GPAddrSv39X4 {
    unsafe fn as_mut<'a, 'b, T>(&'a self) -> &'b mut T {
        &mut *(self.0 as *mut T)
    }
}

impl AddressL3 for GPAddrSv39X4 {
    fn p3_index(&self) -> usize {
        self.0.get_bits(30..41) as usize
    }
    fn p2_index(&self) -> usize {
        self.0.get_bits(21..30) as usize
    }
    fn p1_index(&self) -> usize {
        self.0.get_bits(12..21) as usize
    }
    fn from_page_table_indices(
        p3_index: usize,
        p2_index: usize,
        p1_index: usize,
        offset: usize,
    ) -> Self {
        let p3_index = p3_index as u64;
        let p2_index = p2_index as u64;
        let p1_index = p1_index as u64;
        let offset = offset as u64;
        assert!(p3_index.get_bits(11..) == 0, "p3_index exceeding 11 bits");
        assert!(p2_index.get_bits(9..) == 0, "p2_index exceeding 9 bits");
        assert!(p1_index.get_bits(9..) == 0, "p1_index exceeding 9 bits");
        assert!(offset.get_bits(12..) == 0, "offset exceeding 12 bits");
        GPAddrSv39X4::new_u64(
            (p3_index << 12 << 9 << 9) | (p2_index << 12 << 9) | (p1_index << 12) | offset,
        )
    }
}

impl AddressX64 for GPAddrSv39X4 {
    fn new_u64(addr: u64) -> Self {
        assert!(
            addr.get_bits(41..64) == 0,
            "Sv39x4 does not allow pa 41..64!=0"
        );
        GPAddrSv39X4(addr)
    }
    fn as_u64(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct GPAddrSv48X4(u64);

impl Address for GPAddrSv48X4 {
    fn new(addr: usize) -> Self {
        GPAddrSv48X4(addr.try_into().unwrap())
    }
    fn as_usize(&self) -> usize {
        self.0 as usize
    }
    fn page_number(&self) -> usize {
        self.0.get_bits(12..50) as usize
    }
    fn page_offset(&self) -> usize {
        self.0.get_bits(0..12) as usize
    }
    fn to_4k_aligned(&self) -> Self {
        GPAddrSv48X4((self.0 >> 12) << 12)
    }
}

impl VirtualAddress for GPAddrSv48X4 {
    unsafe fn as_mut<'a, 'b, T>(&'a self) -> &'b mut T {
        &mut *(self.0 as *mut T)
    }
}

impl AddressL4 for GPAddrSv48X4 {
    fn p4_index(&self) -> usize {
        self.0.get_bits(39..50) as usize
    }
    fn p3_index(&self) -> usize {
        self.0.get_bits(30..39) as usize
    }
    fn p2_index(&self) -> usize {
        self.0.get_bits(21..30) as usize
    }
    fn p1_index(&self) -> usize {
        self.0.get_bits(12..21) as usize
    }
    fn from_page_table_indices(
        p4_index: usize,
        p3_index: usize,
        p2_index: usize,
        p1_index: usize,
        offset: usize,
    ) -> Self {
        let p4_index = p4_index as u64;
        let p3_index = p3_index as u64;
        let p2_index = p2_index as u64;
        let p1_index = p1_index as u64;
        let offset = offset as u64;
        assert!(p4_index.get_bits(11..) == 0, "p4_index exceeding 11 bits");
        assert!(p3_index.get_bits(9..) == 0, "p3_index exceeding 9 bits");
        assert!(p2_index.get_bits(9..) == 0, "p2_index exceeding 9 bits");
        assert!(p1_index.get_bits(9..) == 0, "p1_index exceeding 9 bits");
        assert!(offset.get_bits(12..) == 0, "offset exceeding 12 bits");
        GPAddrSv48X4::new_u64(
            (p4_index << 12 << 9 << 9 << 9)
                | (p3_index << 12 << 9 << 9)
                | (p2_index << 12 << 9)
                | (p1_index << 12)
                | offset,
        )
    }
}

impl AddressX64 for GPAddrSv48X4 {
    fn new_u64(addr: u64) -> Self {
        assert!(
            addr.get_bits(50..64) == 0,
            "Sv48x4 does not allow pa 50..64!=0"
        );
        GPAddrSv48X4(addr)
    }
    fn as_u64(&self) -> u64 {
        self.0
    }
}
