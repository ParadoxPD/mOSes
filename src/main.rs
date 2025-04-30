#![no_std]
#![no_main]

mod vga_buffer;

use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

entry_point!(_start);
fn _start(_boot_info: &'static BootInfo) -> ! {
    println!("Hello\nWorld {}", "\nIs it workking properly");
    panic!("ahhhhhhhhhhh Reached the end");
    //loop {}
}
