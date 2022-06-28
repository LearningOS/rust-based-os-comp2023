//! mip register

use bit_field::BitField;

/// mip register
#[derive(Clone, Copy, Debug)]
pub struct Mip {
    bits: usize,
}

impl Mip {
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

    /// Supervisor Software Interrupt Pending
    #[inline]
    pub fn ssoft(&self) -> bool {
        self.bits.get_bit(1)
    }

    /// Machine Software Interrupt Pending
    #[inline]
    pub fn msoft(&self) -> bool {
        self.bits.get_bit(3)
    }

    /// User Timer Interrupt Pending
    #[inline]
    pub fn utimer(&self) -> bool {
        self.bits.get_bit(4)
    }

    /// Supervisor Timer Interrupt Pending
    #[inline]
    pub fn stimer(&self) -> bool {
        self.bits.get_bit(5)
    }

    /// Machine Timer Interrupt Pending
    #[inline]
    pub fn mtimer(&self) -> bool {
        self.bits.get_bit(7)
    }

    /// User External Interrupt Pending
    #[inline]
    pub fn uext(&self) -> bool {
        self.bits.get_bit(8)
    }

    /// Supervisor External Interrupt Pending
    #[inline]
    pub fn sext(&self) -> bool {
        self.bits.get_bit(9)
    }

    /// Machine External Interrupt Pending
    #[inline]
    pub fn mext(&self) -> bool {
        self.bits.get_bit(11)
    }
}

read_csr_as!(Mip, 0x344, __read_mip);
set!(0x344, __set_mip);
clear!(0x344, __clear_mip);

set_clear_csr!(
    /// User Software Interrupt Pending
    , set_usoft, clear_usoft, 1 << 0);
set_clear_csr!(
    /// Supervisor Software Interrupt Pending
    , set_ssoft, clear_ssoft, 1 << 1);
set_clear_csr!(
    /// Machine Software Interrupt Pending
    , set_msoft, clear_msoft, 1 << 3);
set_clear_csr!(
    /// User Timer Interrupt Pending
    , set_utimer, clear_utimer, 1 << 4);
set_clear_csr!(
    /// Supervisor Timer Interrupt Pending
    , set_stimer, clear_stimer, 1 << 5);
set_clear_csr!(
    /// Machine Timer Interrupt Pending
    , set_mtimer, clear_mtimer, 1 << 7);
set_clear_csr!(
    /// User External Interrupt Pending
    , set_uext, clear_uext, 1 << 8);
set_clear_csr!(
    /// Supervisor External Interrupt Pending
    , set_sext, clear_sext, 1 << 9);
