//! Hypervisor Interrupt Delegation Register.

use bit_field::BitField;

#[derive(Copy, Clone, Debug)]
pub struct Hideleg {
    bits: usize,
}
impl Hideleg {
    #[inline]
    pub fn bits(&self) -> usize {
        return self.bits;
    }
    #[inline]
    pub fn from_bits(x: usize) -> Self {
        return Hideleg { bits: x };
    }
    #[inline]
    pub unsafe fn write(&self) {
        _write(self.bits);
    }
    /// Software Interrupt
    #[inline]
    pub fn sip(&self) -> bool {
        self.bits.get_bit(2)
    }
    #[inline]
    pub fn set_sip(&mut self, val: bool) {
        self.bits.set_bit(2, val);
    }
    /// Timer Interrupt
    #[inline]
    pub fn tip(&self) -> bool {
        self.bits.get_bit(6)
    }
    #[inline]
    pub fn set_tip(&mut self, val: bool) {
        self.bits.set_bit(6, val);
    }
    /// External Interrupt
    #[inline]
    pub fn eip(&self) -> bool {
        self.bits.get_bit(10)
    }
    #[inline]
    pub fn set_eip(&mut self, val: bool) {
        self.bits.set_bit(10, val);
    }
}
read_csr_as!(Hideleg, 1539, __read_hideleg);
write_csr!(1539, __write_hideleg);
set!(1539, __set_hideleg);
clear!(1539, __clear_hideleg);
// bit ops
set_clear_csr!(
    ///Software Interrupt
    , set_sip, clear_sip, 1 << 2);
set_clear_csr!(
    ///Timer Interrupt
    , set_tip, clear_tip, 1 << 6);
set_clear_csr!(
    ///External Interrupt 
    , set_eip, clear_eip, 1 << 10);

// enums
