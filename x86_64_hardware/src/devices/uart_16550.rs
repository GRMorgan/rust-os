use crate::devices::ioport::Port;
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

pub struct SerialPort {
    data_reg : Port,
    inter_reg : Port,
    inter_ident_fifo_control_reg : Port,
    line_control_reg : Port,
    modem_control_reg : Port,
    line_status_reg : Port,
    #[allow(dead_code)]
    modem_status_reg : Port,
    #[allow(dead_code)]
    scratch_reg : Port,
}

impl SerialPort {
    ///This is unsafe as you can create an instance that doesn't actually point
    ///to a serial port. If a correct base port is specified the rest of the
    ///type is safe.
    pub unsafe fn new(base_port: u16) -> SerialPort {
        return SerialPort {
            data_reg : Port::new(base_port),
            inter_reg : Port::new(base_port + 1),
            inter_ident_fifo_control_reg : Port::new(base_port + 2),
            line_control_reg : Port::new(base_port + 3),
            modem_control_reg : Port::new(base_port + 4),
            line_status_reg : Port::new(base_port + 5),
            modem_status_reg : Port::new(base_port + 6),
            scratch_reg : Port::new(base_port + 7),
        };
    }

    pub fn initialise(&self) {
        unsafe {
            self.disable_interrupts();
            self.set_baud_divisor(0x1);
            self.line_control_reg.out_u8(0x3); // 8 bit mode, no parity, 1 stop bit
            self.inter_ident_fifo_control_reg.out_u8(0xC7);
            self.modem_control_reg.out_u8(0x0B);
        }
    }

    pub fn write_byte(&self, value : u8) {
        unsafe {
            while !self.is_transmit_empty() { } //Wait for transmission to clear
            self.data_reg.out_u8(value);
        }
    }

    unsafe fn set_baud_divisor(&self, divisor : u16) {
        let upper_divisor : u8 = ((divisor >> 8) & 0xFF) as u8;
        let lower_divisor : u8 = (divisor & 0xFF) as u8;
        self.set_dlab(true);
        self.data_reg.out_u8(lower_divisor);
        self.inter_reg.out_u8(upper_divisor);
        self.set_dlab(false);

    }

    unsafe fn disable_interrupts(&self) {
        self.inter_reg.out_u8(0x00);
    }

    unsafe fn set_dlab(&self, enable : bool) {
        let current_value = self.line_control_reg.in_u8();
        if enable {
            self.line_control_reg.out_u8(current_value | 0x80);
        } else {
            self.line_control_reg.out_u8(current_value & 0x7F);
        }
    }

    unsafe fn is_transmit_empty(&self) -> bool {
        return (self.line_status_reg.in_u8() & 0x20) != 0;
    }
}

impl fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
        Ok(())
    }
}

const COM1_BASE : u16 = 0x3F8;

pub fn com1_port() -> SerialPort {
    unsafe {
        let com1 = SerialPort::new(COM1_BASE);
        com1.initialise();
        return com1;
    }
}

lazy_static! {
    pub static ref COM1: Mutex<SerialPort> = Mutex::new(com1_port());
}

#[macro_export]
macro_rules! com1_print {
    ($($arg:tt)*) => ($crate::devices::uart_16550::_com1_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! com1_println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::com1_print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _com1_print(args: fmt::Arguments) {
    use core::fmt::Write;
    COM1.lock().write_fmt(args).unwrap();
}
