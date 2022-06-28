use super::frame_alloc::*;
use super::mapper::*;
use super::page_table::{PageTableFlags as F, *};
use crate::addr::*;
use core::marker::PhantomData;

/// This struct is a two level page table with `Mapper` trait implemented.
pub struct Rv32PageTableWith<'a, V: VirtualAddress + AddressL2, FL: MapperFlushable> {
    root_table: &'a mut PageTableX32,
    linear_offset: u64, // VA = PA + linear_offset
    phantom: PhantomData<(V, FL)>,
}

impl<'a, V: VirtualAddress + AddressL2, FL: MapperFlushable> Rv32PageTableWith<'a, V, FL> {
    pub fn new(table: &'a mut PageTableX32, linear_offset: usize) -> Self {
        Rv32PageTableWith {
            root_table: table,
            linear_offset: linear_offset as u64,
            phantom: PhantomData,
        }
    }

    fn create_p1_if_not_exist(
        &mut self,
        p2_index: usize,
        allocator: &mut impl FrameAllocatorFor<<Self as Mapper>::P>,
    ) -> Result<&mut PageTableX32, MapToError> {
        if self.root_table[p2_index].is_unused() {
            let frame = allocator.alloc().ok_or(MapToError::FrameAllocationFailed)?;
            self.root_table[p2_index].set(frame.clone(), F::VALID);
            let p1_table: &mut PageTableX32 = unsafe { frame.as_kernel_mut(self.linear_offset) };
            p1_table.zero();
            Ok(p1_table)
        } else {
            let frame = self.root_table[p2_index].frame::<PhysAddrSv32>();
            let p1_table: &mut PageTableX32 = unsafe { frame.as_kernel_mut(self.linear_offset) };
            Ok(p1_table)
        }
    }
}

impl<'a, V: VirtualAddress + AddressL2, FL: MapperFlushable> Mapper
    for Rv32PageTableWith<'a, V, FL>
{
    type P = PhysAddrSv32;
    type V = V;
    type MapperFlush = FL;
    type Entry = PageTableEntryX32;
    fn map_to(
        &mut self,
        page: <Self as MapperExt>::Page,
        frame: <Self as MapperExt>::Frame,
        flags: PageTableFlags,
        allocator: &mut impl FrameAllocatorFor<<Self as Mapper>::P>,
    ) -> Result<Self::MapperFlush, MapToError> {
        let p1_table = self.create_p1_if_not_exist(page.p2_index(), allocator)?;
        if !p1_table[page.p1_index()].is_unused() {
            return Err(MapToError::PageAlreadyMapped);
        }
        p1_table[page.p1_index()].set(frame, flags);
        Ok(Self::MapperFlush::new(page))
    }

    fn unmap(
        &mut self,
        page: <Self as MapperExt>::Page,
    ) -> Result<(<Self as MapperExt>::Frame, Self::MapperFlush), UnmapError<<Self as Mapper>::P>>
    {
        if self.root_table[page.p2_index()].is_unused() {
            return Err(UnmapError::PageNotMapped);
        }
        let p1_frame = self.root_table[page.p2_index()].frame::<PhysAddrSv32>();
        let p1_table: &mut PageTableX32 = unsafe { p1_frame.as_kernel_mut(self.linear_offset) };
        let p1_entry = &mut p1_table[page.p1_index()];
        if !p1_entry.flags().contains(F::VALID) {
            return Err(UnmapError::PageNotMapped);
        }
        let frame = p1_entry.frame();
        p1_entry.set_unused();
        Ok((frame, Self::MapperFlush::new(page)))
    }

    fn ref_entry(
        &mut self,
        page: <Self as MapperExt>::Page,
    ) -> Result<&mut PageTableEntryX32, FlagUpdateError> {
        if self.root_table[page.p2_index()].is_unused() {
            return Err(FlagUpdateError::PageNotMapped);
        }
        let p1_frame = self.root_table[page.p2_index()].frame::<PhysAddrSv32>();
        let p1_table: &mut PageTableX32 = unsafe { p1_frame.as_kernel_mut(self.linear_offset) };
        Ok(&mut p1_table[page.p1_index()])
    }
}

