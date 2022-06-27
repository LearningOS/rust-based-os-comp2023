//! The panic handler

use crate::console::ANSICON;
use crate::sbi::shutdown;

use core::panic::PanicInfo;

#[panic_handler]
/// panic handler
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println_colorized!(
            "[kernel] Panicked at {}:{} {}",
            ANSICON::FgRed,
            ANSICON::BgDefault,
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        println_colorized!(
            "[kernel] Panicked: {}",
            ANSICON::FgRed,
            ANSICON::BgDefault,
            info.message().unwrap()
        );
    }
    shutdown()
}
