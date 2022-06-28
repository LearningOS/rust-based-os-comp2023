//! HStatus Register.

use bit_field::BitField;

#[derive(Copy, Clone, Debug)]
pub struct Hstatus {
    bits: usize,
}
impl Hstatus {
    #[inline]
    pub fn bits(&self) -> usize {
        return self.bits;
    }
    #[inline]
    pub fn from_bits(x: usize) -> Self {
        return Hstatus { bits: x };
    }
    #[inline]
    pub unsafe fn write(&self) {
        _write(self.bits);
    }
    /// Effective XLEN for VM.
    #[inline]
    pub fn vsxl(&self) -> VsxlValues {
        VsxlValues::from(self.bits.get_bits(32..34))
    }
    #[inline]
    pub fn set_vsxl(&mut self, val: VsxlValues) {
        self.bits.set_bits(32..34, val as usize);
    }
    /// TSR for VM.
    #[inline]
    pub fn vtsr(&self) -> bool {
        self.bits.get_bit(22)
    }
    #[inline]
    pub fn set_vtsr(&mut self, val: bool) {
        self.bits.set_bit(22, val);
    }
    /// TW for VM.
    #[inline]
    pub fn vtw(&self) -> bool {
        self.bits.get_bit(21)
    }
    #[inline]
    pub fn set_vtw(&mut self, val: bool) {
        self.bits.set_bit(21, val);
    }
    /// TVM for VM.
    #[inline]
    pub fn vtvm(&self) -> bool {
        self.bits.get_bit(20)
    }
    #[inline]
    pub fn set_vtvm(&mut self, val: bool) {
        self.bits.set_bit(20, val);
    }
    /// Virtual Guest External Interrupt Number.
    #[inline]
    pub fn vgein(&self) -> usize {
        self.bits.get_bits(12..18)
    }
    #[inline]
    pub fn set_vgein(&mut self, val: usize) {
        self.bits.set_bits(12..18, val);
    }
    /// Hypervisor User mode.
    #[inline]
    pub fn hu(&self) -> bool {
        self.bits.get_bit(9)
    }
    #[inline]
    pub fn set_hu(&mut self, val: bool) {
        self.bits.set_bit(9, val);
    }
    /// Supervisor Previous Virtual Privilege.
    #[inline]
    pub fn spvp(&self) -> bool {
        self.bits.get_bit(8)
    }
    #[inline]
    pub fn set_spvp(&mut self, val: bool) {
        self.bits.set_bit(8, val);
    }
    /// Supervisor Previous Virtualization mode.
    #[inline]
    pub fn spv(&self) -> bool {
        self.bits.get_bit(7)
    }
    #[inline]
    pub fn set_spv(&mut self, val: bool) {
        self.bits.set_bit(7, val);
    }
    /// Guest Virtual Address.
    #[inline]
    pub fn gva(&self) -> bool {
        self.bits.get_bit(6)
    }
    #[inline]
    pub fn set_gva(&mut self, val: bool) {
        self.bits.set_bit(6, val);
    }
    /// VS access endianness.
    #[inline]
    pub fn vsbe(&self) -> bool {
        self.bits.get_bit(5)
    }
    #[inline]
    pub fn set_vsbe(&mut self, val: bool) {
        self.bits.set_bit(5, val);
    }
}
read_csr_as!(Hstatus, 1536, __read_hstatus);
write_csr!(1536, __write_hstatus);
set!(1536, __set_hstatus);
clear!(1536, __clear_hstatus);
// bit ops
set_clear_csr!(
    ///TSR for VM.
    , set_vtsr, clear_vtsr, 1 << 22);
set_clear_csr!(
    ///TW for VM.
    , set_vtw, clear_vtw, 1 << 21);
set_clear_csr!(
    ///TVM for VM.
    , set_vtvm, clear_vtvm, 1 << 20);
set_clear_csr!(
    ///Hypervisor User mode.
    , set_hu, clear_hu, 1 << 9);
set_clear_csr!(
    ///Supervisor Previous Virtual Privilege.
    , set_spvp, clear_spvp, 1 << 8);
set_clear_csr!(
    ///Supervisor Previous Virtualization mode.
    , set_spv, clear_spv, 1 << 7);
set_clear_csr!(
    ///Guest Virtual Address.
    , set_gva, clear_gva, 1 << 6);
set_clear_csr!(
    ///VS access endianness.
    , set_vsbe, clear_vsbe, 1 << 5);

// enums
#[derive(Copy, Clone, Debug)]
#[repr(usize)]
pub enum VsxlValues {
    Vsxl32 = 1,
    Vsxl64 = 2,
    Vsxl128 = 3,
}
impl VsxlValues {
    fn from(x: usize) -> Self {
        match x {
            1 => Self::Vsxl32,
            2 => Self::Vsxl64,
            3 => Self::Vsxl128,
            _ => unreachable!(),
        }
    }
}
