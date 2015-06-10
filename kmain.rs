#![crate_name = "kmain"]
#![crate_type = "dylib"]
#![feature(no_std, core)]
#![no_std]

extern crate core;
use core::iter::*;

mod io;
mod frame_buffer;
mod serial;

#[no_mangle]
#[no_stack_check]
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
