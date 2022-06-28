//! Hypervisor Interrupt Enable Register.

use bit_field::BitField;

#[derive(Copy, Clone, Debug)]
pub struct Hie {
    bits: usize,
}
impl Hie {
    #[inline]
    pub fn bits(&self) -> usize {
        return self.bits;
    }
    #[inline]
    pub fn from_bits(x: usize) -> Self {
        return Hie { bits: x };
    }
    #[inline]
    pub unsafe fn write(&self) {
        _write(self.bits);
    }
    /// Software Interrupt
    #[inline]
    pub fn vssie(&self) -> bool {
        self.bits.get_bit(2)
    }
    #[inline]
    pub fn set_vssie(&mut self, val: bool) {
        self.bits.set_bit(2, val);
    }
    /// Timer Interrupt
    #[inline]
    pub fn vstie(&self) -> bool {
        self.bits.get_bit(6)
    }
    #[inline]
    pub fn set_vstie(&mut self, val: bool) {
        self.bits.set_bit(6, val);
    }
    /// External Interrupt
    #[inline]
    pub fn vseie(&self) -> bool {
        self.bits.get_bit(10)
    }
    #[inline]
    pub fn set_vseie(&mut self, val: bool) {
        self.bits.set_bit(10, val);
    }
    /// Guest External Interrupt
    #[inline]
    pub fn sgeie(&self) -> bool {
        self.bits.get_bit(12)
    }
    #[inline]
    pub fn set_sgeie(&mut self, val: bool) {
        self.bits.set_bit(12, val);
    }
}
read_csr_as!(Hie, 1540, __read_hie);
write_csr!(1540, __write_hie);
set!(1540, __set_hie);
clear!(1540, __clear_hie);
// bit ops
set_clear_csr!(
    ///Software Interrupt
    , set_vssie, clear_vssie, 1 << 2);
set_clear_csr!(
    ///Timer Interrupt
    , set_vstie, clear_vstie, 1 << 6);
set_clear_csr!(
    ///External Interrupt 
    , set_vseie, clear_vseie, 1 << 10);
set_clear_csr!(
    ///Guest External Interrupt 
    , set_sgeie, clear_sgeie, 1 << 12);

// enums
