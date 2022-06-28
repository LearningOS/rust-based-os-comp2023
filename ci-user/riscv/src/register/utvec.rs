//! stvec register

pub use crate::register::mtvec::TrapMode;

/// stvec register
#[derive(Clone, Copy, Debug)]
pub struct Utvec {
    bits: usize,
}

impl Utvec {
    /// Returns the contents of the register as raw bits
    pub fn bits(&self) -> usize {
        self.bits
    }

    /// Returns the trap-vector base-address
    pub fn address(&self) -> usize {
        self.bits - (self.bits & 0b11)
    }

    /// Returns the trap-vector mode
    pub fn trap_mode(&self) -> Option<TrapMode> {
        let mode = self.bits & 0b11;
        match mode {
            0 => Some(TrapMode::Direct),
            1 => Some(TrapMode::Vectored),
            _ => None,
        }
    }
}

read_csr_as!(Utvec, 0x005, __read_utvec);
write_csr!(0x005, __write_utvec);

/// Writes the CSR
#[inline]
pub unsafe fn write(addr: usize, mode: TrapMode) {
    _write(addr + mode as usize);
}
