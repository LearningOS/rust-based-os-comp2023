//! Assembly instructions

macro_rules! instruction {
    ($(#[$attr:meta])*, $fnname:ident, $asm:expr, $asm_fn:ident) => (
        $(#[$attr])*
        #[inline]
        pub unsafe fn $fnname() {
            match () {
                #[cfg(all(riscv, feature = "inline-asm"))]
                () => core::arch::asm!($asm),

                #[cfg(all(riscv, not(feature = "inline-asm")))]
                () => {
                    extern "C" {
                        fn $asm_fn();
                    }

                    $asm_fn();
                }

                #[cfg(not(riscv))]
                () => unimplemented!(),
            }
        }
    )
}

instruction!(
    /// `EBREAK` instruction wrapper
    ///
    /// Generates a breakpoint exception.
    , ebreak, "ebreak", __ebreak);
instruction!(
    /// `WFI` instruction wrapper
    ///
    /// Provides a hint to the implementation that the current hart can be stalled until an interrupt might need servicing.
    /// The WFI instruction is just a hint, and a legal implementation is to implement WFI as a NOP.
    , wfi, "wfi", __wfi);
instruction!(
    /// `SFENCE.VMA` instruction wrapper (all address spaces and page table levels)
    ///
    /// Synchronizes updates to in-memory memory-management data structures with current execution.
    /// Instruction execution causes implicit reads and writes to these data structures; however, these implicit references
    /// are ordinarily not ordered with respect to loads and stores in the instruction stream.
    /// Executing an `SFENCE.VMA` instruction guarantees that any stores in the instruction stream prior to the
    /// `SFENCE.VMA` are ordered before all implicit references subsequent to the `SFENCE.VMA`.
    , sfence_vma_all, "sfence.vma", __sfence_vma_all);

/// `SFENCE.VMA` instruction wrapper
///
/// Synchronizes updates to in-memory memory-management data structures with current execution.
/// Instruction execution causes implicit reads and writes to these data structures; however, these implicit references
/// are ordinarily not ordered with respect to loads and stores in the instruction stream.
/// Executing an `SFENCE.VMA` instruction guarantees that any stores in the instruction stream prior to the
/// `SFENCE.VMA` are ordered before all implicit references subsequent to the `SFENCE.VMA`.
#[inline]
#[allow(unused_variables)]
pub unsafe fn sfence_vma(asid: usize, addr: usize) {
    match () {
        #[cfg(all(riscv, feature = "inline-asm"))]
        () => core::arch::asm!("sfence.vma {0}, {1}", in(reg) asid, in(reg) addr),

        #[cfg(all(riscv, not(feature = "inline-asm")))]
        () => {
            extern "C" {
                fn __sfence_vma(asid: usize, addr: usize);
            }

            __sfence_vma(asid, addr);
        }

        #[cfg(not(riscv))]
        () => unimplemented!(),
    }
}

mod hypervisor_extension {
    // Generating instructions for Hypervisor extension.
    // There are two kinds of instructions: rs1/rs2 type and rs1/rd type.
    // Also special register handling is required before LLVM could generate inline assembly for extended instructions.
    macro_rules! instruction_hypervisor_extension {
        (RS1_RS2, $(#[$attr:meta])*, $fnname:ident, $asm:expr, $asm_fn:ident) => (
            $(#[$attr])*
            #[inline]
            #[allow(unused_variables)]
            pub unsafe fn $fnname(rs1: usize, rs2: usize) {
                match () {
                    #[cfg(all(riscv, feature = "inline-asm"))]
                    // Since LLVM does not recognize the two registers, we assume they are placed in a0 and a1, correspondingly.
                    () => core::arch::asm!($asm, in("x10") rs1, in("x11") rs2),

                    #[cfg(all(riscv, not(feature = "inline-asm")))]
                    () => {
                        extern "C" {
                            fn $asm_fn(rs1: usize, rs2: usize);
                        }

                        $asm_fn(rs1, rs2);
                    }

                    #[cfg(not(riscv))]
                    () => unimplemented!(),
                }
            }
        );
        (RS1_RD, $(#[$attr:meta])*, $fnname:ident, $asm:expr, $asm_fn:ident) => (
            $(#[$attr])*
            #[inline]
            #[allow(unused_variables)]
            pub unsafe fn $fnname(rs1: usize)->usize {
                match () {
                    #[cfg(all(riscv, feature = "inline-asm"))]
                    () => {
                        let mut result : usize;
                        core::arch::asm!($asm, inlateout("x10") rs1 => result);
                        return result;
                    }

                    #[cfg(all(riscv, not(feature = "inline-asm")))]
                    () => {
                        extern "C" {
                            fn $asm_fn(rs1: usize)->usize;
                        }

                        return $asm_fn(rs1);
                    }

                    #[cfg(not(riscv))]
                    () => unimplemented!(),
                }
            }
        )
    }

    instruction_hypervisor_extension!(RS1_RS2,,hfence_gvma,".word 1656029299",__hfence_gvma);
    instruction_hypervisor_extension!(RS1_RS2,,hfence_vvma,".word 582287475",__hfence_vvma);
    instruction_hypervisor_extension!(RS1_RD,,hlv_b,".word 1610958195",__hlv_b);
    instruction_hypervisor_extension!(RS1_RD,,hlv_bu,".word 1612006771",__hlv_bu);
    instruction_hypervisor_extension!(RS1_RD,,hlv_h,".word 1678067059",__hlv_h);
    instruction_hypervisor_extension!(RS1_RD,,hlv_hu,".word 1679115635",__hlv_hu);
    instruction_hypervisor_extension!(RS1_RD,,hlvx_hu,".word 1681212787",__hlvx_hu);
    instruction_hypervisor_extension!(RS1_RD,,hlv_w,".word 1745175923",__hlv_w);
    instruction_hypervisor_extension!(RS1_RD,,hlvx_wu,".word 1748321651",__hlvx_wu);
    instruction_hypervisor_extension!(RS1_RS2,,hsv_b,".word 1656045683",__hsv_b);
    instruction_hypervisor_extension!(RS1_RS2,,hsv_h,".word 1723154547",__hsv_h);
    instruction_hypervisor_extension!(RS1_RS2,,hsv_w,".word 1790263411",__hsv_w);
    instruction_hypervisor_extension!(RS1_RD,,hlv_wu,".word 1746224499",__hlv_wu);
    instruction_hypervisor_extension!(RS1_RD,,hlv_d,".word 1812284787",__hlv_d);
    instruction_hypervisor_extension!(RS1_RS2,,hsv_d,".word 1857372275",__hsv_d);
}

pub use self::hypervisor_extension::*;
