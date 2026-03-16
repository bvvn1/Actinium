#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use crate::{
    status::exit_qemu,
    vga_buffer::{Buffer, Color, ColorCode, Writer},
};
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) }
    });
}

mod panic;
mod serial;
mod status;
mod vga_buffer;

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}... \t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(status::QemuExitCode::Sucess);
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let goyko = 12;
    serial_print!("go {}", goyko);
    println!("goycho borisov 67 {}", goyko);

    #[cfg(test)]
    test_main();

    loop {}
}
