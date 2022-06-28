//! mepc register

read_csr_as_usize!(0x341, __read_mepc);
write_csr_as_usize!(0x341, __write_mepc);
