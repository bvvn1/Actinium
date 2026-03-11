#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u32)]
pub enum QemuExitCode {
    Sucess = 0x10,
    Failed = 0x11,
}

fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0x4f);
        port.write(exit_code as u32);
    }
}
