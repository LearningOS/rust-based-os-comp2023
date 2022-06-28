//! Hypervisor Guest External Interrupt Enable Register.
read_csr_as_usize!(1543, __read_hgeie);
write_csr_as_usize!(1543, __write_hgeie);
