//! Virtual Supervisor Guest Address Translation and Protection Register.

use bit_field::BitField;

#[derive(Copy, Clone, Debug)]
pub struct Vsatp {
    bits: usize,
}
impl Vsatp {
    #[inline]
    pub fn bits(&self) -> usize {
        return self.bits;
    }
    #[inline]
    pub fn from_bits(x: usize) -> Self {
        return Vsatp { bits: x };
    }
    #[inline]
    pub unsafe fn write(&self) {
        _write(self.bits);
    }
    /// Guest address translation mode.
    #[inline]
    pub fn mode(&self) -> HgatpValues {
        HgatpValues::from(self.bits.get_bits(60..64))
    }
    #[inline]
    pub fn set_mode(&mut self, val: HgatpValues) {
        self.bits.set_bits(60..64, val as usize);
    }
    /// ASID.
    #[inline]
    pub fn asid(&self) -> usize {
        self.bits.get_bits(44..60)
    }
    #[inline]
    pub fn set_asid(&mut self, val: usize) {
        self.bits.set_bits(44..60, val);
    }
    /// Physical Page Number for root page table.
    #[inline]
    pub fn ppn(&self) -> usize {
        self.bits.get_bits(0..44)
    }
    #[inline]
    pub fn set_ppn(&mut self, val: usize) {
        self.bits.set_bits(0..44, val);
    }
}
read_csr_as!(Vsatp, 640, __read_vsatp);
write_csr!(640, __write_vsatp);
set!(640, __set_vsatp);
clear!(640, __clear_vsatp);
// bit ops

// enums
#[derive(Copy, Clone, Debug)]
#[repr(usize)]
pub enum HgatpValues {
    Bare = 0,
    Sv39x4 = 8,
    Sv48x4 = 9,
}
impl HgatpValues {
    fn from(x: usize) -> Self {
        match x {
            0 => Self::Bare,
            8 => Self::Sv39x4,
            9 => Self::Sv48x4,
            _ => unreachable!(),
        }
    }
}
