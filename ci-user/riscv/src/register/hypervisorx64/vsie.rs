//! Virtual Supevisor Interrupt Enable Register.

use bit_field::BitField;

#[derive(Copy, Clone, Debug)]
pub struct Vsie {
    bits: usize,
}
impl Vsie {
    #[inline]
    pub fn bits(&self) -> usize {
        return self.bits;
    }
    #[inline]
    pub fn from_bits(x: usize) -> Self {
        return Vsie { bits: x };
    }
    #[inline]
    pub unsafe fn write(&self) {
        _write(self.bits);
    }
    /// Software Interrupt
    #[inline]
    pub fn ssie(&self) -> bool {
        self.bits.get_bit(1)
    }
    #[inline]
    pub fn set_ssie(&mut self, val: bool) {
        self.bits.set_bit(1, val);
    }
    /// Timer Interrupt
    #[inline]
    pub fn stie(&self) -> bool {
        self.bits.get_bit(5)
    }
    #[inline]
    pub fn set_stie(&mut self, val: bool) {
        self.bits.set_bit(5, val);
    }
    /// External Interrupt
    #[inline]
    pub fn seie(&self) -> bool {
        self.bits.get_bit(9)
    }
    #[inline]
    pub fn set_seie(&mut self, val: bool) {
        self.bits.set_bit(9, val);
    }
}
read_csr_as!(Vsie, 516, __read_vsie);
write_csr!(516, __write_vsie);
set!(516, __set_vsie);
clear!(516, __clear_vsie);
// bit ops
set_clear_csr!(
    ///Software Interrupt
    , set_ssie, clear_ssie, 1 << 1);
set_clear_csr!(
    ///Timer Interrupt
    , set_stie, clear_stie, 1 << 5);
set_clear_csr!(
    ///External Interrupt 
    , set_seie, clear_seie, 1 << 9);

// enums
