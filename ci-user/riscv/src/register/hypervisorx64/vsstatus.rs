//! Hypervisor Guest External Interrupt Pending Register.

use bit_field::BitField;

#[derive(Copy, Clone, Debug)]
pub struct Vsstatus {
    bits: usize,
}
impl Vsstatus {
    #[inline]
    pub fn bits(&self) -> usize {
        return self.bits;
    }
    #[inline]
    pub fn from_bits(x: usize) -> Self {
        return Vsstatus { bits: x };
    }
    #[inline]
    pub unsafe fn write(&self) {
        _write(self.bits);
    }
    ///
    #[inline]
    pub fn sd(&self) -> usize {
        self.bits.get_bits(60..64)
    }
    #[inline]
    pub fn set_sd(&mut self, val: usize) {
        self.bits.set_bits(60..64, val);
    }
    /// Effective User XLEN.
    #[inline]
    pub fn uxl(&self) -> UxlValues {
        UxlValues::from(self.bits.get_bits(32..34))
    }
    #[inline]
    pub fn set_uxl(&mut self, val: UxlValues) {
        self.bits.set_bits(32..34, val as usize);
    }
    ///
    #[inline]
    pub fn mxr(&self) -> bool {
        self.bits.get_bit(19)
    }
    #[inline]
    pub fn set_mxr(&mut self, val: bool) {
        self.bits.set_bit(19, val);
    }
    ///
    #[inline]
    pub fn sum(&self) -> bool {
        self.bits.get_bit(18)
    }
    #[inline]
    pub fn set_sum(&mut self, val: bool) {
        self.bits.set_bit(18, val);
    }
    ///
    #[inline]
    pub fn xs(&self) -> usize {
        self.bits.get_bits(15..17)
    }
    #[inline]
    pub fn set_xs(&mut self, val: usize) {
        self.bits.set_bits(15..17, val);
    }
    ///
    #[inline]
    pub fn fs(&self) -> usize {
        self.bits.get_bits(13..15)
    }
    #[inline]
    pub fn set_fs(&mut self, val: usize) {
        self.bits.set_bits(13..15, val);
    }
    ///
    #[inline]
    pub fn spp(&self) -> bool {
        self.bits.get_bit(8)
    }
    #[inline]
    pub fn set_spp(&mut self, val: bool) {
        self.bits.set_bit(8, val);
    }
    ///
    #[inline]
    pub fn ube(&self) -> bool {
        self.bits.get_bit(6)
    }
    #[inline]
    pub fn set_ube(&mut self, val: bool) {
        self.bits.set_bit(6, val);
    }
    ///
    #[inline]
    pub fn spie(&self) -> bool {
        self.bits.get_bit(5)
    }
    #[inline]
    pub fn set_spie(&mut self, val: bool) {
        self.bits.set_bit(5, val);
    }
    ///
    #[inline]
    pub fn sie(&self) -> bool {
        self.bits.get_bit(1)
    }
    #[inline]
    pub fn set_sie(&mut self, val: bool) {
        self.bits.set_bit(1, val);
    }
}
read_csr_as!(Vsstatus, 512, __read_vsstatus);
write_csr!(512, __write_vsstatus);
set!(512, __set_vsstatus);
clear!(512, __clear_vsstatus);
// bit ops
set_clear_csr!(
    ///
    , set_mxr, clear_mxr, 1 << 19);
set_clear_csr!(
    ///
    , set_sum, clear_sum, 1 << 18);
set_clear_csr!(
    ///
    , set_spp, clear_spp, 1 << 8);
set_clear_csr!(
    ///
    , set_ube, clear_ube, 1 << 6);
set_clear_csr!(
    ///
    , set_spie, clear_spie, 1 << 5);
set_clear_csr!(
    ///
    , set_sie, clear_sie, 1 << 1);

// enums
#[derive(Copy, Clone, Debug)]
#[repr(usize)]
pub enum UxlValues {
    Uxl32 = 1,
    Uxl64 = 2,
    Uxl128 = 3,
}
impl UxlValues {
    fn from(x: usize) -> Self {
        match x {
            1 => Self::Uxl32,
            2 => Self::Uxl64,
            3 => Self::Uxl128,
            _ => unreachable!(),
        }
    }
}
