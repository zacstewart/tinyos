#![crate_name = "kmain"]
#![crate_type = "dylib"]
#![no_std]

extern crate core;
use core::iter::*;
const COFFEE: [char; 7] = ['c', 'o', 'f', 'f', 'e', 'e', ' '];

#[no_mangle]
#[no_stack_check]
pub extern "C" fn kmain() {
    for i in range(0, 25 * 80) {
        unsafe {
            *((0xb8000 + i * 2) as *mut char) = COFFEE[i % 7];
            *((0xb8000 + i * 2 + 1) as *mut u8) = 6 << 4 | 1;
        }
    }
}
