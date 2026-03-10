#![no_std]
#![no_main]
static PRINT_TEST: &[u8] = b"we are so back x500 we are so back x500 we are so back x500 we are so back x500 we are so back x500 we are so back x500 we are so back x500 we are so back x500we are so back x500 we are so back x500  we are so back x500 we are so back x500  we are so back x500we are so back x500";

mod panic;
mod vga_buffer;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let vga_buffer: &mut [u8; 2048] = {
        let address = 0xb8000;
        let ptr = address as *mut [u8; 2048];
        unsafe { &mut *ptr }
    };

    for (i, &byte) in PRINT_TEST.iter().enumerate() {
        vga_buffer[i * 2] = byte;
        vga_buffer[i * 2 + 1] = 0xb;
    }

    loop {}
}
