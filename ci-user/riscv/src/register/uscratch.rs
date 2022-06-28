//! uscratch register

read_csr_as_usize!(0x040, __read_uscratch);
write_csr_as_usize!(0x040, __write_uscratch);
