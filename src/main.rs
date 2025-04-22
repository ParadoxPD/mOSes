#![no_std]
#![no_main]

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

use bootloader::{BootInfo, entry_point};
entry_point!(_start);

fn _start(_boot_info: &'static BootInfo) -> ! {
    let vga_buffer: *mut u8 = 0xb8000 as *mut u8;
    let message = b"Hello World!!!!\n";

    for (i, &byte) in message.iter().enumerate() {
        unsafe {
            *vga_buffer.offset((i * 2) as isize) = byte;
            *vga_buffer.offset((i * 2 + 1) as isize) = 0x0f;
        }
    }

    loop {}
}
