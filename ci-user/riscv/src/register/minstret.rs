//! minstret register

read_csr_as_usize!(0xB02, __read_minstret);
read_composite_csr!(super::minstreth::read(), read());
