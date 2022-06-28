//! Hypervisor Guest External Interrupt Pending Register.

use bit_field::BitField;

#[derive(Copy, Clone, Debug)]
pub struct Hcounteren {
    bits: usize,
}
impl Hcounteren {
    #[inline]
    pub fn bits(&self) -> usize {
        return self.bits;
    }
    #[inline]
    pub fn from_bits(x: usize) -> Self {
        return Hcounteren { bits: x };
    }
    #[inline]
    pub unsafe fn write(&self) {
        _write(self.bits);
    }
    ///
    #[inline]
    pub fn cy(&self) -> bool {
        self.bits.get_bit(0)
    }
    #[inline]
    pub fn set_cy(&mut self, val: bool) {
        self.bits.set_bit(0, val);
    }
    ///
    #[inline]
    pub fn tm(&self) -> bool {
        self.bits.get_bit(1)
    }
    #[inline]
    pub fn set_tm(&mut self, val: bool) {
        self.bits.set_bit(1, val);
    }
    ///
    #[inline]
    pub fn ir(&self) -> bool {
        self.bits.get_bit(2)
    }
    #[inline]
    pub fn set_ir(&mut self, val: bool) {
        self.bits.set_bit(2, val);
    }
    ///
    #[inline]
    pub fn hpm3(&self) -> bool {
        self.bits.get_bit(3)
    }
    #[inline]
    pub fn set_hpm3(&mut self, val: bool) {
        self.bits.set_bit(3, val);
    }
    ///
    #[inline]
    pub fn hpm4(&self) -> bool {
        self.bits.get_bit(4)
    }
    #[inline]
    pub fn set_hpm4(&mut self, val: bool) {
        self.bits.set_bit(4, val);
    }
    ///
    #[inline]
    pub fn hpm5(&self) -> bool {
        self.bits.get_bit(5)
    }
    #[inline]
    pub fn set_hpm5(&mut self, val: bool) {
        self.bits.set_bit(5, val);
    }
    ///
    #[inline]
    pub fn hpm6(&self) -> bool {
        self.bits.get_bit(6)
    }
    #[inline]
    pub fn set_hpm6(&mut self, val: bool) {
        self.bits.set_bit(6, val);
    }
    ///
    #[inline]
    pub fn hpm7(&self) -> bool {
        self.bits.get_bit(7)
    }
    #[inline]
    pub fn set_hpm7(&mut self, val: bool) {
        self.bits.set_bit(7, val);
    }
    ///
    #[inline]
    pub fn hpm8(&self) -> bool {
        self.bits.get_bit(8)
    }
    #[inline]
    pub fn set_hpm8(&mut self, val: bool) {
        self.bits.set_bit(8, val);
    }
    ///
    #[inline]
    pub fn hpm9(&self) -> bool {
        self.bits.get_bit(9)
    }
    #[inline]
    pub fn set_hpm9(&mut self, val: bool) {
        self.bits.set_bit(9, val);
    }
    ///
    #[inline]
    pub fn hpm10(&self) -> bool {
        self.bits.get_bit(10)
    }
    #[inline]
    pub fn set_hpm10(&mut self, val: bool) {
        self.bits.set_bit(10, val);
    }
    ///
    #[inline]
    pub fn hpm11(&self) -> bool {
        self.bits.get_bit(11)
    }
    #[inline]
    pub fn set_hpm11(&mut self, val: bool) {
        self.bits.set_bit(11, val);
    }
    ///
    #[inline]
    pub fn hpm12(&self) -> bool {
        self.bits.get_bit(12)
    }
    #[inline]
    pub fn set_hpm12(&mut self, val: bool) {
        self.bits.set_bit(12, val);
    }
    ///
    #[inline]
    pub fn hpm13(&self) -> bool {
        self.bits.get_bit(13)
    }
    #[inline]
    pub fn set_hpm13(&mut self, val: bool) {
        self.bits.set_bit(13, val);
    }
    ///
    #[inline]
    pub fn hpm14(&self) -> bool {
        self.bits.get_bit(14)
    }
    #[inline]
    pub fn set_hpm14(&mut self, val: bool) {
        self.bits.set_bit(14, val);
    }
    ///
    #[inline]
    pub fn hpm15(&self) -> bool {
        self.bits.get_bit(15)
    }
    #[inline]
    pub fn set_hpm15(&mut self, val: bool) {
        self.bits.set_bit(15, val);
    }
    ///
    #[inline]
    pub fn hpm16(&self) -> bool {
        self.bits.get_bit(16)
    }
    #[inline]
    pub fn set_hpm16(&mut self, val: bool) {
        self.bits.set_bit(16, val);
    }
    ///
    #[inline]
    pub fn hpm17(&self) -> bool {
        self.bits.get_bit(17)
    }
    #[inline]
    pub fn set_hpm17(&mut self, val: bool) {
        self.bits.set_bit(17, val);
    }
    ///
    #[inline]
    pub fn hpm18(&self) -> bool {
        self.bits.get_bit(18)
    }
    #[inline]
    pub fn set_hpm18(&mut self, val: bool) {
        self.bits.set_bit(18, val);
    }
    ///
    #[inline]
    pub fn hpm19(&self) -> bool {
        self.bits.get_bit(19)
    }
    #[inline]
    pub fn set_hpm19(&mut self, val: bool) {
        self.bits.set_bit(19, val);
    }
    ///
    #[inline]
    pub fn hpm20(&self) -> bool {
        self.bits.get_bit(20)
    }
    #[inline]
    pub fn set_hpm20(&mut self, val: bool) {
        self.bits.set_bit(20, val);
    }
    ///
    #[inline]
    pub fn hpm21(&self) -> bool {
        self.bits.get_bit(21)
    }
    #[inline]
    pub fn set_hpm21(&mut self, val: bool) {
        self.bits.set_bit(21, val);
    }
    ///
    #[inline]
    pub fn hpm22(&self) -> bool {
        self.bits.get_bit(22)
    }
    #[inline]
    pub fn set_hpm22(&mut self, val: bool) {
        self.bits.set_bit(22, val);
    }
    ///
    #[inline]
    pub fn hpm23(&self) -> bool {
        self.bits.get_bit(23)
    }
    #[inline]
    pub fn set_hpm23(&mut self, val: bool) {
        self.bits.set_bit(23, val);
    }
    ///
    #[inline]
    pub fn hpm24(&self) -> bool {
        self.bits.get_bit(24)
    }
    #[inline]
    pub fn set_hpm24(&mut self, val: bool) {
        self.bits.set_bit(24, val);
    }
    ///
    #[inline]
    pub fn hpm25(&self) -> bool {
        self.bits.get_bit(25)
    }
    #[inline]
    pub fn set_hpm25(&mut self, val: bool) {
        self.bits.set_bit(25, val);
    }
    ///
    #[inline]
    pub fn hpm26(&self) -> bool {
        self.bits.get_bit(26)
    }
    #[inline]
    pub fn set_hpm26(&mut self, val: bool) {
        self.bits.set_bit(26, val);
    }
    ///
    #[inline]
    pub fn hpm27(&self) -> bool {
        self.bits.get_bit(27)
    }
    #[inline]
    pub fn set_hpm27(&mut self, val: bool) {
        self.bits.set_bit(27, val);
    }
    ///
    #[inline]
    pub fn hpm28(&self) -> bool {
        self.bits.get_bit(28)
    }
    #[inline]
    pub fn set_hpm28(&mut self, val: bool) {
        self.bits.set_bit(28, val);
    }
    ///
    #[inline]
    pub fn hpm29(&self) -> bool {
        self.bits.get_bit(29)
    }
    #[inline]
    pub fn set_hpm29(&mut self, val: bool) {
        self.bits.set_bit(29, val);
    }
    ///
    #[inline]
    pub fn hpm30(&self) -> bool {
        self.bits.get_bit(30)
    }
    #[inline]
    pub fn set_hpm30(&mut self, val: bool) {
        self.bits.set_bit(30, val);
    }
    ///
    #[inline]
    pub fn hpm31(&self) -> bool {
        self.bits.get_bit(31)
    }
    #[inline]
    pub fn set_hpm31(&mut self, val: bool) {
        self.bits.set_bit(31, val);
    }
}
read_csr_as!(Hcounteren, 3602, __read_hcounteren);
write_csr!(3602, __write_hcounteren);
set!(3602, __set_hcounteren);
clear!(3602, __clear_hcounteren);
// bit ops
set_clear_csr!(
    ///
    , set_cy, clear_cy, 1 << 0);
