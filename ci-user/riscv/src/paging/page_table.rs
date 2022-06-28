use addr::*;
use core::convert::TryInto;
use core::fmt::{Debug, Error, Formatter};
use core::marker::PhantomData;
use core::ops::{Index, IndexMut};

pub type Entries32 = [PageTableEntryX32; RV32_ENTRY_COUNT];
pub type Entries64 = [PageTableEntryX64; RV64_ENTRY_COUNT];

// To avoid const generic.
pub trait PTEIterableSlice<T> {
    fn to_pte_slice<'a>(&'a self) -> &'a [T];
    fn to_pte_slice_mut<'a>(&'a mut self) -> &'a mut [T];
    fn pte_index(&self, index: usize) -> &T;
    fn pte_index_mut(&mut self, index: usize) -> &mut T;
}

impl PTEIterableSlice<PageTableEntryX32> for Entries32 {
    fn to_pte_slice(&self) -> &[PageTableEntryX32] {
        self
    }
    fn to_pte_slice_mut(&mut self) -> &mut [PageTableEntryX32] {
        self
    }
    fn pte_index(&self, index: usize) -> &PageTableEntryX32 {
        &self[index]
    }
    fn pte_index_mut(&mut self, index: usize) -> &mut PageTableEntryX32 {
        &mut self[index]
    }
}
impl PTEIterableSlice<PageTableEntryX64> for Entries64 {
    fn to_pte_slice(&self) -> &[PageTableEntryX64] {
        self
    }
    fn to_pte_slice_mut(&mut self) -> &mut [PageTableEntryX64] {
        self
    }
    fn pte_index(&self, index: usize) -> &PageTableEntryX64 {
        &self[index]
    }
    fn pte_index_mut(&mut self, index: usize) -> &mut PageTableEntryX64 {
        &mut self[index]
    }
}

#[repr(C)]
pub struct PageTableWith<T: PTEIterableSlice<E>, E: PTE> {
    entries: T,
    phantom: PhantomData<E>,
}

impl<T: PTEIterableSlice<E>, E: PTE> PageTableWith<T, E> {
    /// Clears all entries.
    pub fn zero(&mut self) {
        for entry in self.entries.to_pte_slice_mut().iter_mut() {
            entry.set_unused();
        }
    }
}

impl<T: PTEIterableSlice<E>, E: PTE> Index<usize> for PageTableWith<T, E> {
    type Output = E;

    fn index(&self, index: usize) -> &Self::Output {
        self.entries.pte_index(index)
    }
}

impl<T: PTEIterableSlice<E>, E: PTE> IndexMut<usize> for PageTableWith<T, E> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.entries.pte_index_mut(index)
    }
}

impl<T: PTEIterableSlice<E>, E: PTE + Debug> Debug for PageTableWith<T, E> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.debug_map()
            .entries(
                self.entries
                    .to_pte_slice()
                    .iter()
                    .enumerate()
                    .filter(|p| !p.1.is_unused()),
            )
            .finish()
    }
}

pub trait PTE {
    fn is_unused(&self) -> bool;
    fn set_unused(&mut self);
    fn flags(&self) -> PageTableFlags;
    fn ppn(&self) -> usize;
    fn ppn_u64(&self) -> u64;
    fn addr<T: PhysicalAddress>(&self) -> T;
    fn frame<T: PhysicalAddress>(&self) -> FrameWith<T>;
    fn set<T: PhysicalAddress>(&mut self, frame: FrameWith<T>, flags: PageTableFlags);
    fn flags_mut(&mut self) -> &mut PageTableFlags;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PageTableEntryX32(u32);

impl PTE for PageTableEntryX32 {
    fn is_unused(&self) -> bool {
        self.0 == 0
    }
    fn set_unused(&mut self) {
        self.0 = 0;
    }
    fn flags(&self) -> PageTableFlags {
        PageTableFlags::from_bits_truncate(self.0 as usize)
    }
    fn ppn(&self) -> usize {
        self.ppn_u64().try_into().unwrap()
    }
    fn ppn_u64(&self) -> u64 {
        (self.0 >> 10) as u64
    }
    fn addr<T: PhysicalAddress>(&self) -> T {
        T::new_u64((self.ppn() as u64) << 12)
    }
    fn frame<T: PhysicalAddress>(&self) -> FrameWith<T> {
        FrameWith::of_addr(self.addr())
    }
    fn set<T: PhysicalAddress>(&mut self, frame: FrameWith<T>, mut flags: PageTableFlags) {
        // U540 will raise page fault when accessing page with A=0 or D=0
        flags |= EF::ACCESSED | EF::DIRTY;
        self.0 = ((frame.number() << 10) | flags.bits()) as u32;
    }
    fn flags_mut(&mut self) -> &mut PageTableFlags {
        unsafe { &mut *(self as *mut _ as *mut PageTableFlags) }
    }
}

impl Debug for PageTableEntryX32 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.debug_struct("PageTableEntryX32")
            .field("frame", &self.frame::<PhysAddrSv32>())
            .field("flags", &self.flags())
            .finish()
    }
}

