//! sscratch register

read_csr_as_usize!(0x140, __read_sscratch);
write_csr_as_usize!(0x140, __write_sscratch);
