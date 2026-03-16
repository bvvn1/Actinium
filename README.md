# ACTINIUM

Actinium is a toy kernel developed in entirely in Rust. 
The kernel currently includes :
+ Output to the VGA buffer
+ Output to serial port 0x3F8

>[!NOTE]
>Please note that the kernel is still work in progress and bugs are to expected.
>Please feel free to open an issue on any encountered


>[!CAUTION]
>You must have the Rust toolchain setup



> [!IMPORTANT]
> Actinium depends on the QEMU emulator and Bootimage. 

To install Qemu and Bootimage run:

For Arch based distros:
```
sudo pacman -S qemu-desktop
cargo install bootimage
```

for Debian based distros: 
```
apt-get install qemu-system
cargo install bootimage
```
For fedora:
```
dnf install @virtualization
cargo install bootimage
```


After having everything installed use ``` cargo run ``` to build and run the kernel in the QEMU emulator
