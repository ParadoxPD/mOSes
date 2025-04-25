#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

use bootloader::{BootInfo, entry_point};
entry_point!(_start);

fn _start(_boot_info: &'static BootInfo) -> ! {
    vga_buffer::print_something();

    loop {}
}
