//! Virtual Supevisor Interrupt Pending Register.

use bit_field::BitField;

#[derive(Copy, Clone, Debug)]
pub struct Vsip {
    bits: usize,
}
impl Vsip {
    #[inline]
    pub fn bits(&self) -> usize {
        return self.bits;
    }
    #[inline]
    pub fn from_bits(x: usize) -> Self {
        return Vsip { bits: x };
    }
    #[inline]
    pub unsafe fn write(&self) {
        _write(self.bits);
    }
    /// Software Interrupt
    #[inline]
    pub fn ssip(&self) -> bool {
        self.bits.get_bit(1)
    }
    #[inline]
    pub fn set_ssip(&mut self, val: bool) {
        self.bits.set_bit(1, val);
    }
    /// Timer Interrupt
    #[inline]
    pub fn stip(&self) -> bool {
        self.bits.get_bit(5)
    }
    #[inline]
    pub fn set_stip(&mut self, val: bool) {
        self.bits.set_bit(5, val);
    }
    /// External Interrupt
    #[inline]
    pub fn seip(&self) -> bool {
        self.bits.get_bit(9)
    }
    #[inline]
    pub fn set_seip(&mut self, val: bool) {
        self.bits.set_bit(9, val);
    }
}
read_csr_as!(Vsip, 580, __read_vsip);
write_csr!(580, __write_vsip);
set!(580, __set_vsip);
clear!(580, __clear_vsip);
// bit ops
set_clear_csr!(
    ///Software Interrupt
    , set_ssip, clear_ssip, 1 << 1);
set_clear_csr!(
    ///Timer Interrupt
    , set_stip, clear_stip, 1 << 5);
set_clear_csr!(
    ///External Interrupt 
    , set_seip, clear_seip, 1 << 9);

// enums
