macro_rules! reg {
    (
        $addr:expr, $csr:ident, $readf:ident, $writef:ident
    ) => {
        /// Machine performance-monitoring event selector
        pub mod $csr {
            read_csr_as_usize!($addr, $readf);
            write_csr_as_usize!($addr, $writef);
        }
    };
}

reg!(0x323, mhpmevent3, __read_mhpmevent3, __write_mhpmevent3);
reg!(0x324, mhpmevent4, __read_mhpmevent4, __write_mhpmevent4);
reg!(0x325, mhpmevent5, __read_mhpmevent5, __write_mhpmevent5);
reg!(0x326, mhpmevent6, __read_mhpmevent6, __write_mhpmevent6);
reg!(0x327, mhpmevent7, __read_mhpmevent7, __write_mhpmevent7);
reg!(0x328, mhpmevent8, __read_mhpmevent8, __write_mhpmevent8);
reg!(0x329, mhpmevent9, __read_mhpmevent9, __write_mhpmevent9);
reg!(0x32A, mhpmevent10, __read_mhpmevent10, __write_mhpmevent10);
reg!(0x32B, mhpmevent11, __read_mhpmevent11, __write_mhpmevent11);
reg!(0x32C, mhpmevent12, __read_mhpmevent12, __write_mhpmevent12);
reg!(0x32D, mhpmevent13, __read_mhpmevent13, __write_mhpmevent13);
reg!(0x32E, mhpmevent14, __read_mhpmevent14, __write_mhpmevent14);
reg!(0x32F, mhpmevent15, __read_mhpmevent15, __write_mhpmevent15);
reg!(0x330, mhpmevent16, __read_mhpmevent16, __write_mhpmevent16);
reg!(0x331, mhpmevent17, __read_mhpmevent17, __write_mhpmevent17);
reg!(0x332, mhpmevent18, __read_mhpmevent18, __write_mhpmevent18);
reg!(0x333, mhpmevent19, __read_mhpmevent19, __write_mhpmevent19);
reg!(0x334, mhpmevent20, __read_mhpmevent20, __write_mhpmevent20);
reg!(0x335, mhpmevent21, __read_mhpmevent21, __write_mhpmevent21);
reg!(0x336, mhpmevent22, __read_mhpmevent22, __write_mhpmevent22);
reg!(0x337, mhpmevent23, __read_mhpmevent23, __write_mhpmevent23);
reg!(0x338, mhpmevent24, __read_mhpmevent24, __write_mhpmevent24);
reg!(0x339, mhpmevent25, __read_mhpmevent25, __write_mhpmevent25);
reg!(0x33A, mhpmevent26, __read_mhpmevent26, __write_mhpmevent26);
reg!(0x33B, mhpmevent27, __read_mhpmevent27, __write_mhpmevent27);
reg!(0x33C, mhpmevent28, __read_mhpmevent28, __write_mhpmevent28);
reg!(0x33D, mhpmevent29, __read_mhpmevent29, __write_mhpmevent29);
reg!(0x33E, mhpmevent30, __read_mhpmevent30, __write_mhpmevent30);
reg!(0x33F, mhpmevent31, __read_mhpmevent31, __write_mhpmevent31);
