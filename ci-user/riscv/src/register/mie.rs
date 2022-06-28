//! mie register

use bit_field::BitField;

/// mie register
#[derive(Clone, Copy, Debug)]
pub struct Mie {
    bits: usize,
}

impl Mie {
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

    /// Machine Software Interrupt Enable
    #[inline]
    pub fn msoft(&self) -> bool {
        self.bits.get_bit(3)
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

    /// Machine Timer Interrupt Enable
    #[inline]
    pub fn mtimer(&self) -> bool {
        self.bits.get_bit(7)
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

    /// Machine External Interrupt Enable
    #[inline]
    pub fn mext(&self) -> bool {
        self.bits.get_bit(11)
    }
}

read_csr_as!(Mie, 0x304, __read_mie);
set!(0x304, __set_mie);
clear!(0x304, __clear_mie);

set_clear_csr!(
    /// User Software Interrupt Enable
    , set_usoft, clear_usoft, 1 << 0);
set_clear_csr!(
    /// Supervisor Software Interrupt Enable
    , set_ssoft, clear_ssoft, 1 << 1);
set_clear_csr!(
    /// Machine Software Interrupt Enable
    , set_msoft, clear_msoft, 1 << 3);
set_clear_csr!(
    /// User Timer Interrupt Enable
    , set_utimer, clear_utimer, 1 << 4);
set_clear_csr!(
    /// Supervisor Timer Interrupt Enable
    , set_stimer, clear_stimer, 1 << 5);
set_clear_csr!(
    /// Machine Timer Interrupt Enable
    , set_mtimer, clear_mtimer, 1 << 7);
set_clear_csr!(
    /// User External Interrupt Enable
    , set_uext, clear_uext, 1 << 8);
set_clear_csr!(
    /// Supervisor External Interrupt Enable
    , set_sext, clear_sext, 1 << 9);
set_clear_csr!(
    /// Machine External Interrupt Enable
    , set_mext, clear_mext, 1 << 11);
