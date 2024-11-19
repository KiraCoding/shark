#![no_std]
#![no_main]

#[uefi::entry]
#[cfg(target_os = "uefi")]
fn efi_main() -> uefi::Status {
    use uefi::boot::{exit_boot_services, MemoryType};
    use uefi::mem::memory_map::{MemoryMap, MemoryMapMut};

    let _mmap = unsafe { exit_boot_services(MemoryType::CONVENTIONAL) };

    kernel_main();

    Status::SUCCESS
}

fn kernel_main() {
    loop {}
}

struct MemoryMapInfo {
    size: usize,
    descriptor_size: usize,
    descriptor_version: usize,
}

struct KernelArgs {}