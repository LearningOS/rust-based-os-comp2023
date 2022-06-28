macro_rules! reg {
    (
        $addr:expr, $csrl:ident, $csrh:ident, $readf:ident, $writef:ident
    ) => {
        /// Performance-monitoring counter
        pub mod $csrl {
            read_csr_as_usize!($addr, $readf);
            read_composite_csr!(super::$csrh::read(), read());
        }
    }
}

macro_rules! regh {
    (
        $addr:expr, $csrh:ident, $readf:ident, $writef:ident
    ) => {
        /// Upper 32 bits of performance-monitoring counter (RV32I only)
        pub mod $csrh {
            read_csr_as_usize_rv32!($addr, $readf);
        }
    }
}

reg!(0xC03, hpmcounter3,  hpmcounter3h,  __read_hpmcounter3,  __write_hpmcounter3);
reg!(0xC04, hpmcounter4,  hpmcounter4h,  __read_hpmcounter4,  __write_hpmcounter4);
reg!(0xC05, hpmcounter5,  hpmcounter5h,  __read_hpmcounter5,  __write_hpmcounter5);
reg!(0xC06, hpmcounter6,  hpmcounter6h,  __read_hpmcounter6,  __write_hpmcounter6);
reg!(0xC07, hpmcounter7,  hpmcounter7h,  __read_hpmcounter7,  __write_hpmcounter7);
reg!(0xC08, hpmcounter8,  hpmcounter8h,  __read_hpmcounter8,  __write_hpmcounter8);
reg!(0xC09, hpmcounter9,  hpmcounter9h,  __read_hpmcounter9,  __write_hpmcounter9);
reg!(0xC0A, hpmcounter10, hpmcounter10h, __read_hpmcounter10, __write_hpmcounter10);
reg!(0xC0B, hpmcounter11, hpmcounter11h, __read_hpmcounter11, __write_hpmcounter11);
reg!(0xC0C, hpmcounter12, hpmcounter12h, __read_hpmcounter12, __write_hpmcounter12);
reg!(0xC0D, hpmcounter13, hpmcounter13h, __read_hpmcounter13, __write_hpmcounter13);
reg!(0xC0E, hpmcounter14, hpmcounter14h, __read_hpmcounter14, __write_hpmcounter14);
reg!(0xC0F, hpmcounter15, hpmcounter15h, __read_hpmcounter15, __write_hpmcounter15);
reg!(0xC10, hpmcounter16, hpmcounter16h, __read_hpmcounter16, __write_hpmcounter16);
reg!(0xC11, hpmcounter17, hpmcounter17h, __read_hpmcounter17, __write_hpmcounter17);
reg!(0xC12, hpmcounter18, hpmcounter18h, __read_hpmcounter18, __write_hpmcounter18);
reg!(0xC13, hpmcounter19, hpmcounter19h, __read_hpmcounter19, __write_hpmcounter19);
reg!(0xC14, hpmcounter20, hpmcounter20h, __read_hpmcounter20, __write_hpmcounter20);
reg!(0xC15, hpmcounter21, hpmcounter21h, __read_hpmcounter21, __write_hpmcounter21);
reg!(0xC16, hpmcounter22, hpmcounter22h, __read_hpmcounter22, __write_hpmcounter22);
reg!(0xC17, hpmcounter23, hpmcounter23h, __read_hpmcounter23, __write_hpmcounter23);
reg!(0xC18, hpmcounter24, hpmcounter24h, __read_hpmcounter24, __write_hpmcounter24);
reg!(0xC19, hpmcounter25, hpmcounter25h, __read_hpmcounter25, __write_hpmcounter25);
reg!(0xC1A, hpmcounter26, hpmcounter26h, __read_hpmcounter26, __write_hpmcounter26);
reg!(0xC1B, hpmcounter27, hpmcounter27h, __read_hpmcounter27, __write_hpmcounter27);
reg!(0xC1C, hpmcounter28, hpmcounter28h, __read_hpmcounter28, __write_hpmcounter28);
reg!(0xC1D, hpmcounter29, hpmcounter29h, __read_hpmcounter29, __write_hpmcounter29);
reg!(0xC1E, hpmcounter30, hpmcounter30h, __read_hpmcounter30, __write_hpmcounter30);
reg!(0xC1F, hpmcounter31, hpmcounter31h, __read_hpmcounter31, __write_hpmcounter31);

regh!(0xC83, hpmcounter3h,  __read_hpmcounter3h,  __write_hpmcounter3h);
regh!(0xC84, hpmcounter4h,  __read_hpmcounter4h,  __write_hpmcounter4h);
regh!(0xC85, hpmcounter5h,  __read_hpmcounter5h,  __write_hpmcounter5h);
regh!(0xC86, hpmcounter6h,  __read_hpmcounter6h,  __write_hpmcounter6h);
regh!(0xC87, hpmcounter7h,  __read_hpmcounter7h,  __write_hpmcounter7h);
regh!(0xC88, hpmcounter8h,  __read_hpmcounter8h,  __write_hpmcounter8h);
regh!(0xC89, hpmcounter9h,  __read_hpmcounter9h,  __write_hpmcounter9h);
regh!(0xC8A, hpmcounter10h, __read_hpmcounter10h, __write_hpmcounter10h);
regh!(0xC8B, hpmcounter11h, __read_hpmcounter11h, __write_hpmcounter11h);
regh!(0xC8C, hpmcounter12h, __read_hpmcounter12h, __write_hpmcounter12h);
regh!(0xC8D, hpmcounter13h, __read_hpmcounter13h, __write_hpmcounter13h);
regh!(0xC8E, hpmcounter14h, __read_hpmcounter14h, __write_hpmcounter14h);
regh!(0xC8F, hpmcounter15h, __read_hpmcounter15h, __write_hpmcounter15h);
regh!(0xC90, hpmcounter16h, __read_hpmcounter16h, __write_hpmcounter16h);
regh!(0xC91, hpmcounter17h, __read_hpmcounter17h, __write_hpmcounter17h);
regh!(0xC92, hpmcounter18h, __read_hpmcounter18h, __write_hpmcounter18h);
regh!(0xC93, hpmcounter19h, __read_hpmcounter19h, __write_hpmcounter19h);
regh!(0xC94, hpmcounter20h, __read_hpmcounter20h, __write_hpmcounter20h);
regh!(0xC95, hpmcounter21h, __read_hpmcounter21h, __write_hpmcounter21h);
regh!(0xC96, hpmcounter22h, __read_hpmcounter22h, __write_hpmcounter22h);
regh!(0xC97, hpmcounter23h, __read_hpmcounter23h, __write_hpmcounter23h);
regh!(0xC98, hpmcounter24h, __read_hpmcounter24h, __write_hpmcounter24h);
regh!(0xC99, hpmcounter25h, __read_hpmcounter25h, __write_hpmcounter25h);
regh!(0xC9A, hpmcounter26h, __read_hpmcounter26h, __write_hpmcounter26h);
regh!(0xC9B, hpmcounter27h, __read_hpmcounter27h, __write_hpmcounter27h);
regh!(0xC9C, hpmcounter28h, __read_hpmcounter28h, __write_hpmcounter28h);
regh!(0xC9D, hpmcounter29h, __read_hpmcounter29h, __write_hpmcounter29h);
regh!(0xC9E, hpmcounter30h, __read_hpmcounter30h, __write_hpmcounter30h);
regh!(0xC9F, hpmcounter31h, __read_hpmcounter31h, __write_hpmcounter31h);
