//! sip register

use bit_field::BitField;

/// sip register
#[derive(Clone, Copy, Debug)]
pub struct Sip {
    bits: usize,
}

impl Sip {
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
}

read_csr_as!(Sip, 0x144, __read_sip);
