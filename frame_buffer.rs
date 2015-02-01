use io;
use core::str::StrExt;

const FB_COMMAND_PORT: u16 = 0x3d4;
const FB_DATA_PORT: u16 = 0x3d5;
const FB_HIGH_BYTE_COMMAND: u8 = 14;
const FB_LOW_BYTE_COMMAND: u8 = 15;

pub enum Color {
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

pub fn write_cell(position: usize, character: u8, background: Color, foreground: Color) {
    unsafe {
        *((0xb8000 + position * 2) as *mut u8) = character;
        *((0xb8000 + position * 2 + 1) as *mut u8) = (background as u8) << 4 | foreground as u8;
    }
}

pub fn cursor_position() -> usize {
    let high: u8;
    let low: u8;

    unsafe {
        io::outb(FB_COMMAND_PORT, FB_HIGH_BYTE_COMMAND);
        high = io::inb(FB_DATA_PORT);
        io::outb(FB_COMMAND_PORT, FB_LOW_BYTE_COMMAND);
        low = io::inb(FB_DATA_PORT);
    }
    ((high as usize) << 8) | low as usize
}

pub fn move_cursor(position: usize) {
    unsafe {
        io::outb(FB_COMMAND_PORT, FB_HIGH_BYTE_COMMAND);
        io::outb(FB_DATA_PORT, ((position >> 8) & 0x0ff) as u8);
        io::outb(FB_COMMAND_PORT, FB_LOW_BYTE_COMMAND);
        io::outb(FB_DATA_PORT, (position & 0x0ff) as u8);
    }
}

pub fn write(text: &str, background: &Color, foreground: &Color) {
    let mut position = cursor_position();
    for b in StrExt::bytes(text) {
        write_cell(position, b, Color::Brown, Color::Blue);
        position += 1;
        move_cursor(position);
    }
}
