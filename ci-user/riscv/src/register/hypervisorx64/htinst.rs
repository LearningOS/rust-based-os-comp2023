//! Hypervisor Trap Instruction Register.
read_csr_as_usize!(1610, __read_htinst);
write_csr_as_usize!(1610, __write_htinst);
