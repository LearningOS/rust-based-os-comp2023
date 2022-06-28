macro_rules! reg {
    (
        $addr:expr, $csrl:ident, $csrh:ident, $readf:ident, $writef:ident
    ) => {
        /// Machine performance-monitoring counter
        pub mod $csrl {
            read_csr_as_usize!($addr, $readf);
            write_csr_as_usize!($addr, $writef);
            read_composite_csr!(super::$csrh::read(), read());
        }
    }
}

macro_rules! regh {
    (
        $addr:expr, $csrh:ident, $readf:ident, $writef:ident
    ) => {
        /// Upper 32 bits of machine performance-monitoring counter (RV32I only)
        pub mod $csrh {
            read_csr_as_usize_rv32!($addr, $readf);
            write_csr_as_usize_rv32!($addr, $writef);
        }
    }
}

reg!(0xB03, mhpmcounter3,  mhpmcounter3h,  __read_mhpmcounter3,  __write_mhpmcounter3);
reg!(0xB04, mhpmcounter4,  mhpmcounter4h,  __read_mhpmcounter4,  __write_mhpmcounter4);
reg!(0xB05, mhpmcounter5,  mhpmcounter5h,  __read_mhpmcounter5,  __write_mhpmcounter5);
reg!(0xB06, mhpmcounter6,  mhpmcounter6h,  __read_mhpmcounter6,  __write_mhpmcounter6);
reg!(0xB07, mhpmcounter7,  mhpmcounter7h,  __read_mhpmcounter7,  __write_mhpmcounter7);
reg!(0xB08, mhpmcounter8,  mhpmcounter8h,  __read_mhpmcounter8,  __write_mhpmcounter8);
reg!(0xB09, mhpmcounter9,  mhpmcounter9h,  __read_mhpmcounter9,  __write_mhpmcounter9);
reg!(0xB0A, mhpmcounter10, mhpmcounter10h, __read_mhpmcounter10, __write_mhpmcounter10);
reg!(0xB0B, mhpmcounter11, mhpmcounter11h, __read_mhpmcounter11, __write_mhpmcounter11);
reg!(0xB0C, mhpmcounter12, mhpmcounter12h, __read_mhpmcounter12, __write_mhpmcounter12);
reg!(0xB0D, mhpmcounter13, mhpmcounter13h, __read_mhpmcounter13, __write_mhpmcounter13);
reg!(0xB0E, mhpmcounter14, mhpmcounter14h, __read_mhpmcounter14, __write_mhpmcounter14);
reg!(0xB0F, mhpmcounter15, mhpmcounter15h, __read_mhpmcounter15, __write_mhpmcounter15);
reg!(0xB10, mhpmcounter16, mhpmcounter16h, __read_mhpmcounter16, __write_mhpmcounter16);
reg!(0xB11, mhpmcounter17, mhpmcounter17h, __read_mhpmcounter17, __write_mhpmcounter17);
reg!(0xB12, mhpmcounter18, mhpmcounter18h, __read_mhpmcounter18, __write_mhpmcounter18);
reg!(0xB13, mhpmcounter19, mhpmcounter19h, __read_mhpmcounter19, __write_mhpmcounter19);
reg!(0xB14, mhpmcounter20, mhpmcounter20h, __read_mhpmcounter20, __write_mhpmcounter20);
reg!(0xB15, mhpmcounter21, mhpmcounter21h, __read_mhpmcounter21, __write_mhpmcounter21);
reg!(0xB16, mhpmcounter22, mhpmcounter22h, __read_mhpmcounter22, __write_mhpmcounter22);
reg!(0xB17, mhpmcounter23, mhpmcounter23h, __read_mhpmcounter23, __write_mhpmcounter23);
reg!(0xB18, mhpmcounter24, mhpmcounter24h, __read_mhpmcounter24, __write_mhpmcounter24);
reg!(0xB19, mhpmcounter25, mhpmcounter25h, __read_mhpmcounter25, __write_mhpmcounter25);
reg!(0xB1A, mhpmcounter26, mhpmcounter26h, __read_mhpmcounter26, __write_mhpmcounter26);
reg!(0xB1B, mhpmcounter27, mhpmcounter27h, __read_mhpmcounter27, __write_mhpmcounter27);
reg!(0xB1C, mhpmcounter28, mhpmcounter28h, __read_mhpmcounter28, __write_mhpmcounter28);
reg!(0xB1D, mhpmcounter29, mhpmcounter29h, __read_mhpmcounter29, __write_mhpmcounter29);
reg!(0xB1E, mhpmcounter30, mhpmcounter30h, __read_mhpmcounter30, __write_mhpmcounter30);
reg!(0xB1F, mhpmcounter31, mhpmcounter31h, __read_mhpmcounter31, __write_mhpmcounter31);

