#![crate_name = "kmain"]
#![crate_type = "dylib"]
#![no_std]

extern crate core;
use core::iter::*;

mod io;
mod frame_buffer;

const COFFEE: [char; 7] = ['c', 'o', 'f', 'f', 'e', 'e', ' '];

#[no_mangle]
#[no_stack_check]
pub extern "C" fn kmain() {
    frame_buffer::move_cursor(80);
    for i in range(0, 25 * 80) {
        frame_buffer::write_cell(i, COFFEE[i % 7], frame_buffer::Color::Brown, frame_buffer::Color::Blue);
    }
}
