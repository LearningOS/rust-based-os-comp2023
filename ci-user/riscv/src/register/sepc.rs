//! sepc register

read_csr_as_usize!(0x141, __read_sepc);
write_csr_as_usize!(0x141, __write_sepc);
