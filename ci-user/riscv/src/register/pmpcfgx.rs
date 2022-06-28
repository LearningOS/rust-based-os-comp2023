/// Physical memory protection configuration
pub mod pmpcfg0 {
    read_csr_as_usize!(0x3A0, __read_pmpcfg0);
    write_csr_as_usize!(0x3A0, __write_pmpcfg0);
}

/// Physical memory protection configuration, RV32 only
pub mod pmpcfg1 {
    read_csr_as_usize_rv32!(0x3A1, __read_pmpcfg1);
    write_csr_as_usize_rv32!(0x3A1, __write_pmpcfg1);
}

/// Physical memory protection configuration
pub mod pmpcfg2 {
    read_csr_as_usize!(0x3A2, __read_pmpcfg2);
    write_csr_as_usize!(0x3A2, __write_pmpcfg2);
}

/// Physical memory protection configuration, RV32 only
pub mod pmpcfg3 {
    read_csr_as_usize_rv32!(0x3A3, __read_pmpcfg3);
    write_csr_as_usize_rv32!(0x3A3, __write_pmpcfg3);
}
