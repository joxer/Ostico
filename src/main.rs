
#![feature(asm)]
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ostico::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use ostico::println;
use ostico::interrupts;
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    interrupts::init();
    divide_by_zero();
    loop {}
    #[cfg(test)]
    test_main();

}

fn divide_by_zero() {
    unsafe {
        asm!("mov dx, 0; div dx" ::: "ax", "dx" : "volatile", "intel")
    }
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    ostico::test_panic_handler(info)
}