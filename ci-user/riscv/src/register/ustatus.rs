//! ustatus register
// TODO: Virtualization, Memory Privilege and Extension Context Fields

use bit_field::BitField;

/// ustatus register
#[derive(Clone, Copy, Debug)]
pub struct Ustatus {
    bits: usize,
}

impl Ustatus {
    /// User Interrupt Enable
    #[inline]
    pub fn uie(&self) -> bool {
        self.bits.get_bit(0)
    }

    /// User Previous Interrupt Enable
    #[inline]
    pub fn upie(&self) -> bool {
        self.bits.get_bit(4)
    }
}

read_csr_as!(Ustatus, 0x000, __read_ustatus);
write_csr!(0x000, __write_ustatus);
set!(0x000, __set_ustatus);
clear!(0x000, __clear_ustatus);

set_clear_csr!(
    /// User Interrupt Enable
    , set_uie, clear_uie, 1 << 0);

set_csr!(
    /// User Previous Interrupt Enable
    , set_upie, 1 << 4);
