#![no_std]
#![no_main]

mod vga_buffer;

use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

entry_point!(_start);
fn _start(_boot_info: &'static BootInfo) -> ! {
    println!("Hello World");
    loop {}
}
