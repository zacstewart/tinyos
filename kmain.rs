#![crate_name = "kmain"]
#![crate_type = "dylib"]
#![no_std]

extern crate core;
use core::iter::*;

mod io;
mod frame_buffer;

#[no_mangle]
#[no_stack_check]
pub extern "C" fn kmain() {
    frame_buffer::move_cursor(0);
    let ref bg = frame_buffer::Color::Brown;
    let ref fg = frame_buffer::Color::Blue;
    for i in range(0, 285) {
        frame_buffer::write("coffee ", bg, fg);
    }
}
