/// A struct that supports
/// 
/// ## Safety
/// Operations on this are fundamentally unsafe. A port can be specified which
/// does not exist or it can be used in an unsupported way. It is up to any code
/// that uses a port to ensure it is safe.
/// 
/// To use this safely the port needs to be controlled by a driver that understands
/// which ports it.
pub struct Port {
    port_number: u16,
}

impl Port {

    pub const unsafe fn new(port_number: u16) -> Port {
        return Port {
            port_number: port_number
        };
    }

    pub unsafe fn out_u8(&self, value: u8) {
        core::arch::asm!("out dx, al", in("dx") self.port_number, in("al") value, options(nomem, nostack, preserves_flags));
    }

    pub unsafe fn in_u8(&self) -> u8{
        let output: u8;
        core::arch::asm!("in al, dx", out("al") output, in("dx") self.port_number, options(nomem, nostack, preserves_flags));

        return output;
    }
}
