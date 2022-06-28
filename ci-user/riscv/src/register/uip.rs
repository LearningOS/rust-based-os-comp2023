//! uip register

use bit_field::BitField;

/// uip register
#[derive(Clone, Copy, Debug)]
pub struct Uip {
    bits: usize,
}

impl Uip {
    /// Returns the contents of the register as raw bits
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    /// User Software Interrupt Pending
    #[inline]
    pub fn usoft(&self) -> bool {
        self.bits.get_bit(0)
    }

    /// User Timer Interrupt Pending
    #[inline]
    pub fn utimer(&self) -> bool {
        self.bits.get_bit(4)
    }

    /// User External Interrupt Pending
    #[inline]
    pub fn uext(&self) -> bool {
        self.bits.get_bit(8)
    }
}

read_csr_as!(Uip, 0x044, __read_uip);
