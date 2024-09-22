// don't use and link the standard library
#![no_std]
// no main function
#![no_main]

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello, world!\n";

/*
By using the #[no_mangle] attribute, we disable name mangling to ensure that the Rust
compiler really outputs a function with the name _start. Without the attribute, the
compiler would generate some cryptic _ZN3rust_op_system4_start7hb173fedf945531caE symbol to
give every function a unique name. The attribute is required because we need to tell
the name of the entry point function to the linker in the next step.
*/
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // VGA text buffer format: 2 bytes = ASCII + color
    let vag_buffer_ptr = 0xb8000 as *mut u8;
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vag_buffer_ptr.offset(i as isize * 2) = byte;
            *vag_buffer_ptr.offset(i as isize * 2 + 1) = 0xb; // color(light cyan)
        }
    }

    loop {
        // do nothing
    }
}
