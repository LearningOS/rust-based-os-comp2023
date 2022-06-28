//! Virtual Supervisor Trap Vector Base Address Register.

use bit_field::BitField;

#[derive(Copy, Clone, Debug)]
pub struct Vstvec {
    bits: usize,
}
impl Vstvec {
    #[inline]
    pub fn bits(&self) -> usize {
        return self.bits;
    }
    #[inline]
    pub fn from_bits(x: usize) -> Self {
        return Vstvec { bits: x };
    }
    #[inline]
    pub unsafe fn write(&self) {
        _write(self.bits);
    }
    ///
    #[inline]
    pub fn base(&self) -> usize {
        self.bits.get_bits(2..64)
    }
    #[inline]
    pub fn set_base(&mut self, val: usize) {
        self.bits.set_bits(2..64, val);
    }
    ///
    #[inline]
    pub fn mode(&self) -> usize {
        self.bits.get_bits(0..2)
    }
    #[inline]
    pub fn set_mode(&mut self, val: usize) {
        self.bits.set_bits(0..2, val);
    }
}
read_csr_as!(Vstvec, 517, __read_vstvec);
write_csr!(517, __write_vstvec);
set!(517, __set_vstvec);
clear!(517, __clear_vstvec);
// bit ops

// enums
