//! mideleg register

use bit_field::BitField;

/// mideleg register
#[derive(Clone, Copy, Debug)]
pub struct Mideleg {
    bits: usize,
}

impl Mideleg {
    /// Returns the contents of the register as raw bits
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    /// User Software Interrupt Delegate
    #[inline]
    pub fn usoft(&self) -> bool {
        self.bits.get_bit(0)
    }

    /// Supervisor Software Interrupt Delegate
    #[inline]
    pub fn ssoft(&self) -> bool {
        self.bits.get_bit(1)
    }

    /// User Timer Interrupt Delegate
    #[inline]
    pub fn utimer(&self) -> bool {
        self.bits.get_bit(4)
    }

    /// Supervisor Timer Interrupt Delegate
    #[inline]
    pub fn stimer(&self) -> bool {
        self.bits.get_bit(5)
    }

    /// User External Interrupt Delegate
    #[inline]
    pub fn uext(&self) -> bool {
        self.bits.get_bit(8)
    }

    /// Supervisor External Interrupt Delegate
    #[inline]
    pub fn sext(&self) -> bool {
        self.bits.get_bit(9)
    }
}

read_csr_as!(Mideleg, 0x303, __read_mideleg);
set!(0x303, __set_mideleg);
clear!(0x303, __clear_mideleg);

set_clear_csr!(
    /// User Software Interrupt Delegate
    , set_usoft, clear_usoft, 1 << 0);
set_clear_csr!(
    /// Supervisor Software Interrupt Delegate
    , set_ssoft, clear_ssoft, 1 << 1);
set_clear_csr!(
    /// User Timer Interrupt Delegate
    , set_utimer, clear_utimer, 1 << 4);
set_clear_csr!(
    /// Supervisor Timer Interrupt Delegate
    , set_stimer, clear_stimer, 1 << 5);
set_clear_csr!(
    /// User External Interrupt Delegate
    , set_uext, clear_uext, 1 << 8);
set_clear_csr!(
    /// Supervisor External Interrupt Delegate
    , set_sext, clear_sext, 1 << 9);
