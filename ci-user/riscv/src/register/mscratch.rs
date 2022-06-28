//! mscratch register

read_csr_as_usize!(0x340, __read_mscratch);
write_csr_as_usize!(0x340, __write_mscratch);
