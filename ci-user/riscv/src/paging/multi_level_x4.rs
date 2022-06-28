use crate::addr::*;
use crate::asm::{hfence_gvma, hfence_vvma};
use crate::paging::mapper::MapperFlushable;
use crate::paging::multi_level::Rv32PageTableWith;
use crate::paging::multi_level::{Rv39PageTableWith, Rv48PageTableWith};

#[must_use = "Guest Physical Address Table changes must be flushed or ignored."]
pub struct MapperFlushGPA(usize);

impl MapperFlushable for MapperFlushGPA {
    fn new<T: VirtualAddress>(page: PageWith<T>) -> Self {
        MapperFlushGPA(page.start_address().as_usize())
    }
    fn flush(self) {
        unsafe {
            hfence_gvma(self.0, 0);
        }
    }
    fn ignore(self) {}
}

#[must_use = "Guest Page Table changes must be flushed or ignored."]
pub struct MapperFlushGPT(usize);

impl MapperFlushable for MapperFlushGPT {
    fn new<T: VirtualAddress>(page: PageWith<T>) -> Self {
        MapperFlushGPT(page.start_address().as_usize())
    }
    fn flush(self) {
        unsafe {
            hfence_vvma(self.0, 0);
        }
    }
    fn ignore(self) {}
}

pub type Rv32PageTableX4<'a> = Rv32PageTableWith<'a, GPAddrSv32X4, MapperFlushGPA>;
pub type Rv39PageTableX4<'a> = Rv39PageTableWith<'a, GPAddrSv39X4, MapperFlushGPA>;
pub type Rv48PageTableX4<'a> = Rv48PageTableWith<'a, GPAddrSv48X4, MapperFlushGPA>;
pub type Rv32PageTableGuest<'a> = Rv32PageTableWith<'a, VirtAddrSv32, MapperFlushGPT>;
pub type Rv39PageTableGuest<'a> = Rv39PageTableWith<'a, VirtAddrSv39, MapperFlushGPT>;
pub type Rv48PageTableGuest<'a> = Rv48PageTableWith<'a, VirtAddrSv48, MapperFlushGPT>;
