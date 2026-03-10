#![no_std]
#![no_main]
static PRINT_TEST: &[u8] = b"the goyim know";

mod panic;
mod vga_buffer;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8; //cast vuv raw pointer

    for (i, &byte) in PRINT_TEST.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}
