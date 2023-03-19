use crate::memory::*;

const PRESENT_FLAG: u64 = 1 << 0;
const READ_WRITE_FLAG: u64 = 1 << 1;
const _USER_SUPERVISOR_FLAG: u64 = 1 << 2;
const PAGE_SIZE_FLAG: u64 = 1 << 7;
const _EXECUTE_DISABLE_FLAG: u64 = 1 << 63;


#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct PageTableEntry {
    entry : u64,
}

impl PageTableEntry {
    #[inline]
    pub fn is_unused(&self) -> bool {
        return self.entry == 0;
    }

    #[inline]
    pub fn make_unused(&mut self) {
        self.entry = 0;
    }

    #[inline]
    pub fn present(&self) -> bool {
        return self.flags_active(PRESENT_FLAG);
    }

    #[inline]
    pub fn set_present(&mut self, value: bool) {
        self.set_flags(PRESENT_FLAG, value);
    }

    #[inline]
    pub fn read_write(&self) -> bool {
        return self.flags_active(READ_WRITE_FLAG);
    }

    #[inline]
    pub fn set_read_write(&mut self, value: bool) {
        self.set_flags(READ_WRITE_FLAG, value);
    }

    #[inline]
    pub fn page_size(&self) -> bool {
        return self.flags_active(PAGE_SIZE_FLAG);
    }

    #[inline]
    pub fn set_page_size(&mut self, value: bool) {
        self.set_flags(PAGE_SIZE_FLAG, value);
    }

    #[inline]
    pub fn address(&self) -> PhysicalAddress {
        return PhysicalAddress::new(self.entry & PHYSICAL_ADDRESS_MASK);
    }

    #[inline]
    pub fn set_address(&mut self, addr: PhysicalAddress) {
        self.entry = (self.entry & !PHYSICAL_ADDRESS_MASK) | addr.as_u64();
    }

    #[inline]
    fn flags_active(&self, flags: u64) -> bool{
        return (self.entry & flags) == flags;
    }

    #[inline]
    fn set_flags(&mut self, flags: u64, value: bool) {
        if value {
            self.entry |= flags;
        } else {
            self.entry &= !flags;
        }
    }
}

impl Default for PageTableEntry {
    fn default() -> PageTableEntry {
        PageTableEntry {
            entry: 0,
        }
    }
}

#[repr(align(4096))]
#[repr(C)]
pub struct PageTable {
    pub table: [PageTableEntry; 512],
}

impl PageTable {
    pub fn make_unused(&mut self) {
        for i in 0..self.table.len() {
            self.table[i].make_unused();
        }
    }

    pub fn copy_from(&mut self, other_page_table: &PageTable) {
        self.table = other_page_table.table;
    }
}


impl Default for PageTable {
    fn default() -> PageTable {
        let blank_entry = PageTableEntry::default();
        PageTable {
            table: [blank_entry;512],
        }
    }
}
