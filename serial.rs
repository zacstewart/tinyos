use io;

const COM1_BASE: u16 = 0x3f8;
const LINE_ENABLE_DLAB: u8 = 0x80;

fn fifo_command_port(base: u16) -> u16 {
    base + 2
}

fn line_command_port(base: u16) -> u16 {
    base + 3
}

fn modem_command_port(base: u16) -> u16 {
    base + 4
}

fn line_status_port(base: u16) -> u16 {
    base + 5
}

pub struct Port {
    base: u16
}

impl Port {
    pub fn new() -> Port {
        let port = Port {
            base: COM1_BASE
        };

        port.configure_baud_rate(1);
        port.configure_line();
        port.configure_fifo_command_port();
        port.configure_modem_command_port();

        port
    }

    /// Sets the speed of the data being sent. Default speed is 115200 bits/s.
    /// Divisor will be used to set the speed to (115200 / divisor) bits/s.
    fn configure_baud_rate(&self, divisor: u16) {
        let high: u8 = ((divisor >> 8) & 0x00ff) as u8;
        let low: u8 = (divisor & 0x00ff) as u8;
        io::outb(line_command_port(self.base), LINE_ENABLE_DLAB);
        io::outb(self.base, high);
        io::outb(self.base, low);
    }

    /// Configures the line of the given serial port. Data length of 8 bits, no
    /// parity bits, one stop bit, DLAB disabled, and break control disabled.
    fn configure_line(&self) {
        // bit:     | 7 | 6 | 5 4 3 | 2 | 1 0 |
        // content: | d | b | prty  | s | dln |
        // value:   | 0 | 0 | 0 0 0 | 0 | 1 1 |
        io::outb(line_command_port(self.base), 0x03);
    }

    /// Configures the fifo command port to buffer 14 bytes.
    fn configure_fifo_command_port(&self) {
        // bit:     | 7 6 | 5  | 4 | 3   | 2   | 1   | 0 |
        // content: | lvl | bs | r | dma | clt | clr | e |
        // value:   | 1 1 | 0  | 0 | 0   | 1   | 1   | 1 |
        io::outb(fifo_command_port(self.base), 0xc7);
    }

    fn configure_modem_command_port(&self) {
        // bit:     | 7 | 6 | 5  | 4  | 3   | 2   | 1   | 0   |
        // content: | r | r | af | lb | ao2 | ao1 | rts | dtr |
        // value:   | 0 | 0 | 0  | 0  | 0   | 0   | 1   | 1   |
        io::outb(modem_command_port(self.base), 0x03);
    }

    fn transmitting_fifo_empty(&self) -> bool {
        let status = io::inb(line_status_port(self.base));
        (status & 0x20) == 0x20
    }

    /// Write a string to the serial port
    pub fn write(&self, text: &str) {

        for b in text.bytes() {
            loop {
                match self.transmitting_fifo_empty() {
                    true => break,
                    false => {}
                }
            }

            io::outb(self.base, b)
        }
    }
}
