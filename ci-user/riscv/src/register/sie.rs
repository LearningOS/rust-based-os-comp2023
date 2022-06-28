//! sie register

use bit_field::BitField;

/// sie register
#[derive(Clone, Copy, Debug)]
pub struct Sie {
    bits: usize,
}

impl Sie {
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

    /// Supervisor Software Interrupt Enable
    #[inline]
    pub fn ssoft(&self) -> bool {
        self.bits.get_bit(1)
    }

    /// User Timer Interrupt Enable
    #[inline]
    pub fn utimer(&self) -> bool {
        self.bits.get_bit(4)
    }

    /// Supervisor Timer Interrupt Enable
    #[inline]
    pub fn stimer(&self) -> bool {
        self.bits.get_bit(5)
    }

    /// User External Interrupt Enable
    #[inline]
    pub fn uext(&self) -> bool {
        self.bits.get_bit(8)
    }

    /// Supervisor External Interrupt Enable
    #[inline]
    pub fn sext(&self) -> bool {
        self.bits.get_bit(9)
    }
}

read_csr_as!(Sie, 0x104, __read_sie);
set!(0x104, __set_sie);
clear!(0x104, __clear_sie);

set_clear_csr!(
    /// User Software Interrupt Enable
    , set_usoft, clear_usoft, 1 << 0);
set_clear_csr!(
    /// Supervisor Software Interrupt Enable
    , set_ssoft, clear_ssoft, 1 << 1);
set_clear_csr!(
    /// User Timer Interrupt Enable
    , set_utimer, clear_utimer, 1 << 4);
set_clear_csr!(
    /// Supervisor Timer Interrupt Enable
    , set_stimer, clear_stimer, 1 << 5);
set_clear_csr!(
    /// User External Interrupt Enable
    , set_uext, clear_uext, 1 << 8);
set_clear_csr!(
    /// Supervisor External Interrupt Enable
    , set_sext, clear_sext, 1 << 9);
