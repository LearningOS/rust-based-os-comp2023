//! medeleg register

use bit_field::BitField;

/// medeleg register
#[derive(Clone, Copy, Debug)]
pub struct Medeleg {
    bits: usize,
}

impl Medeleg {
    /// Returns the contents of the register as raw bits
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    /// Instruction Address Misaligned Delegate
    #[inline]
    pub fn instruction_misaligned(&self) -> bool {
        self.bits.get_bit(0)
    }

    /// Instruction Access Fault Delegate
    #[inline]
    pub fn instruction_fault(&self) -> bool {
        self.bits.get_bit(1)
    }

    /// Illegal Instruction Delegate
    #[inline]
    pub fn illegal_instruction(&self) -> bool {
        self.bits.get_bit(2)
    }

    /// Breakpoint Delegate
    #[inline]
    pub fn breakpoint(&self) -> bool {
        self.bits.get_bit(3)
    }

    /// Load Address Misaligned Delegate
    #[inline]
    pub fn load_misaligned(&self) -> bool {
        self.bits.get_bit(4)
    }

    /// Load Access Fault Delegate
    #[inline]
    pub fn load_fault(&self) -> bool {
        self.bits.get_bit(5)
    }

    /// Store/AMO Address Misaligned Delegate
    #[inline]
    pub fn store_misaligned(&self) -> bool {
        self.bits.get_bit(6)
    }

    /// Store/AMO Access Fault Delegate
    #[inline]
    pub fn store_fault(&self) -> bool {
        self.bits.get_bit(7)
    }

    /// Environment Call from U-mode Delegate
    #[inline]
    pub fn user_env_call(&self) -> bool {
        self.bits.get_bit(8)
    }

    /// Environment Call from S-mode Delegate
    #[inline]
    pub fn supervisor_env_call(&self) -> bool {
        self.bits.get_bit(9)
    }

    /// Environment Call from M-mode Delegate
    #[inline]
    pub fn machine_env_call(&self) -> bool {
        self.bits.get_bit(11)
    }

    /// Instruction Page Fault Delegate
    #[inline]
    pub fn instruction_page_fault(&self) -> bool {
        self.bits.get_bit(12)
    }

    /// Load Page Fault Delegate
    #[inline]
    pub fn load_page_fault(&self) -> bool {
        self.bits.get_bit(13)
    }

    /// Store/AMO Page Fault Delegate
    #[inline]
    pub fn store_page_fault(&self) -> bool {
        self.bits.get_bit(15)
    }
}

read_csr_as!(Medeleg, 0x302, __read_medeleg);
set!(0x302, __set_medeleg);
clear!(0x302, __clear_medeleg);

set_clear_csr!(
    /// Instruction Address Misaligned Delegate
    , set_instruction_misaligned, clear_instruction_misaligned, 1 << 0);
set_clear_csr!(
    /// Instruction Access Fault Delegate
    , set_instruction_fault, clear_instruction_fault, 1 << 1);
set_clear_csr!(
    /// Illegal Instruction Delegate
    , set_illegal_instruction, clear_illegal_instruction, 1 << 2);
set_clear_csr!(
    /// Breakpoint Delegate
    , set_breakpoint, clear_breakpoint, 1 << 3);
set_clear_csr!(
    /// Load Address Misaligned Delegate
    , set_load_misaligned, clear_load_misaligned, 1 << 4);
set_clear_csr!(
    /// Load Access Fault Delegate
    , set_load_fault, clear_load_fault, 1 << 5);
set_clear_csr!(
    /// Store/AMO Address Misaligned Delegate
    , set_store_misaligned, clear_store_misaligned, 1 << 6);
set_clear_csr!(
    /// Store/AMO Access fault
    , set_store_fault, clear_store_fault, 1 << 7);
set_clear_csr!(
    /// Environment Call from U-mode Delegate
    , set_user_env_call, clear_user_env_call, 1 << 8);
set_clear_csr!(
    /// Environment Call from S-mode Delegate
    , set_supervisor_env_call, clear_supervisor_env_call, 1 << 9);
set_clear_csr!(
    /// Environment Call from M-mode Delegate
    , set_machine_env_call, clear_machine_env_call, 1 << 11);
set_clear_csr!(
    /// Instruction Page Fault Delegate
    , set_instruction_page_fault, clear_instruction_page_fault, 1 << 12);
set_clear_csr!(
    /// Load Page Fault Delegate
    , set_load_page_fault, clear_load_page_fault, 1 << 13);
set_clear_csr!(
    /// Store/AMO Page Fault Delegate
    , set_store_page_fault, clear_store_page_fault, 1 << 15);