/// This struct is a three level page table with `Mapper` trait implemented.

pub struct Rv39PageTableWith<'a, V: VirtualAddress + AddressL3, FL: MapperFlushable> {
    root_table: &'a mut PageTableX64,
    linear_offset: u64, // VA = PA + linear_offset
    phantom: PhantomData<(V, FL)>,
}

impl<'a, V: VirtualAddress + AddressL3, FL: MapperFlushable> Rv39PageTableWith<'a, V, FL> {
    pub fn new(table: &'a mut PageTableX64, linear_offset: usize) -> Self {
        Rv39PageTableWith {
            root_table: table,
            linear_offset: linear_offset as u64,
            phantom: PhantomData,
        }
    }

    fn create_p1_if_not_exist(
        &mut self,
        p3_index: usize,
        p2_index: usize,
        allocator: &mut impl FrameAllocatorFor<<Self as Mapper>::P>,
    ) -> Result<&mut PageTableX64, MapToError> {
        let p2_table = if self.root_table[p3_index].is_unused() {
            let frame = allocator.alloc().ok_or(MapToError::FrameAllocationFailed)?;
            self.root_table[p3_index].set(frame.clone(), F::VALID);
            let p2_table: &mut PageTableX64 = unsafe { frame.as_kernel_mut(self.linear_offset) };
            p2_table.zero();
            p2_table
        } else {
            let frame = self.root_table[p3_index].frame::<PhysAddrSv39>();
            unsafe { frame.as_kernel_mut(self.linear_offset) }
        };
        if p2_table[p2_index].is_unused() {
            let frame = allocator.alloc().ok_or(MapToError::FrameAllocationFailed)?;
            p2_table[p2_index].set(frame.clone(), F::VALID);
            let p1_table: &mut PageTableX64 = unsafe { frame.as_kernel_mut(self.linear_offset) };
            p1_table.zero();
            Ok(p1_table)
        } else {
            let frame = p2_table[p2_index].frame::<PhysAddrSv39>();
            let p1_table: &mut PageTableX64 = unsafe { frame.as_kernel_mut(self.linear_offset) };
            Ok(p1_table)
        }
    }
}

impl<'a, V: VirtualAddress + AddressL3, FL: MapperFlushable> Mapper
    for Rv39PageTableWith<'a, V, FL>
{
    type P = PhysAddrSv39;
    type V = V;
    type MapperFlush = FL;
    type Entry = PageTableEntryX64;
    fn map_to(
        &mut self,
        page: <Self as MapperExt>::Page,
        frame: <Self as MapperExt>::Frame,
        flags: PageTableFlags,
        allocator: &mut impl FrameAllocatorFor<<Self as Mapper>::P>,
    ) -> Result<Self::MapperFlush, MapToError> {
        let p1_table = self.create_p1_if_not_exist(page.p3_index(), page.p2_index(), allocator)?;
        if !p1_table[page.p1_index()].is_unused() {
            return Err(MapToError::PageAlreadyMapped);
        }
        p1_table[page.p1_index()].set(frame, flags);
        Ok(Self::MapperFlush::new(page))
    }

    fn unmap(
        &mut self,
        page: <Self as MapperExt>::Page,
    ) -> Result<(<Self as MapperExt>::Frame, Self::MapperFlush), UnmapError<<Self as Mapper>::P>>
    {
        if self.root_table[page.p3_index()].is_unused() {
            return Err(UnmapError::PageNotMapped);
        }
        let p2_frame = self.root_table[page.p3_index()].frame::<PhysAddrSv39>();
        let p2_table: &mut PageTableX64 = unsafe { p2_frame.as_kernel_mut(self.linear_offset) };

        if p2_table[page.p2_index()].is_unused() {
            return Err(UnmapError::PageNotMapped);
        }
        let p1_frame = p2_table[page.p2_index()].frame::<PhysAddrSv39>();
        let p1_table: &mut PageTableX64 = unsafe { p1_frame.as_kernel_mut(self.linear_offset) };
        let p1_entry = &mut p1_table[page.p1_index()];
        if !p1_entry.flags().contains(F::VALID) {
            return Err(UnmapError::PageNotMapped);
        }
        let frame = p1_entry.frame();
        p1_entry.set_unused();
        Ok((frame, Self::MapperFlush::new(page)))
    }

    fn ref_entry(
        &mut self,
        page: <Self as MapperExt>::Page,
    ) -> Result<&mut PageTableEntryX64, FlagUpdateError> {
        if self.root_table[page.p3_index()].is_unused() {
            return Err(FlagUpdateError::PageNotMapped);
        }
        let p2_frame = self.root_table[page.p3_index()].frame::<PhysAddrSv39>();
        let p2_table: &mut PageTableX64 = unsafe { p2_frame.as_kernel_mut(self.linear_offset) };
        if p2_table[page.p2_index()].is_unused() {
            return Err(FlagUpdateError::PageNotMapped);
        }

        let p1_frame = p2_table[page.p2_index()].frame::<PhysAddrSv39>();
        let p1_table: &mut PageTableX64 = unsafe { p1_frame.as_kernel_mut(self.linear_offset) };
        Ok(&mut p1_table[page.p1_index()])
    }
}

