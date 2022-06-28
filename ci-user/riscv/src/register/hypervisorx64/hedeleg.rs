//! Hypervisor Exception Delegation Register.

use bit_field::BitField;

#[derive(Copy, Clone, Debug)]
pub struct Hedeleg {
    bits: usize,
}
impl Hedeleg {
    #[inline]
    pub fn bits(&self) -> usize {
        return self.bits;
    }
    #[inline]
    pub fn from_bits(x: usize) -> Self {
        return Hedeleg { bits: x };
    }
    #[inline]
    pub unsafe fn write(&self) {
        _write(self.bits);
    }
    /// Instruction address misaligned
    #[inline]
    pub fn ex0(&self) -> bool {
        self.bits.get_bit(0)
    }
    #[inline]
    pub fn set_ex0(&mut self, val: bool) {
        self.bits.set_bit(0, val);
    }
    /// Instruction access fault
    #[inline]
    pub fn ex1(&self) -> bool {
        self.bits.get_bit(1)
    }
    #[inline]
    pub fn set_ex1(&mut self, val: bool) {
        self.bits.set_bit(1, val);
    }
    /// Illegal instruction
    #[inline]
    pub fn ex2(&self) -> bool {
        self.bits.get_bit(2)
    }
    #[inline]
    pub fn set_ex2(&mut self, val: bool) {
        self.bits.set_bit(2, val);
    }
    /// Breakpoint
    #[inline]
    pub fn ex3(&self) -> bool {
        self.bits.get_bit(3)
    }
    #[inline]
    pub fn set_ex3(&mut self, val: bool) {
        self.bits.set_bit(3, val);
    }
    /// Load address misaligned
    #[inline]
    pub fn ex4(&self) -> bool {
        self.bits.get_bit(4)
    }
    #[inline]
    pub fn set_ex4(&mut self, val: bool) {
        self.bits.set_bit(4, val);
    }
    /// Load access fault
    #[inline]
    pub fn ex5(&self) -> bool {
        self.bits.get_bit(5)
    }
    #[inline]
    pub fn set_ex5(&mut self, val: bool) {
        self.bits.set_bit(5, val);
    }
    /// Store/AMO address misaligned
    #[inline]
    pub fn ex6(&self) -> bool {
        self.bits.get_bit(6)
    }
    #[inline]
    pub fn set_ex6(&mut self, val: bool) {
        self.bits.set_bit(6, val);
    }
    /// Store/AMO access fault
    #[inline]
    pub fn ex7(&self) -> bool {
        self.bits.get_bit(7)
    }
    #[inline]
    pub fn set_ex7(&mut self, val: bool) {
        self.bits.set_bit(7, val);
    }
    /// Environment call from U-mode or VU-mode
    #[inline]
    pub fn ex8(&self) -> bool {
        self.bits.get_bit(8)
    }
    #[inline]
    pub fn set_ex8(&mut self, val: bool) {
        self.bits.set_bit(8, val);
    }
    /// Instruction page fault
    #[inline]
    pub fn ex12(&self) -> bool {
        self.bits.get_bit(12)
    }
    #[inline]
    pub fn set_ex12(&mut self, val: bool) {
        self.bits.set_bit(12, val);
    }
    /// Load page fault
    #[inline]
    pub fn ex13(&self) -> bool {
        self.bits.get_bit(13)
    }
    #[inline]
    pub fn set_ex13(&mut self, val: bool) {
        self.bits.set_bit(13, val);
    }
    /// Store/AMO page fault
    #[inline]
    pub fn ex15(&self) -> bool {
        self.bits.get_bit(15)
    }
    #[inline]
    pub fn set_ex15(&mut self, val: bool) {
        self.bits.set_bit(15, val);
    }
}
read_csr_as!(Hedeleg, 1538, __read_hedeleg);
write_csr!(1538, __write_hedeleg);
set!(1538, __set_hedeleg);
clear!(1538, __clear_hedeleg);
// bit ops
set_clear_csr!(
    ///Instruction address misaligned
    , set_ex0, clear_ex0, 1 << 0);
set_clear_csr!(
    ///Instruction access fault
    , set_ex1, clear_ex1, 1 << 1);
set_clear_csr!(
    ///Illegal instruction 
    , set_ex2, clear_ex2, 1 << 2);
set_clear_csr!(
    ///Breakpoint 
    , set_ex3, clear_ex3, 1 << 3);
set_clear_csr!(
    ///Load address misaligned 
    , set_ex4, clear_ex4, 1 << 4);
set_clear_csr!(
    ///Load access fault 
    , set_ex5, clear_ex5, 1 << 5);
set_clear_csr!(
    ///Store/AMO address misaligned 
    , set_ex6, clear_ex6, 1 << 6);
set_clear_csr!(
    ///Store/AMO access fault 
    , set_ex7, clear_ex7, 1 << 7);
set_clear_csr!(
    ///Environment call from U-mode or VU-mode 
    , set_ex8, clear_ex8, 1 << 8);
set_clear_csr!(
    ///Instruction page fault 
    , set_ex12, clear_ex12, 1 << 12);
set_clear_csr!(
    ///Load page fault 
    , set_ex13, clear_ex13, 1 << 13);
set_clear_csr!(
    ///Store/AMO page fault 
    , set_ex15, clear_ex15, 1 << 15);

// enums
