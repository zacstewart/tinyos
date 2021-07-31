#![crate_name = "kmain"]
#![crate_type = "dylib"]
#![no_std]

use core::iter::*;

mod io;
mod frame_buffer;
mod serial;

#[no_mangle]
pub extern "C" fn kmain() {
    frame_buffer::move_cursor(0);
    for _ in (0..285) {
        frame_buffer::write(
            "coffee ",
            frame_buffer::Color::Green,
            frame_buffer::Color::Blue
        );
    }

    // Serial logging
    let port = serial::Port::new();
    for _ in (0..1000) {
        port.write("coffee ");
    }
}
