#![crate_name = "kmain"]
#![crate_type = "dylib"]
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
    let ref bg = frame_buffer::Color::Brown;
    let ref fg = frame_buffer::Color::Blue;
    for _ in (0..285) {
        frame_buffer::write("coffee ", bg, fg);
    }

    // Serial logging
    let port = serial::Port::new();
    for _ in (0..1000) {
        port.write("coffee ");
    }
}
