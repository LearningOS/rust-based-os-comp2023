//! Virtual Supervisor Cause Register.

use bit_field::BitField;

#[derive(Copy, Clone, Debug)]
pub struct Vscause {
    bits: usize,
}
impl Vscause {
    #[inline]
    pub fn bits(&self) -> usize {
        return self.bits;
    }
    #[inline]
    pub fn from_bits(x: usize) -> Self {
        return Vscause { bits: x };
    }
    #[inline]
    pub unsafe fn write(&self) {
        _write(self.bits);
    }
    /// Is cause interrupt.
    #[inline]
    pub fn interrupt(&self) -> bool {
        self.bits.get_bit(63)
    }
    #[inline]
    pub fn set_interrupt(&mut self, val: bool) {
        self.bits.set_bit(63, val);
    }
    /// Exception code
    #[inline]
    pub fn code(&self) -> usize {
        self.bits.get_bits(0..63)
    }
    #[inline]
    pub fn set_code(&mut self, val: usize) {
        self.bits.set_bits(0..63, val);
    }
}
read_csr_as!(Vscause, 578, __read_vscause);
write_csr!(578, __write_vscause);
set!(578, __set_vscause);
clear!(578, __clear_vscause);
// bit ops
set_clear_csr!(
    ///Is cause interrupt.
    , set_interrupt, clear_interrupt, 1 << 63);

// enums
