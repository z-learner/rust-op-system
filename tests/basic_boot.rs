#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_op_system::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rust_op_system::println;
use rust_op_system::serial_println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_op_system::test_panic_handler(info)
    // serial_println!("it panics");
    // loop {}
}

#[test_case]
fn test_println() {
    println!("test_println output");
}
