use super::frame_alloc::*;
use super::page_table::*;
use addr::*;

pub trait Mapper {
    type P: PhysicalAddress;
    type V: VirtualAddress;
    type MapperFlush: MapperFlushable;
    type Entry: PTE;

    /// Creates a new mapping in the page table.
    ///
    /// This function might need additional physical frames to create new page tables. These
    /// frames are allocated from the `allocator` argument. At most three frames are required.
    fn map_to(
        &mut self,
        page: PageWith<Self::V>,
        frame: FrameWith<Self::P>,
        flags: PageTableFlags,
        allocator: &mut impl FrameAllocatorFor<<Self as Mapper>::P>,
    ) -> Result<Self::MapperFlush, MapToError>;

    /// Removes a mapping from the page table and returns the frame that used to be mapped.
    ///
    /// Note that no page tables or pages are deallocated.
    fn unmap(
        &mut self,
        page: PageWith<Self::V>,
    ) -> Result<(FrameWith<Self::P>, Self::MapperFlush), UnmapError<<Self as Mapper>::P>>;

    /// Get the reference of the specified `page` entry
    fn ref_entry(&mut self, page: PageWith<Self::V>) -> Result<&mut Self::Entry, FlagUpdateError>;

    /// Updates the flags of an existing mapping.
    fn update_flags(
        &mut self,
        page: PageWith<Self::V>,
        flags: PageTableFlags,
    ) -> Result<Self::MapperFlush, FlagUpdateError> {
        self.ref_entry(page).map(|e| {
            e.set(e.frame::<Self::P>(), flags);
            Self::MapperFlush::new(page)
        })
    }

    /// Return the frame that the specified page is mapped to.
    fn translate_page(&mut self, page: PageWith<Self::V>) -> Option<FrameWith<Self::P>> {
        match self.ref_entry(page) {
            Ok(e) => {
                if e.is_unused() {
                    None
                } else {
                    Some(e.frame())
                }
            }
            Err(_) => None,
        }
    }

    /// Maps the given frame to the virtual page with the same address.
    fn identity_map(
        &mut self,
        frame: FrameWith<Self::P>,
        flags: PageTableFlags,
        allocator: &mut impl FrameAllocatorFor<<Self as Mapper>::P>,
    ) -> Result<Self::MapperFlush, MapToError> {
        let page = PageWith::of_addr(Self::V::new(frame.start_address().as_usize()));
        self.map_to(page, frame, flags, allocator)
    }
}

pub trait MapperFlushable {
    /// Create a new flush promise
    fn new<T: VirtualAddress>(page: PageWith<T>) -> Self;
    /// Flush the page from the TLB to ensure that the newest mapping is used.
    fn flush(self);
    /// Don't flush the TLB and silence the “must be used” warning.
    fn ignore(self);
}

#[must_use = "Page Table changes must be flushed or ignored."]
pub struct MapperFlush(usize);

impl MapperFlushable for MapperFlush {
    fn new<T: VirtualAddress>(page: PageWith<T>) -> Self {
        MapperFlush(page.start_address().as_usize())
    }
    fn flush(self) {
        unsafe {
            crate::asm::sfence_vma(0, self.0);
        }
    }
    fn ignore(self) {}
}

/// This error is returned from `map_to` and similar methods.
#[derive(Debug)]
pub enum MapToError {
    /// An additional frame was needed for the mapping process, but the frame allocator
    /// returned `None`.
    FrameAllocationFailed,
    /// An upper level page table entry has the `HUGE_PAGE` flag set, which means that the
    /// given page is part of an already mapped huge page.
    ParentEntryHugePage,
    /// The given page is already mapped to a physical frame.
    PageAlreadyMapped,
}

/// An error indicating that an `unmap` call failed.
#[derive(Debug)]
pub enum UnmapError<P: PhysicalAddress> {
    /// An upper level page table entry has the `HUGE_PAGE` flag set, which means that the
    /// given page is part of a huge page and can't be freed individually.
    ParentEntryHugePage,
    /// The given page is not mapped to a physical frame.
    PageNotMapped,
    /// The page table entry for the given page points to an invalid physical address.
    InvalidFrameAddress(P),
}

/// An error indicating that an `update_flags` call failed.
#[derive(Debug)]
pub enum FlagUpdateError {
    /// The given page is not mapped to a physical frame.
    PageNotMapped,
}

pub trait MapperExt {
    type Page;
    type Frame;
}

impl<T: Mapper> MapperExt for T {
    type Page = PageWith<<T as Mapper>::V>;
    type Frame = FrameWith<<T as Mapper>::P>;
}
