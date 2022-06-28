//! Floating-point control and status register

use bit_field::BitField;

/// Floating-point control and status register
#[derive(Clone, Copy, Debug)]
pub struct FCSR {
    bits: u32,
}

/// Accrued Exception Flags
#[derive(Clone, Copy, Debug)]
pub struct Flags(u32);

/// Accrued Exception Flag
#[derive(Clone, Copy, Debug)]
pub enum Flag {
    /// Inexact
    NX = 0b00001,

    /// Underflow
    UF = 0b00010,

    /// Overflow
    OF = 0b00100,

    /// Divide by Zero
    DZ = 0b01000,

    /// Invalid Operation
    NV = 0b10000,
}

impl Flags {
    /// Inexact
    #[inline]
    pub fn nx(&self) -> bool {
        self.0.get_bit(0)
    }

    /// Underflow
    #[inline]
    pub fn uf(&self) -> bool {
        self.0.get_bit(1)
    }

    /// Overflow
    #[inline]
    pub fn of(&self) -> bool {
        self.0.get_bit(2)
    }

    /// Divide by Zero
    #[inline]
    pub fn dz(&self) -> bool {
        self.0.get_bit(3)
    }

    /// Invalid Operation
    #[inline]
    pub fn nv(&self) -> bool {
        self.0.get_bit(4)
    }
}

/// Rounding Mode
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RoundingMode {
    RoundToNearestEven = 0b000,
    RoundTowardsZero = 0b001,
    RoundDown = 0b010,
    RoundUp = 0b011,
    RoundToNearestMaxMagnitude = 0b100,
    Invalid = 0b111,
}

impl FCSR {
    /// Returns the contents of the register as raw bits
    pub fn bits(&self) -> u32 {
        self.bits
    }

    /// Accrued Exception Flags
    #[inline]
    pub fn fflags(&self) -> Flags {
        Flags(self.bits.get_bits(0..5))
    }

    /// Rounding Mode
    #[inline]
    pub fn frm(&self) -> RoundingMode {
        match self.bits.get_bits(5..8) {
            0b000 => RoundingMode::RoundToNearestEven,
            0b001 => RoundingMode::RoundTowardsZero,
            0b010 => RoundingMode::RoundDown,
            0b011 => RoundingMode::RoundUp,
            0b100 => RoundingMode::RoundToNearestMaxMagnitude,
            _ => RoundingMode::Invalid,
        }
    }
}

read_csr!(0x003, __read_fcsr);
write_csr!(0x003, __write_fcsr);
clear!(0x003, __clear_fcsr);

/// Reads the CSR
#[inline]
pub fn read() -> FCSR {
    FCSR {
        bits: unsafe { _read() as u32 },
    }
}

/// Writes the CSR
#[inline]
pub unsafe fn set_rounding_mode(frm: RoundingMode) {
    let old = read();
    let bits = ((frm as u32) << 5) | old.fflags().0;
    _write(bits as usize);
}

/// Resets `fflags` field bits
#[inline]
pub unsafe fn clear_flags() {
    let mask = 0b11111;
    _clear(mask);
}

/// Resets `fflags` field bit
#[inline]
pub unsafe fn clear_flag(flag: Flag) {
    _clear(flag as usize);
}