/// This struct is a four level page table with `Mapper` trait implemented.

pub struct Rv48PageTableWith<'a, V: VirtualAddress + AddressL4, FL: MapperFlushable> {
    root_table: &'a mut PageTableX64,
    linear_offset: u64, // VA = PA + linear_offset
    phantom: PhantomData<(V, FL)>,
}

impl<'a, V: VirtualAddress + AddressL4, FL: MapperFlushable> Rv48PageTableWith<'a, V, FL> {
    pub fn new(table: &'a mut PageTableX64, linear_offset: usize) -> Self {
        Rv48PageTableWith {
            root_table: table,
            linear_offset: linear_offset as u64,
            phantom: PhantomData,
        }
    }

    fn create_p1_if_not_exist(
        &mut self,
        p4_index: usize,
        p3_index: usize,
        p2_index: usize,
        allocator: &mut impl FrameAllocatorFor<<Self as Mapper>::P>,
    ) -> Result<&mut PageTableX64, MapToError> {
        let p3_table = if self.root_table[p4_index].is_unused() {
            let frame = allocator.alloc().ok_or(MapToError::FrameAllocationFailed)?;
            self.root_table[p4_index].set(frame.clone(), F::VALID);
            let p3_table: &mut PageTableX64 = unsafe { frame.as_kernel_mut(self.linear_offset) };
            p3_table.zero();
            p3_table
        } else {
            let frame = self.root_table[p4_index].frame::<PhysAddrSv48>();
            unsafe { frame.as_kernel_mut(self.linear_offset) }
        };

        let p2_table = if p3_table[p3_index].is_unused() {
            let frame = allocator.alloc().ok_or(MapToError::FrameAllocationFailed)?;
            p3_table[p3_index].set(frame.clone(), F::VALID);
            let p2_table: &mut PageTableX64 = unsafe { frame.as_kernel_mut(self.linear_offset) };
            p2_table.zero();
            p2_table
        } else {
            let frame = p3_table[p3_index].frame::<PhysAddrSv48>();
            unsafe { frame.as_kernel_mut(self.linear_offset) }
        };

        if p2_table[p2_index].is_unused() {
            let frame = allocator.alloc().ok_or(MapToError::FrameAllocationFailed)?;
            p2_table[p2_index].set(frame.clone(), F::VALID);
            let p1_table: &mut PageTableX64 = unsafe { frame.as_kernel_mut(self.linear_offset) };
            p1_table.zero();
            Ok(p1_table)
        } else {
            let frame = p2_table[p2_index].frame::<PhysAddrSv48>();
            let p1_table: &mut PageTableX64 = unsafe { frame.as_kernel_mut(self.linear_offset) };
            Ok(p1_table)
        }
    }
}

