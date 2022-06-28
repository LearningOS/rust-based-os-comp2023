//! Interrupts

// NOTE: Adapted from cortex-m/src/interrupt.rs
pub use bare_metal::{CriticalSection, Mutex, Nr};
use register::mstatus;

/// Disables all interrupts
#[inline]
pub unsafe fn disable() {
    match () {
        #[cfg(riscv)]
        () => mstatus::clear_mie(),
        #[cfg(not(riscv))]
        () => unimplemented!(),
    }
}

/// Enables all the interrupts
///
/// # Safety
///
/// - Do not call this function inside an `interrupt::free` critical section
#[inline]
pub unsafe fn enable() {
    match () {
        #[cfg(riscv)]
        () => mstatus::set_mie(),
        #[cfg(not(riscv))]
        () => unimplemented!(),
    }
}

/// Execute closure `f` in an interrupt-free context.
///
/// This as also known as a "critical section".
pub fn free<F, R>(f: F) -> R
where
    F: FnOnce(&CriticalSection) -> R,
{
    let mstatus = mstatus::read();

    // disable interrupts
    unsafe {
        disable();
    }

    let r = f(unsafe { &CriticalSection::new() });

    // If the interrupts were active before our `disable` call, then re-enable
    // them. Otherwise, keep them disabled
    if mstatus.mie() {
        unsafe {
            enable();
        }
    }

    r
}
