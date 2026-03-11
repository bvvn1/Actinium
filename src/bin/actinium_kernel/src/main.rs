#![no_std]
#![no_main]

use crate::vga_buffer::{Buffer, Color, ColorCode, Writer};
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) }
    });
}

pub mod macros;
mod panic;
mod vga_buffer;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let goyko = 12;
    println!("dj pichka {}", goyko);

    loop {}
}
