extern "C" {
    fn _outb(port: u16, data: u8);
    fn _inb(port: u16) -> u8;
}

pub fn outb(port: u16, data: u8) {
    unsafe { _outb(port, data) }
}

pub fn inb(port: u16) -> u8 {
    unsafe { _inb(port) }
}
