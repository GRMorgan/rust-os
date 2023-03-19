use crate::memory::VirtualAddress;
use core::mem::size_of;

#[derive(Clone, Copy)]
#[repr(C, align(8))]
pub struct GdtEntry(u64);

impl GdtEntry {
    ///Since this is for a strictly 64bit OS base and limit are always 0
    ///Only flags and access byte need be set
    pub const fn new(access_byte: u8, mut flags: u8) -> GdtEntry {
        flags &= 0xF; //Only the lower 4 bytes are relevant
        let mut value: u64 = (access_byte as u64) << 40;
        value |= (flags as u64) << 52;
        return GdtEntry(value);
    }

    pub const fn as_u64(&self) -> u64 {
        return self.0;
    }
}

#[repr(C, packed(2))]
pub struct GdtPointer {
    pub size: u16,
    pub addr: VirtualAddress
}

impl GdtPointer {
    pub fn new_from_array(table: &[GdtEntry]) -> GdtPointer {
        GdtPointer {
            addr: VirtualAddress::new(table.as_ptr() as u64),
            size: (size_of::<u64>() * table.len() - 1) as u16,
        }
    }
}

static DEFAULT_GDT: [GdtEntry;5] = [
    GdtEntry::new(0x00, 0x0), //NULL descriptor
    GdtEntry::new(0x9A, 0xA), //Kernel code
    GdtEntry::new(0x92, 0xA), //Kernel data
    GdtEntry::new(0xFA, 0xA), //User code
    GdtEntry::new(0xF2, 0xA), //User data
];

extern "C" { pub fn load_gdt(gdt_ptr: *const GdtPointer); }

pub fn init_default_gdt() {
    let gdt_pointer = GdtPointer::new_from_array(&DEFAULT_GDT);
    unsafe { load_gdt(&gdt_pointer); }
}
