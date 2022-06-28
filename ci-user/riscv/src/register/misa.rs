//! misa register

use core::num::NonZeroUsize;

/// misa register
#[derive(Clone, Copy, Debug)]
pub struct Misa {
    bits: NonZeroUsize,
}

/// Machine XLEN
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MXL {
    XLEN32,
    XLEN64,
    XLEN128,
}

impl Misa {
    /// Returns the contents of the register as raw bits
    pub fn bits(&self) -> usize {
        self.bits.get()
    }

    /// Returns the machine xlen.
    pub fn mxl(&self) -> MXL {
        let value = match () {
            #[cfg(target_pointer_width = "32")]
            () => (self.bits() >> 30) as u8,
            #[cfg(target_pointer_width = "64")]
            () => (self.bits() >> 62) as u8,
        };
        match value {
            1 => MXL::XLEN32,
            2 => MXL::XLEN64,
            3 => MXL::XLEN128,
            _ => unreachable!(),
        }
    }

    /// Returns true when the atomic extension is implemented.
    pub fn has_extension(&self, extension: char) -> bool {
        let bit = extension as u8 - 65;
        if bit > 25 {
            return false;
        }
        self.bits() & (1 << bit) == (1 << bit)
    }
}

read_csr!(0x301, __read_misa);

/// Reads the CSR
#[inline]
pub fn read() -> Option<Misa> {
    let r = unsafe { _read() };
    // When misa is hardwired to zero it means that the misa csr
    // isn't implemented.
    NonZeroUsize::new(r).map(|bits| Misa { bits })
}
