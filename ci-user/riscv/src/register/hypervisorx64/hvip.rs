//! Hypervisor Virtual Interrupt Pending Register.

use bit_field::BitField;

#[derive(Copy, Clone, Debug)]
pub struct Hvip {
    bits: usize,
}
impl Hvip {
    #[inline]
    pub fn bits(&self) -> usize {
        return self.bits;
    }
    #[inline]
    pub fn from_bits(x: usize) -> Self {
        return Hvip { bits: x };
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
}
read_csr_as!(Hvip, 1605, __read_hvip);
write_csr!(1605, __write_hvip);
set!(1605, __set_hvip);
clear!(1605, __clear_hvip);
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

// enums
