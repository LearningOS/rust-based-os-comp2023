//! Hypervisor Guest External Interrupt Pending Register.
read_csr_as_usize!(3602, __read_hgeip);
write_csr_as_usize!(3602, __write_hgeip);
