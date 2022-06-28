# Building
TARGET := riscv64gc-unknown-none-elf
MODE := release
KERNEL_ELF := target/$(TARGET)/$(MODE)/os

# BOARD
BOARD ?= qemu
SBI ?= rustsbi
BOOTLOADER := ../bootloader/$(SBI)-$(BOARD).bin

kernel:
	cargo build --release

clean:
	cargo clean

run: kernel
	timeout --foreground 40s qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(BOOTLOADER) \
		-kernel $(KERNEL_ELF)

.PHONY: build kernel clean run
