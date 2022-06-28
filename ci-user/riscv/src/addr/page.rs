pub use super::*;
pub use bit_field::BitField;

pub trait PageWithL4 {
    fn p4_index(&self) -> usize;
    fn p3_index(&self) -> usize;
    fn p2_index(&self) -> usize;
    fn p1_index(&self) -> usize;
    fn from_page_table_indices(
        p4_index: usize,
        p3_index: usize,
        p2_index: usize,
        p1_index: usize,
    ) -> Self;
}

pub trait PageWithL3 {
    fn p3_index(&self) -> usize;
    fn p2_index(&self) -> usize;
    fn p1_index(&self) -> usize;
    fn from_page_table_indices(p3_index: usize, p2_index: usize, p1_index: usize) -> Self;
}

pub trait PageWithL2 {
    fn p2_index(&self) -> usize;
    fn p1_index(&self) -> usize;
    fn from_page_table_indices(p2_index: usize, p1_index: usize) -> Self;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PageWith<T: VirtualAddress>(T);

impl<T: AddressL4 + VirtualAddress> PageWithL4 for PageWith<T> {
    fn p4_index(&self) -> usize {
        self.0.p4_index()
    }
    fn p3_index(&self) -> usize {
        self.0.p3_index()
    }
    fn p2_index(&self) -> usize {
        self.0.p2_index()
    }
    fn p1_index(&self) -> usize {
        self.0.p1_index()
    }
    fn from_page_table_indices(
        p4_index: usize,
        p3_index: usize,
        p2_index: usize,
        p1_index: usize,
    ) -> Self {
        PageWith::of_addr(T::from_page_table_indices(
            p4_index, p3_index, p2_index, p1_index, 0,
        ))
    }
}
impl<T: AddressL3 + VirtualAddress> PageWithL3 for PageWith<T> {
    fn p3_index(&self) -> usize {
        self.0.p3_index()
    }
    fn p2_index(&self) -> usize {
        self.0.p2_index()
    }
    fn p1_index(&self) -> usize {
        self.0.p1_index()
    }
    fn from_page_table_indices(p3_index: usize, p2_index: usize, p1_index: usize) -> Self {
        PageWith::of_addr(T::from_page_table_indices(p3_index, p2_index, p1_index, 0))
    }
}
impl<T: AddressL2 + VirtualAddress> PageWithL2 for PageWith<T> {
    fn p2_index(&self) -> usize {
        self.0.p2_index()
    }
    fn p1_index(&self) -> usize {
        self.0.p1_index()
    }
    fn from_page_table_indices(p2_index: usize, p1_index: usize) -> Self {
        PageWith::of_addr(T::from_page_table_indices(p2_index, p1_index, 0))
    }
}
impl<T: VirtualAddress> PageWith<T> {
    pub fn of_addr(addr: T) -> Self {
        PageWith(addr.to_4k_aligned())
    }

    pub fn of_vpn(vpn: usize) -> Self {
        PageWith(T::new(vpn << 12))
    }

    pub fn start_address(&self) -> T {
        self.0.clone()
    }

    pub fn number(&self) -> usize {
        self.0.page_number()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FrameWith<T: PhysicalAddress>(T);

impl<T: AddressL4 + PhysicalAddress> PageWithL4 for FrameWith<T> {
    fn p4_index(&self) -> usize {
        self.0.p4_index()
    }
    fn p3_index(&self) -> usize {
        self.0.p3_index()
    }
    fn p2_index(&self) -> usize {
        self.0.p2_index()
    }
    fn p1_index(&self) -> usize {
        self.0.p1_index()
    }
    fn from_page_table_indices(
        p4_index: usize,
        p3_index: usize,
        p2_index: usize,
        p1_index: usize,
    ) -> Self {
        FrameWith::of_addr(T::from_page_table_indices(
            p4_index, p3_index, p2_index, p1_index, 0,
        ))
    }
}
impl<T: AddressL3 + PhysicalAddress> PageWithL3 for FrameWith<T> {
    fn p3_index(&self) -> usize {
        self.0.p3_index()
    }
    fn p2_index(&self) -> usize {
        self.0.p2_index()
    }
    fn p1_index(&self) -> usize {
        self.0.p1_index()
    }
    fn from_page_table_indices(p3_index: usize, p2_index: usize, p1_index: usize) -> Self {
        FrameWith::of_addr(T::from_page_table_indices(p3_index, p2_index, p1_index, 0))
    }
}
impl<T: AddressL2 + PhysicalAddress> PageWithL2 for FrameWith<T> {
    fn p2_index(&self) -> usize {
        self.0.p2_index()
    }
    fn p1_index(&self) -> usize {
        self.0.p1_index()
    }
    fn from_page_table_indices(p2_index: usize, p1_index: usize) -> Self {
        FrameWith::of_addr(T::from_page_table_indices(p2_index, p1_index, 0))
    }
}

impl<T: PhysicalAddress> FrameWith<T> {
    pub fn of_addr(addr: T) -> Self {
        FrameWith(addr.to_4k_aligned())
    }

    #[inline(always)]
    pub fn of_ppn(ppn: usize) -> Self {
        FrameWith(T::new_u64((ppn as u64) << 12))
    }

    pub fn start_address(&self) -> T {
        self.0.clone()
    }

    pub fn number(&self) -> usize {
        self.0.page_number()
    }

    pub unsafe fn as_kernel_mut<'a, 'b, U>(&'a self, linear_offset: u64) -> &'b mut U {
        &mut *(((self.0).as_u64() + linear_offset) as *mut U)
    }
}
