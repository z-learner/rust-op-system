#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_op_system::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use rust_op_system::println;
use x86_64::structures::paging::PageTable;

entry_point!(kernel_main);

// BootInfo is passed by the bootloader.
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use rust_op_system::memory;
    use rust_op_system::memory::BootInfoFrameAllocator;
    use x86_64::structures::paging::Page;
    use x86_64::structures::paging::Translate;
    use x86_64::VirtAddr;

    println!("Hello World{}", "!");

    rust_op_system::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    let mut mapper = unsafe { memory::init(phys_mem_offset) };

    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    // map an unused page
    let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write the string 'New!' to the screen through new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe {
        page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e);
    };

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
