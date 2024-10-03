#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_op_system::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rust_op_system::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    rust_op_system::init();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");

    rust_op_system::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rust_op_system::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_op_system::test_panic_handler(info)
}
