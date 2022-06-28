//! mimpid register

use core::num::NonZeroUsize;

/// mimpid register
#[derive(Clone, Copy, Debug)]
pub struct Mimpid {
    bits: NonZeroUsize,
}

impl Mimpid {
    /// Returns the contents of the register as raw bits
    pub fn bits(&self) -> usize {
        self.bits.get()
    }
}

read_csr!(0xF11, __read_mimpid);

/// Reads the CSR
#[inline]
pub fn read() -> Option<Mimpid> {
    let r = unsafe { _read() };
    // When mimpid is hardwired to zero it means that the mimpid
    // csr isn't implemented.
    NonZeroUsize::new(r).map(|bits| Mimpid { bits })
}
