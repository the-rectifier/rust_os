#![no_std]
#![no_main]
#![reexport_test_harness_main = "test_main"]
#![test_runner(rust_os::test_runner)]
#![feature(custom_test_frameworks)]

use core::panic::PanicInfo;
use rust_os::println;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    println!("test_println output");
}