regh!(0xB83, mhpmcounter3h,  __read_mhpmcounter3h,  __write_mhpmcounter3h);
regh!(0xB84, mhpmcounter4h,  __read_mhpmcounter4h,  __write_mhpmcounter4h);
regh!(0xB85, mhpmcounter5h,  __read_mhpmcounter5h,  __write_mhpmcounter5h);
regh!(0xB86, mhpmcounter6h,  __read_mhpmcounter6h,  __write_mhpmcounter6h);
regh!(0xB87, mhpmcounter7h,  __read_mhpmcounter7h,  __write_mhpmcounter7h);
regh!(0xB88, mhpmcounter8h,  __read_mhpmcounter8h,  __write_mhpmcounter8h);
regh!(0xB89, mhpmcounter9h,  __read_mhpmcounter9h,  __write_mhpmcounter9h);
regh!(0xB8A, mhpmcounter10h, __read_mhpmcounter10h, __write_mhpmcounter10h);
regh!(0xB8B, mhpmcounter11h, __read_mhpmcounter11h, __write_mhpmcounter11h);
regh!(0xB8C, mhpmcounter12h, __read_mhpmcounter12h, __write_mhpmcounter12h);
regh!(0xB8D, mhpmcounter13h, __read_mhpmcounter13h, __write_mhpmcounter13h);
regh!(0xB8E, mhpmcounter14h, __read_mhpmcounter14h, __write_mhpmcounter14h);
regh!(0xB8F, mhpmcounter15h, __read_mhpmcounter15h, __write_mhpmcounter15h);
regh!(0xB90, mhpmcounter16h, __read_mhpmcounter16h, __write_mhpmcounter16h);
regh!(0xB91, mhpmcounter17h, __read_mhpmcounter17h, __write_mhpmcounter17h);
regh!(0xB92, mhpmcounter18h, __read_mhpmcounter18h, __write_mhpmcounter18h);
regh!(0xB93, mhpmcounter19h, __read_mhpmcounter19h, __write_mhpmcounter19h);
regh!(0xB94, mhpmcounter20h, __read_mhpmcounter20h, __write_mhpmcounter20h);
regh!(0xB95, mhpmcounter21h, __read_mhpmcounter21h, __write_mhpmcounter21h);
regh!(0xB96, mhpmcounter22h, __read_mhpmcounter22h, __write_mhpmcounter22h);
regh!(0xB97, mhpmcounter23h, __read_mhpmcounter23h, __write_mhpmcounter23h);
regh!(0xB98, mhpmcounter24h, __read_mhpmcounter24h, __write_mhpmcounter24h);
regh!(0xB99, mhpmcounter25h, __read_mhpmcounter25h, __write_mhpmcounter25h);
regh!(0xB9A, mhpmcounter26h, __read_mhpmcounter26h, __write_mhpmcounter26h);
regh!(0xB9B, mhpmcounter27h, __read_mhpmcounter27h, __write_mhpmcounter27h);
regh!(0xB9C, mhpmcounter28h, __read_mhpmcounter28h, __write_mhpmcounter28h);
regh!(0xB9D, mhpmcounter29h, __read_mhpmcounter29h, __write_mhpmcounter29h);
regh!(0xB9E, mhpmcounter30h, __read_mhpmcounter30h, __write_mhpmcounter30h);
regh!(0xB9F, mhpmcounter31h, __read_mhpmcounter31h, __write_mhpmcounter31h);
