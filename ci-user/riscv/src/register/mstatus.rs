//! mstatus register
// TODO: Virtualization, Memory Privilege and Extension Context Fields

use bit_field::BitField;
use core::mem::size_of;

/// mstatus register
#[derive(Clone, Copy, Debug)]
pub struct Mstatus {
    bits: usize,
}

/// Additional extension state
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum XS {
    /// All off
    AllOff = 0,

    /// None dirty or clean, some on
    NoneDirtyOrClean = 1,

    /// None dirty, some clean
    NoneDirtySomeClean = 2,

    /// Some dirty
    SomeDirty = 3,
}

/// Floating-point extension state
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FS {
    Off = 0,
    Initial = 1,
    Clean = 2,
    Dirty = 3,
}

/// Machine Previous Privilege Mode
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MPP {
    Machine = 3,
    Supervisor = 1,
    User = 0,
}

/// Supervisor Previous Privilege Mode
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SPP {
    Supervisor = 1,
    User = 0,
}

impl Mstatus {
    /// User Interrupt Enable
    #[inline]
    pub fn uie(&self) -> bool {
        self.bits.get_bit(0)
    }

    /// Supervisor Interrupt Enable
    #[inline]
    pub fn sie(&self) -> bool {
        self.bits.get_bit(1)
    }

    /// Machine Interrupt Enable
    #[inline]
    pub fn mie(&self) -> bool {
        self.bits.get_bit(3)
    }

    /// User Previous Interrupt Enable
    #[inline]
    pub fn upie(&self) -> bool {
        self.bits.get_bit(4)
    }

    /// Supervisor Previous Interrupt Enable
    #[inline]
    pub fn spie(&self) -> bool {
        self.bits.get_bit(5)
    }

    /// Machine Previous Interrupt Enable
    #[inline]
    pub fn mpie(&self) -> bool {
        self.bits.get_bit(7)
    }

    /// Supervisor Previous Privilege Mode
    #[inline]
    pub fn spp(&self) -> SPP {
        match self.bits.get_bit(8) {
            true => SPP::Supervisor,
            false => SPP::User,
        }
    }

    /// Machine Previous Privilege Mode
    #[inline]
    pub fn mpp(&self) -> MPP {
        match self.bits.get_bits(11..13) {
            0b00 => MPP::User,
            0b01 => MPP::Supervisor,
            0b11 => MPP::Machine,
            _ => unreachable!(),
        }
    }

    #[inline]
    pub fn set_mpie(&mut self, val: bool) {
        self.bits.set_bit(7, val);
    }

    #[inline]
    pub fn set_mie(&mut self, val: bool) {
        self.bits.set_bit(3, val);
    }

    #[inline]
    pub fn set_mpp(&mut self, val: MPP) {
        self.bits.set_bits(11..13, val as usize);
    }

    /// Floating-point extension state
    ///
    /// Encodes the status of the floating-point unit,
    /// including the CSR `fcsr` and floating-point data registers `f0–f31`.
    #[inline]
    pub fn fs(&self) -> FS {
        match self.bits.get_bits(13..15) {
            0b00 => FS::Off,
            0b01 => FS::Initial,
            0b10 => FS::Clean,
            0b11 => FS::Dirty,
            _ => unreachable!(),
        }
    }

    /// Additional extension state
    ///
    /// Encodes the status of additional user-mode extensions and associated state.
    #[inline]
    pub fn xs(&self) -> XS {
        match self.bits.get_bits(15..17) {
            0b00 => XS::AllOff,
            0b01 => XS::NoneDirtyOrClean,
            0b10 => XS::NoneDirtySomeClean,
            0b11 => XS::SomeDirty,
            _ => unreachable!(),
        }
    }

    /// Whether either the FS field or XS field
    /// signals the presence of some dirty state
    #[inline]
    pub fn sd(&self) -> bool {
        self.bits.get_bit(size_of::<usize>() * 8 - 1)
    }
}

read_csr_as!(Mstatus, 0x300, __read_mstatus);
write_csr!(0x300, __write_mstatus);
set!(0x300, __set_mstatus);
clear!(0x300, __clear_mstatus);

set_clear_csr!(
    /// User Interrupt Enable
    , set_uie, clear_uie, 1 << 0);

set_clear_csr!(
    /// Supervisor Interrupt Enable
    , set_sie, clear_sie, 1 << 1);

set_clear_csr!(
    /// Machine Interrupt Enable
    , set_mie, clear_mie, 1 << 3);

set_csr!(
    /// User Previous Interrupt Enable
    , set_upie, 1 << 4);

set_csr!(
    /// Supervisor Previous Interrupt Enable
    , set_spie, 1 << 5);

set_csr!(
    /// Machine Previous Interrupt Enable
    , set_mpie, 1 << 7);

/// Supervisor Previous Privilege Mode
#[inline]
pub unsafe fn set_spp(spp: SPP) {
    match spp {
        SPP::Supervisor => _set(1 << 8),
        SPP::User => _clear(1 << 8),
    }
}

/// Machine Previous Privilege Mode
#[inline]
pub unsafe fn set_mpp(mpp: MPP) {
    let mut value = _read();
    value.set_bits(11..13, mpp as usize);
    _write(value);
}

/// Floating-point extension state
#[inline]
pub unsafe fn set_fs(fs: FS) {
    let mut value = _read();
    value.set_bits(13..15, fs as usize);
    _write(value);
}
