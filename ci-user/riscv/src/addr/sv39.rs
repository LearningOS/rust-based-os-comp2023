use super::*;
use bit_field::BitField;
use core::convert::TryInto;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VirtAddrSv39(u64);

impl VirtualAddress for VirtAddrSv39 {
    unsafe fn as_mut<'a, 'b, T>(&'a self) -> &'b mut T {
        &mut *(self.0 as *mut T)
    }
}
impl Address for VirtAddrSv39 {
    fn new(addr: usize) -> Self {
        Self::new_u64(addr as u64)
    }
    fn as_usize(&self) -> usize {
        self.0.try_into().unwrap()
    }
    fn page_number(&self) -> usize {
        self.0.get_bits(12..39).try_into().unwrap()
    }
    fn page_offset(&self) -> usize {
        self.0.get_bits(0..12) as usize
    }
    fn to_4k_aligned(&self) -> Self {
        VirtAddrSv39((self.0 >> 12) << 12)
    }
}

impl AddressL3 for VirtAddrSv39 {
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
        let mut addr =
            (p3_index << 12 << 9 << 9) | (p2_index << 12 << 9) | (p1_index << 12) | offset;
        if addr.get_bit(38) {
            addr.set_bits(39..64, (1 << (64 - 39)) - 1);
        } else {
            addr.set_bits(39..64, 0x0000);
        }
        VirtAddrSv39::new_u64(addr)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PhysAddrSv39(u64);
impl Address for PhysAddrSv39 {
    fn new(addr: usize) -> Self {
        Self::new_u64(addr as u64)
    }
    fn as_usize(&self) -> usize {
        self.0.try_into().unwrap()
    }
    fn page_number(&self) -> usize {
        self.0.get_bits(12..56) as usize
    }
    fn page_offset(&self) -> usize {
        self.0.get_bits(0..12) as usize
    }
    fn to_4k_aligned(&self) -> Self {
        PhysAddrSv39((self.0 >> 12) << 12)
    }
}

impl AddressX64 for VirtAddrSv39 {
    fn new_u64(addr: u64) -> Self {
        if addr.get_bit(38) {
            assert!(
                addr.get_bits(39..64) == (1 << (64 - 39)) - 1,
                "va 39..64 is not sext"
            );
        } else {
            assert!(addr.get_bits(39..64) == 0x0000, "va 39..64 is not sext");
        }
        VirtAddrSv39(addr as u64)
    }
    fn as_u64(&self) -> u64 {
        self.0
    }
}
impl AddressX64 for PhysAddrSv39 {
    fn new_u64(addr: u64) -> Self {
        assert!(
            addr.get_bits(56..64) == 0,
            "Sv39 does not allow pa 56..64!=0"
        );
        PhysAddrSv39(addr)
    }
    fn as_u64(&self) -> u64 {
        self.0
    }
}

impl PhysicalAddress for PhysAddrSv39 {}
