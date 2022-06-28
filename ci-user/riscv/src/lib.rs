//! Low level access to RISC-V processors
//!
//! # Minimum Supported Rust Version (MSRV)
//!
//! This crate is guaranteed to compile on stable Rust 1.42 and up. It *might*
//! compile with older versions but that may change in any new patch release.
//!
//! # Features
//!
//! This crate provides:
//!
//! - Access to core registers like `mstatus` or `mcause`.
//! - Interrupt manipulation mechanisms.
//! - Wrappers around assembly instructions like `WFI`.

#![no_std]
#![cfg_attr(feature = "inline-asm", feature(asm))]
extern crate bare_metal;
#[macro_use]
extern crate bitflags;
extern crate bit_field;

pub mod addr;
pub mod asm;
pub mod interrupt;
pub mod paging;
pub mod register;
