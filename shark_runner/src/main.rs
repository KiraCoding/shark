use std::fs::{copy, create_dir_all};
use std::io::Result;
use std::path::Path;
use std::process::{exit, Command, ExitStatus};

fn main() -> Result<()> {
    run_build()?;
    run_qemu()?;

    Ok(())
}

fn run_build() -> Result<ExitStatus> {
    let status = Command::new("cargo")
        .arg("+nightly")
        .arg("build")
        .arg("-p")
        .arg("shark_boot")
        .arg("--config")
        .arg("shark_boot/.cargo/config.toml")
        .status()?;

    if !status.success() {
        exit(status.code().unwrap_or(1)); // Exit with the same code as cargo
    }

    Ok(status)
}

fn run_qemu() -> Result<ExitStatus> {
    let esp_directory = "target/qemu/esp/efi/boot";
    let shark_efi_source = "target/x86_64-unknown-uefi/debug/shark_boot.efi";
    let shark_efi_destination = format!("{}/bootx64.efi", esp_directory);

    let ovmf_code_path = "tools/ovmf/code.fd";
    let ovmf_vars_path = "tools/ovmf/vars.fd";

    let esp_path = Path::new(esp_directory);
    if !esp_path.exists() {
        create_dir_all(esp_path)?;
    }

    copy(shark_efi_source, &shark_efi_destination)?;

    Command::new("qemu-system-x86_64")
        .arg("-drive")
        .arg(format!("format=raw,file=fat:rw:target/qemu/esp"))
        .arg("-drive")
        .arg(format!(
            "if=pflash,format=raw,readonly=on,file={}",
            ovmf_code_path
        ))
        .arg("-drive")
        .arg(format!(
            "if=pflash,format=raw,readonly=on,file={}",
            ovmf_vars_path
        ))
        .status()
}