impl<'a, V: VirtualAddress + AddressL4, FL: MapperFlushable> Mapper
    for Rv48PageTableWith<'a, V, FL>
{
    type P = PhysAddrSv48;
    type V = V;
    type MapperFlush = FL;
    type Entry = PageTableEntryX64;
    fn map_to(
        &mut self,
        page: <Self as MapperExt>::Page,
        frame: <Self as MapperExt>::Frame,
        flags: PageTableFlags,
        allocator: &mut impl FrameAllocatorFor<<Self as Mapper>::P>,
    ) -> Result<Self::MapperFlush, MapToError> {
        let p1_table = self.create_p1_if_not_exist(
            page.p4_index(),
            page.p3_index(),
            page.p2_index(),
            allocator,
        )?;
        if !p1_table[page.p1_index()].is_unused() {
            return Err(MapToError::PageAlreadyMapped);
        }
        p1_table[page.p1_index()].set(frame, flags);
        Ok(Self::MapperFlush::new(page))
    }

    fn unmap(
        &mut self,
        page: <Self as MapperExt>::Page,
    ) -> Result<(<Self as MapperExt>::Frame, Self::MapperFlush), UnmapError<<Self as Mapper>::P>>
    {
        if self.root_table[page.p4_index()].is_unused() {
            return Err(UnmapError::PageNotMapped);
        }
        let p3_frame = self.root_table[page.p4_index()].frame::<PhysAddrSv48>();
        let p3_table: &mut PageTableX64 = unsafe { p3_frame.as_kernel_mut(self.linear_offset) };

        if p3_table[page.p3_index()].is_unused() {
            return Err(UnmapError::PageNotMapped);
        }
        let p2_frame = p3_table[page.p3_index()].frame::<PhysAddrSv48>();
        let p2_table: &mut PageTableX64 = unsafe { p2_frame.as_kernel_mut(self.linear_offset) };

        if p2_table[page.p2_index()].is_unused() {
            return Err(UnmapError::PageNotMapped);
        }
        let p1_frame = p2_table[page.p2_index()].frame::<PhysAddrSv48>();
        let p1_table: &mut PageTableX64 = unsafe { p1_frame.as_kernel_mut(self.linear_offset) };
        let p1_entry = &mut p1_table[page.p1_index()];
        if !p1_entry.flags().contains(F::VALID) {
            return Err(UnmapError::PageNotMapped);
        }
        let frame = p1_entry.frame::<PhysAddrSv48>();
        p1_entry.set_unused();
        Ok((frame, Self::MapperFlush::new(page)))
    }

    fn ref_entry(
        &mut self,
        page: <Self as MapperExt>::Page,
    ) -> Result<&mut PageTableEntryX64, FlagUpdateError> {
        if self.root_table[page.p4_index()].is_unused() {
            return Err(FlagUpdateError::PageNotMapped);
        }
        let p3_frame = self.root_table[page.p4_index()].frame::<PhysAddrSv48>();
        let p3_table: &mut PageTableX64 = unsafe { p3_frame.as_kernel_mut(self.linear_offset) };

        if p3_table[page.p3_index()].is_unused() {
            return Err(FlagUpdateError::PageNotMapped);
        }
        let p2_frame = p3_table[page.p3_index()].frame::<PhysAddrSv48>();
        let p2_table: &mut PageTableX64 = unsafe { p2_frame.as_kernel_mut(self.linear_offset) };
        if p2_table[page.p2_index()].is_unused() {
            return Err(FlagUpdateError::PageNotMapped);
        }

        let p1_frame = p2_table[page.p2_index()].frame::<PhysAddrSv48>();
        let p1_table: &mut PageTableX64 = unsafe { p1_frame.as_kernel_mut(self.linear_offset) };
        Ok(&mut p1_table[page.p1_index()])
    }
}

pub type Rv32PageTable<'a> = Rv32PageTableWith<'a, VirtAddrSv32, MapperFlush>;
pub type Rv39PageTable<'a> = Rv39PageTableWith<'a, VirtAddrSv39, MapperFlush>;
pub type Rv48PageTable<'a> = Rv48PageTableWith<'a, VirtAddrSv48, MapperFlush>;
