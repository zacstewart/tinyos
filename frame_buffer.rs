use io;

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

pub fn write_cell(position: usize, character: char, background: Color, foreground: Color) {
    unsafe {
        *((0xb8000 + position * 2) as *mut char) = character;
        *((0xb8000 + position * 2 + 1) as *mut u8) = (background as u8) << 4 | foreground as u8;
    }
}

pub fn move_cursor(position: u16) {
    unsafe {
        io::outb(0x3d4, 14);
        io::outb(0x3d5, ((position >> 8) & 0x0ff) as u8);
        io::outb(0x3d4, 15);
        io::outb(0x3d5, (position & 0x0ff) as u8);
    }
}
