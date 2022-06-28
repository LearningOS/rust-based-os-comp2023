use super::*;
use bit_field::BitField;
use core::convert::TryInto;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VirtAddrSv48(u64);

impl VirtualAddress for VirtAddrSv48 {
    unsafe fn as_mut<'a, 'b, T>(&'a self) -> &'b mut T {
        &mut *(self.0 as *mut T)
    }
}
impl Address for VirtAddrSv48 {
    fn new(addr: usize) -> Self {
        Self::new_u64(addr as u64)
    }
    fn as_usize(&self) -> usize {
        self.0.try_into().unwrap()
    }
    fn page_number(&self) -> usize {
        self.0.get_bits(12..48).try_into().unwrap()
    }
    fn page_offset(&self) -> usize {
        self.0.get_bits(0..12) as usize
    }
    fn to_4k_aligned(&self) -> Self {
        VirtAddrSv48((self.0 >> 12) << 12)
    }
}

impl AddressL4 for VirtAddrSv48 {
    fn p4_index(&self) -> usize {
        self.0.get_bits(39..48) as usize
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
        assert!(p4_index.get_bits(9..) == 0, "p4_index exceeding 9 bits");
        assert!(p3_index.get_bits(9..) == 0, "p3_index exceeding 9 bits");
        assert!(p2_index.get_bits(9..) == 0, "p2_index exceeding 9 bits");
        assert!(p1_index.get_bits(9..) == 0, "p1_index exceeding 9 bits");
        assert!(offset.get_bits(12..) == 0, "offset exceeding 12 bits");
        let mut addr = (p4_index << 12 << 9 << 9 << 9)
            | (p3_index << 12 << 9 << 9)
            | (p2_index << 12 << 9)
            | (p1_index << 12)
            | offset;
        if addr.get_bit(47) {
            addr.set_bits(48..64, (1 << (64 - 48)) - 1);
        } else {
            addr.set_bits(48..64, 0x0000);
        }
        VirtAddrSv48::new_u64(addr)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PhysAddrSv48(u64);
impl Address for PhysAddrSv48 {
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
        PhysAddrSv48((self.0 >> 12) << 12)
    }
}

impl AddressX64 for VirtAddrSv48 {
    fn new_u64(addr: u64) -> Self {
        if addr.get_bit(47) {
            assert!(
                addr.get_bits(48..64) == (1 << (64 - 48)) - 1,
                "va 48..64 is not sext"
            );
        } else {
            assert!(addr.get_bits(48..64) == 0x0000, "va 48..64 is not sext");
        }
        VirtAddrSv48(addr as u64)
    }
    fn as_u64(&self) -> u64 {
        self.0
    }
}
impl AddressX64 for PhysAddrSv48 {
    fn new_u64(addr: u64) -> Self {
        assert!(
            addr.get_bits(56..64) == 0,
            "Sv48 does not allow pa 56..64!=0"
        );
        PhysAddrSv48(addr)
    }
    fn as_u64(&self) -> u64 {
        self.0
    }
}

impl PhysicalAddress for PhysAddrSv48 {}
