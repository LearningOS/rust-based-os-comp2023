//! Hypervisor Time Delta Register.
read_composite_csr!(super::htimedeltah::read(), read());
read_csr_as_usize!(1541, __read_htimedelta);
write_csr_as_usize!(1541, __write_htimedelta);
