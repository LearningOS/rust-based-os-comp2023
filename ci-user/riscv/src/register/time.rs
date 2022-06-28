//! time register

read_csr_as_usize!(0xC01, __read_time);
read_composite_csr!(super::timeh::read(), read());