set_clear_csr!(
    ///
    , set_tm, clear_tm, 1 << 1);
set_clear_csr!(
    ///
    , set_ir, clear_ir, 1 << 2);
set_clear_csr!(
    ///
    , set_hpm3, clear_hpm3, 1 << 3);
set_clear_csr!(
    ///
    , set_hpm4, clear_hpm4, 1 << 4);
set_clear_csr!(
    ///
    , set_hpm5, clear_hpm5, 1 << 5);
set_clear_csr!(
    ///
    , set_hpm6, clear_hpm6, 1 << 6);
set_clear_csr!(
    ///
    , set_hpm7, clear_hpm7, 1 << 7);
set_clear_csr!(
    ///
    , set_hpm8, clear_hpm8, 1 << 8);
set_clear_csr!(
    ///
    , set_hpm9, clear_hpm9, 1 << 9);
set_clear_csr!(
    ///
    , set_hpm10, clear_hpm10, 1 << 10);
set_clear_csr!(
    ///
    , set_hpm11, clear_hpm11, 1 << 11);
set_clear_csr!(
    ///
    , set_hpm12, clear_hpm12, 1 << 12);
set_clear_csr!(
    ///
    , set_hpm13, clear_hpm13, 1 << 13);
set_clear_csr!(
    ///
    , set_hpm14, clear_hpm14, 1 << 14);
