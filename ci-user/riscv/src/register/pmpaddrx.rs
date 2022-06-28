macro_rules! reg {
    (
        $addr:expr, $csr:ident, $readf:ident, $writef:ident
    ) => {
        /// Physical memory protection address register
        pub mod $csr {
            read_csr_as_usize!($addr, $readf);
            write_csr_as_usize!($addr, $writef);
        }
    };
}

reg!(0x3B0, pmpaddr0, __read_pmpaddr0, __write_pmpaddr0);
reg!(0x3B1, pmpaddr1, __read_pmpaddr1, __write_pmpaddr1);
reg!(0x3B2, pmpaddr2, __read_pmpaddr2, __write_pmpaddr2);
reg!(0x3B3, pmpaddr3, __read_pmpaddr3, __write_pmpaddr3);
reg!(0x3B4, pmpaddr4, __read_pmpaddr4, __write_pmpaddr4);
reg!(0x3B5, pmpaddr5, __read_pmpaddr5, __write_pmpaddr5);
reg!(0x3B6, pmpaddr6, __read_pmpaddr6, __write_pmpaddr6);
reg!(0x3B7, pmpaddr7, __read_pmpaddr7, __write_pmpaddr7);
reg!(0x3B8, pmpaddr8, __read_pmpaddr8, __write_pmpaddr8);
reg!(0x3B9, pmpaddr9, __read_pmpaddr9, __write_pmpaddr9);
reg!(0x3BA, pmpaddr10, __read_pmpaddr10, __write_pmpaddr10);
reg!(0x3BB, pmpaddr11, __read_pmpaddr11, __write_pmpaddr11);
reg!(0x3BC, pmpaddr12, __read_pmpaddr12, __write_pmpaddr12);
reg!(0x3BD, pmpaddr13, __read_pmpaddr13, __write_pmpaddr13);
reg!(0x3BE, pmpaddr14, __read_pmpaddr14, __write_pmpaddr14);
reg!(0x3BF, pmpaddr15, __read_pmpaddr15, __write_pmpaddr15);
