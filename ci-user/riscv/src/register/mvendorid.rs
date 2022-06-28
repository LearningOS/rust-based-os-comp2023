//! mvendorid register

use core::num::NonZeroUsize;

/// mvendorid register
#[derive(Clone, Copy, Debug)]
pub struct Mvendorid {
    bits: NonZeroUsize,
}

impl Mvendorid {
    /// Returns the contents of the register as raw bits
    pub fn bits(&self) -> usize {
        self.bits.get()
    }

    /// Returns the JEDEC manufacturer ID
    pub fn jedec_manufacturer(&self) -> usize {
        self.bits() >> 7
    }
}

read_csr!(0xF11, __read_mvendorid);

/// Reads the CSR
#[inline]
pub fn read() -> Option<Mvendorid> {
    let r = unsafe { _read() };
    // When mvendorid is hardwired to zero it means that the mvendorid
    // csr isn't implemented.
    NonZeroUsize::new(r).map(|bits| Mvendorid { bits })
}
