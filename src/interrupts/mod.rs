mod idt;
use lazy_static::lazy_static;
use crate::{serial_print, serial_println};

lazy_static! {
       static ref IDT: idt::Idt = {
        let mut idt = idt::Idt::new();
        idt.set_handler(0, divide_by_zero_handler);
        idt
    };
}

extern "C" fn divide_by_zero_handler() -> ! {
    serial_println!("EXCEPTION: DIVIDE BY ZERO");
    loop {}
}

pub fn init() {
    IDT.load();
}