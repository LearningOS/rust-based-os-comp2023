# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [v0.6.0] - 2020-06-20

### Changed

- `Mtvec::trap_mode()`, `Stvec::trap_mode()` and `Utvec::trap_mode()` functions now return `Option<TrapMode>` (breaking change)
- Updated Minimum Supported Rust Version to 1.42.0
- Use `llvm_asm!` instead of `asm!`

### Removed

- vexriscv-specific registers were moved to the `vexriscv` crate

## [v0.5.6] - 2020-03-14

### Added

- Added vexriscv-specific registers

## [v0.5.5] - 2020-02-28

### Added

- Added `riscv32i-unknown-none-elf` target support
- Added user trap setup and handling registers
- Added write methods for the `mip` and `satp` registers
- Added `mideleg` register
- Added Changelog

### Changed

- Fixed MSRV by restricting the upper bound of `bare-metal` version

[Unreleased]: https://github.com/rust-embedded/riscv/compare/v0.6.0...HEAD
[v0.6.0]: https://github.com/rust-embedded/riscv/compare/v0.5.6...v0.6.0
[v0.5.6]: https://github.com/rust-embedded/riscv/compare/v0.5.5...v0.5.6
[v0.5.5]: https://github.com/rust-embedded/riscv/compare/v0.5.4...v0.5.5
