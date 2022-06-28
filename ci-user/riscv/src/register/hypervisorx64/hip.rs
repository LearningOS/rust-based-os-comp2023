//! Hypervisor Interrupt Pending Register.

use bit_field::BitField;

#[derive(Copy, Clone, Debug)]
pub struct Hip {
    bits: usize,
}
impl Hip {
    #[inline]
    pub fn bits(&self) -> usize {
        return self.bits;
    }
    #[inline]
    pub fn from_bits(x: usize) -> Self {
        return Hip { bits: x };
    }
    #[inline]
    pub unsafe fn write(&self) {
        _write(self.bits);
    }
    /// Software Interrupt
    #[inline]
    pub fn vssip(&self) -> bool {
        self.bits.get_bit(2)
    }
    #[inline]
    pub fn set_vssip(&mut self, val: bool) {
        self.bits.set_bit(2, val);
    }
    /// Timer Interrupt
    #[inline]
    pub fn vstip(&self) -> bool {
        self.bits.get_bit(6)
    }
    #[inline]
    pub fn set_vstip(&mut self, val: bool) {
        self.bits.set_bit(6, val);
    }
    /// External Interrupt
    #[inline]
    pub fn vseip(&self) -> bool {
        self.bits.get_bit(10)
    }
    #[inline]
    pub fn set_vseip(&mut self, val: bool) {
        self.bits.set_bit(10, val);
    }
    /// Guest External Interrupt
    #[inline]
    pub fn sgeip(&self) -> bool {
        self.bits.get_bit(12)
    }
    #[inline]
    pub fn set_sgeip(&mut self, val: bool) {
        self.bits.set_bit(12, val);
    }
}
read_csr_as!(Hip, 1604, __read_hip);
write_csr!(1604, __write_hip);
set!(1604, __set_hip);
clear!(1604, __clear_hip);
// bit ops
set_clear_csr!(
    ///Software Interrupt
    , set_vssip, clear_vssip, 1 << 2);
set_clear_csr!(
    ///Timer Interrupt
    , set_vstip, clear_vstip, 1 << 6);
set_clear_csr!(
    ///External Interrupt 
    , set_vseip, clear_vseip, 1 << 10);
set_clear_csr!(
    ///Guest External Interrupt 
    , set_sgeip, clear_sgeip, 1 << 12);

// enums
