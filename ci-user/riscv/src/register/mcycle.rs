//! mcycle register

read_csr_as_usize!(0xB00, __read_mcycle);
read_composite_csr!(super::mcycleh::read(), read());