#[derive(Copy, Clone)]
pub struct PageTableEntryX64(u64);

impl PTE for PageTableEntryX64 {
    fn is_unused(&self) -> bool {
        self.0 == 0
    }
    fn set_unused(&mut self) {
        self.0 = 0;
    }
    fn flags(&self) -> PageTableFlags {
        PageTableFlags::from_bits_truncate(self.0 as usize)
    }
    fn ppn(&self) -> usize {
        self.ppn_u64().try_into().unwrap()
    }
    fn ppn_u64(&self) -> u64 {
        (self.0 >> 10) as u64
    }
    fn addr<T: PhysicalAddress>(&self) -> T {
        T::new_u64((self.ppn() as u64) << 12)
    }
    fn frame<T: PhysicalAddress>(&self) -> FrameWith<T> {
        FrameWith::of_addr(self.addr())
    }
    fn set<T: PhysicalAddress>(&mut self, frame: FrameWith<T>, mut flags: PageTableFlags) {
        // U540 will raise page fault when accessing page with A=0 or D=0
        flags |= EF::ACCESSED | EF::DIRTY;
        self.0 = ((frame.number() << 10) | flags.bits()) as u64;
    }
    fn flags_mut(&mut self) -> &mut PageTableFlags {
        unsafe { &mut *(self as *mut _ as *mut PageTableFlags) }
    }
}

pub struct PageTableEntryX64Printer<'a, P: PhysicalAddress>(
    &'a PageTableEntryX64,
    PhantomData<*const P>,
);

impl<'a, P: PhysicalAddress> Debug for PageTableEntryX64Printer<'a, P> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.debug_struct("PageTableEntryX64")
            .field("frame", &self.0.frame::<P>())
            .field("flags", &self.0.flags())
            .finish()
    }
}

impl PageTableEntryX64 {
    pub fn debug_sv39<'a>(&'a self) -> PageTableEntryX64Printer<'a, PhysAddrSv39> {
        PageTableEntryX64Printer(self, PhantomData)
    }
    pub fn debug_sv48<'a>(&'a self) -> PageTableEntryX64Printer<'a, PhysAddrSv48> {
        PageTableEntryX64Printer(self, PhantomData)
    }
}

impl Debug for PageTableEntryX64 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        self.debug_sv48().fmt(f)
    }
}

pub const RV64_ENTRY_COUNT: usize = 1 << 9;
pub const RV32_ENTRY_COUNT: usize = 1 << 10;
#[cfg(riscv64)]
pub const ENTRY_COUNT: usize = RV64_ENTRY_COUNT;
#[cfg(riscv32)]
pub const ENTRY_COUNT: usize = RV32_ENTRY_COUNT;
#[cfg(riscv64)]
pub type PageTableEntry = PageTableEntryX64;
#[cfg(riscv32)]
pub type PageTableEntry = PageTableEntryX32;
#[cfg(riscv64)]
pub type Entries = Entries64;
#[cfg(riscv32)]
pub type Entries = Entries32;
#[cfg(not(any(riscv32, riscv64)))]
pub const ENTRY_COUNT: usize = 1 << 0;
#[cfg(not(any(riscv32, riscv64)))]
pub type Entries = Entries64;

pub type PageTableX32 = PageTableWith<Entries32, PageTableEntryX32>;
pub type PageTableX64 = PageTableWith<Entries64, PageTableEntryX64>;
#[cfg(riscv64)]
pub type PageTable = PageTableX64;
#[cfg(riscv32)]
pub type PageTable = PageTableX32;
bitflags! {
    /// Possible flags for a page table entry.
    pub struct PageTableFlags: usize {
        const VALID =       1 << 0;
        const READABLE =    1 << 1;
        const WRITABLE =    1 << 2;
        const EXECUTABLE =  1 << 3;
        const USER =        1 << 4;
        const GLOBAL =      1 << 5;
        const ACCESSED =    1 << 6;
        const DIRTY =       1 << 7;
        const RESERVED1 =   1 << 8;
        const RESERVED2 =   1 << 9;
    }
}

type EF = PageTableFlags;
