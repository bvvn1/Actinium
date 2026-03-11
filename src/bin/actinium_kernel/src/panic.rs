use core::panic::PanicInfo;

use crate::println;

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}
