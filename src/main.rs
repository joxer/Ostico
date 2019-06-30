#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;
use core::fmt::Write;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

static HELLO: &[u8] = b"hello world";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello darkness my old friend{}", "!");

    loop {}
}