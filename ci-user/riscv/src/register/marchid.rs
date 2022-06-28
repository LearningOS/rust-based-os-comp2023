//! marchid register

use core::num::NonZeroUsize;

/// marchid register
#[derive(Clone, Copy, Debug)]
pub struct Marchid {
    bits: NonZeroUsize,
}

impl Marchid {
    /// Returns the contents of the register as raw bits
    pub fn bits(&self) -> usize {
        self.bits.get()
    }
}

read_csr!(0xF11, __read_marchid);

/// Reads the CSR
#[inline]
pub fn read() -> Option<Marchid> {
    let r = unsafe { _read() };
    // When marchid is hardwired to zero it means that the marchid
    // csr isn't implemented.
    NonZeroUsize::new(r).map(|bits| Marchid { bits })
}
