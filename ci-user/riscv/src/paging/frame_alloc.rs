//! Traits for abstracting away frame allocation and deallocation.

use addr::*;
/// A trait for types that can allocate a frame of memory.
pub trait FrameAllocatorFor<P: PhysicalAddress> {
    /// Allocate a frame of the appropriate size and return it if possible.
    fn alloc(&mut self) -> Option<FrameWith<P>>;
}

/// A trait for types that can deallocate a frame of memory.
pub trait FrameDeallocatorFor<P: PhysicalAddress> {
    /// Deallocate the given frame of memory.
    fn dealloc(&mut self, frame: FrameWith<P>);
}

/// Polyfill for default use cases.

#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
pub trait FrameAllocator {
    fn alloc(&mut self) -> Option<Frame>;
}
#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
pub trait FrameDeallocator {
    fn dealloc(&mut self, frame: Frame);
}

#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
impl<T: FrameAllocator> FrameAllocatorFor<PhysAddr> for T {
    #[inline]
    fn alloc(&mut self) -> Option<Frame> {
        FrameAllocator::alloc(self)
    }
}
#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
impl<T: FrameDeallocator> FrameDeallocatorFor<PhysAddr> for T {
    #[inline]
    fn dealloc(&mut self, frame: Frame) {
        FrameDeallocator::dealloc(self, frame)
    }
}
