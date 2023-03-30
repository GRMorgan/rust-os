pub const PAGE_SIZE: u64 = 4096;
pub const PHYSICAL_ADDRESS_MASK: u64 = 0x000FFFFFFFFFF000;

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
pub struct PhysicalAddress(u64);

impl PhysicalAddress {
    #[inline]
    pub const fn new(addr: u64) -> PhysicalAddress {
        return PhysicalAddress(addr & PHYSICAL_ADDRESS_MASK);
    }

    #[inline]
    pub const fn as_u64(self) -> u64 {
        self.0
    }

    #[inline]
    pub const fn as_usize(self) -> usize {
        self.0 as usize
    }

    #[inline]
    pub const fn increment_page_4kb(self, num_pages: u64) -> PhysicalAddress {
        return PhysicalAddress(self.as_u64() + (num_pages * PAGE_SIZE));
    }

    pub fn get_virtual_address_at_offset(&self, offset: u64) -> VirtualAddress {
        return VirtualAddress::new(self.0 + offset);
    }
}

impl PartialOrd for PhysicalAddress {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        return self.0.partial_cmp(&other.0);
    }
}


pub const VIRTUAL_ADDRESS_MASK: u64 = 0x0000_FFFF_FFFF_FFFF;
pub const VIRTUAL_ADDRESS_SIGN_EXTENSION_MASK: u64 = 0xFFFF_0000_0000_0000;
pub const MAX_VIRTUAL_ADDRESS: u64 = 0xFFFF_FFFF_FFFF_FFFF;
pub const PAGE_OFFSET_MASK: u64 = 0xFFFu64;
pub const VIRTUAL_ADDRESS_HIGH_BIT: u64 = 1 << 47;

const PAGE_TABLE_INDEX_MASK: u64 = 0x1FFu64;
const P1_OFFSET: u64 = 12;
const P1_INDEX_MASK: u64 = PAGE_TABLE_INDEX_MASK << P1_OFFSET;
const P2_OFFSET: u64 = P1_OFFSET + 9;
const P2_INDEX_MASK: u64 = PAGE_TABLE_INDEX_MASK << P2_OFFSET;
const P3_OFFSET: u64 = P2_OFFSET + 9;
const P3_INDEX_MASK: u64 = PAGE_TABLE_INDEX_MASK << P3_OFFSET;
const P4_OFFSET: u64 = P3_OFFSET + 9;
const P4_INDEX_MASK: u64 = PAGE_TABLE_INDEX_MASK << P4_OFFSET;

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
pub struct VirtualAddress(u64);

impl VirtualAddress {
    #[inline]
    pub const fn new(mut addr: u64) -> VirtualAddress {
        addr &= VIRTUAL_ADDRESS_MASK;

        if addr & VIRTUAL_ADDRESS_HIGH_BIT != 0 {
            addr |= VIRTUAL_ADDRESS_SIGN_EXTENSION_MASK;
        } 
        return VirtualAddress(addr);
    }

    #[inline]
    pub fn new_from_page_table_indexes(p4: u64, p3: u64, p2: u64, p1: u64, offset: u64) -> VirtualAddress {
        let mut addr = offset;
        addr |= (p4 & PAGE_TABLE_INDEX_MASK) << P4_OFFSET;
        addr |= (p3 & PAGE_TABLE_INDEX_MASK) << P3_OFFSET;
        addr |= (p2 & PAGE_TABLE_INDEX_MASK) << P2_OFFSET;
        addr |= (p1 & PAGE_TABLE_INDEX_MASK) << P1_OFFSET;
        return VirtualAddress::new(addr);
    }

    #[inline]
    pub const fn as_u64(self) -> u64 {
        self.0
    }

    #[inline]
    pub const fn as_usize(self) -> usize {
        self.0 as usize
    }

    #[inline]
    pub const fn increment_page_4kb(self, num_pages: u64) -> VirtualAddress {
        return VirtualAddress(self.as_u64() + (num_pages * PAGE_SIZE));
    }

    #[inline]
    pub const fn page_offset(self) -> u64 {
        return self.0 & PAGE_OFFSET_MASK;
    }

    #[inline]
    pub const fn p1_index(self) -> usize {
        return ((self.0 & P1_INDEX_MASK) >> P1_OFFSET) as usize;
    }

    #[inline]
    pub const fn p2_index(self) -> usize {
        return ((self.0 & P2_INDEX_MASK) >> P2_OFFSET) as usize;
    }

    #[inline]
    pub const fn p3_index(self) -> usize {
        return ((self.0 & P3_INDEX_MASK) >> P3_OFFSET) as usize;
    }

    #[inline]
    pub const fn p4_index(self) -> usize {
        return ((self.0 & P4_INDEX_MASK) >> P4_OFFSET) as usize;
    }

    #[inline]
    pub const unsafe fn get_mut_ptr<T>(self) ->  *mut T {
        return self.0 as *mut T;
    }
}

impl PartialOrd for VirtualAddress {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        return self.0.partial_cmp(&other.0);
    }
}
