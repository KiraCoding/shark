#![no_std]
#![no_main]

extern crate alloc;

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::Write;
use core::slice::from_raw_parts;
use uefi::boot::wait_for_event;
use uefi::helpers::init;
use uefi::proto::console::text::{Color, Key, ScanCode};
use uefi::runtime::{get_variable_boxed, VariableVendor};
use uefi::system::{with_stdin, with_stdout};
use uefi::{cstr16, entry, CStr16, Status};

#[entry]
fn main() -> Status {
    init().unwrap();

    with_stdout(|output| {
        output.clear().unwrap();

        {
            output.set_color(Color::Yellow, Color::LightGray).unwrap();
            let text = concat!("SHARK BOOT v", env!("CARGO_PKG_VERSION"));
            let width = output.current_mode().unwrap().unwrap().columns();
            writeln!(output, "{:^width$}", text, width = width).unwrap();
            output.set_color(Color::White, Color::Black).unwrap();
        }

        {
            let (data, _attrs) =
                get_variable_boxed(cstr16!("BootOrder"), &VariableVendor::GLOBAL_VARIABLE).unwrap();

            let boot_order: &[u16] =
                unsafe { from_raw_parts(data.as_ptr() as *const u16, data.len() / 2) };

            for &index in boot_order {
                let boot_entry_name = format!("Boot{:04}", index);
                let mut u16_name: Vec<u16> = boot_entry_name.encode_utf16().collect();
                u16_name.push(0);
                let boot_entry_var = CStr16::from_u16_with_nul(&u16_name).unwrap();

                let (data, _attrs) =
                    get_variable_boxed(boot_entry_var, &VariableVendor::GLOBAL_VARIABLE).unwrap();
            }
        }
    });

    with_stdin(|input| loop {
        wait_for_event(&mut [input.wait_for_key_event().unwrap()]).unwrap();

        match input.read_key().unwrap() {
            Some(Key::Special(ScanCode::UP)) => {}
            Some(Key::Special(ScanCode::DOWN)) => {}
            Some(Key::Special(ScanCode::ESCAPE)) => break,
            _ => (),
        }
    });

    Status::SUCCESS
}