set_clear_csr!(
    ///
    , set_hpm15, clear_hpm15, 1 << 15);
set_clear_csr!(
    ///
    , set_hpm16, clear_hpm16, 1 << 16);
set_clear_csr!(
    ///
    , set_hpm17, clear_hpm17, 1 << 17);
set_clear_csr!(
    ///
    , set_hpm18, clear_hpm18, 1 << 18);
set_clear_csr!(
    ///
    , set_hpm19, clear_hpm19, 1 << 19);
set_clear_csr!(
    ///
    , set_hpm20, clear_hpm20, 1 << 20);
set_clear_csr!(
    ///
    , set_hpm21, clear_hpm21, 1 << 21);
set_clear_csr!(
    ///
    , set_hpm22, clear_hpm22, 1 << 22);
set_clear_csr!(
    ///
    , set_hpm23, clear_hpm23, 1 << 23);
set_clear_csr!(
    ///
    , set_hpm24, clear_hpm24, 1 << 24);
set_clear_csr!(
    ///
    , set_hpm25, clear_hpm25, 1 << 25);
set_clear_csr!(
    ///
    , set_hpm26, clear_hpm26, 1 << 26);
set_clear_csr!(
    ///
    , set_hpm27, clear_hpm27, 1 << 27);
set_clear_csr!(
    ///
    , set_hpm28, clear_hpm28, 1 << 28);
set_clear_csr!(
    ///
    , set_hpm29, clear_hpm29, 1 << 29);
set_clear_csr!(
    ///
    , set_hpm30, clear_hpm30, 1 << 30);
set_clear_csr!(
    ///
    , set_hpm31, clear_hpm31, 1 << 31);

// enums
