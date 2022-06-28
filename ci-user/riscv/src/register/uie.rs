//! uie register

use bit_field::BitField;

/// uie register
#[derive(Clone, Copy, Debug)]
pub struct Uie {
    bits: usize,
}

impl Uie {
    /// Returns the contents of the register as raw bits
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    /// User Software Interrupt Enable
    #[inline]
    pub fn usoft(&self) -> bool {
        self.bits.get_bit(0)
    }

    /// User Timer Interrupt Enable
    #[inline]
    pub fn utimer(&self) -> bool {
        self.bits.get_bit(4)
    }

    /// User External Interrupt Enable
    #[inline]
    pub fn uext(&self) -> bool {
        self.bits.get_bit(8)
    }
}

read_csr_as!(Uie, 0x004, __read_uie);
set!(0x004, __set_uie);
clear!(0x004, __clear_uie);

set_clear_csr!(
    /// User Software Interrupt Enable
    , set_usoft, clear_usoft, 1 << 0);
set_clear_csr!(
    /// User Timer Interrupt Enable
    , set_utimer, clear_utimer, 1 << 4);
set_clear_csr!(
    /// User External Interrupt Enable
    , set_uext, clear_uext, 1 << 8);
