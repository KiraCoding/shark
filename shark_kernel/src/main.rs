#![no_std]
#![no_main]

use core::slice::from_raw_parts_mut;
use uefi::boot::{exit_boot_services, image_handle, open_protocol_exclusive, MemoryType};
use uefi::proto::console::gop::GraphicsOutput;
use uefi::{entry, Status};

#[entry]
#[cfg(target_os = "uefi")]
fn efi_main() -> Status {
    let mut gop = open_protocol_exclusive::<GraphicsOutput>(image_handle()).unwrap();
    let mut framebuffer = gop.frame_buffer();
    let size = framebuffer.size();
    let ptr = framebuffer.as_mut_ptr();
    let framebuffer = unsafe { from_raw_parts_mut(ptr, size) };

    let _mmap = unsafe { exit_boot_services(MemoryType::CONVENTIONAL) };

    kernel_main(framebuffer);

    Status::SUCCESS
}

fn kernel_main(fb: &mut [u8]) {
    loop {}
}
