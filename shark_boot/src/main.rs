#![no_std]
#![no_main]

use core::fmt::Write;
use uefi::boot::wait_for_event;
use uefi::helpers::init;
use uefi::proto::console::text::{Color, Key, ScanCode};
use uefi::system::{with_stdin, with_stdout};
use uefi::{entry, Status};

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
