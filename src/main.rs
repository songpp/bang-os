#![no_std]
#![no_main]

use core::borrow::BorrowMut;
use core::fmt::Write;
use core::panic::PanicInfo;

use bootloader::{BootInfo, entry_point};

mod logger;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    // turn the screen gray
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        let info = framebuffer.info();
        let fl = logger::LockedLogger::new(framebuffer.buffer_mut(), info);
        let mut log = fl.0.lock();

        for x in 1..100 {
            writeln!(log, " what happened! {}", x);
        }
    }
    // vga_buffer::print_something();
    loop {}
}
