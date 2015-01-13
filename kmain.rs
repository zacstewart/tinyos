#![crate_name = "kmain"]
#![crate_type = "dylib"]
#![no_std]

extern crate core;
use core::iter::*;
const COFFEE: [char; 7] = ['c', 'o', 'f', 'f', 'e', 'e', ' '];

enum Color {
    Black = 0,
    Red = 4,
    DarkGrey = 8,
    LightRed = 12,
    Blue = 1,
    Magenta = 5,
    LightBlue = 9,
    LightMagenta = 13,
    Green = 2,
    Brown = 6,
    LightGreen = 10,
    LightBrown = 14,
    Cyan = 3,
    LightGrey = 7,
    LightCyan = 11,
    White = 15
}

fn write_cell(position: usize, character: char, background: Color, foreground: Color) {
    unsafe {
        *((0xb8000 + position * 2) as *mut char) = character;
        *((0xb8000 + position * 2 + 1) as *mut u8) = (background as u8) << 4 | foreground as u8;
    }
}

#[no_mangle]
#[no_stack_check]
pub extern "C" fn kmain() {
    for i in range(0, 25 * 80) {
        write_cell(i, COFFEE[i % 7], Color::Brown, Color::Blue);
    }
}
