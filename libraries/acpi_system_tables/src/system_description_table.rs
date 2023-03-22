#[repr(C)]
pub struct SystemDescriptionTableHeader {
    signature: [u8;4],
    length: u32,
    revision: u8,
    checksum: u8,
    oem_id: [u8;6],
    oem_table_id: [u8;8],
    oem_revision: u32,
    creator_id: u32,
    creator_revision: u32,
}

impl SystemDescriptionTableHeader {
    pub fn length(&self) -> u32 {
        return self.length;
    }
}

pub struct SystemDescriptionTable {
    sdt_ptr: *mut SystemDescriptionTableHeader,
    mem_offset: u64,
}

impl SystemDescriptionTable {
    /// Creates a new SystemDescriptionTable that wraps an SystemDescriptionTableHeader pointer. We
    /// use a wrapper because the data can extend off beyond the last entry in the internal struct.
    /// So if we want to implement Copy for this it must contain the pointer internally.
    /// 
    /// Additionally we don't know precisely what kind of SDT this is. Potentially we can allow for
    /// a conversion to a more accurate vendor specific SDT here.
    /// 
    /// ## Safety
    /// 
    /// This is unsafe because we cannot know the address and offset are valid. The address
    /// must come from the RSDT and the offset from the virtual memory management logic of
    /// the OS
    pub unsafe fn new(physical_address: u64, offset: u64) -> SystemDescriptionTable {
        let virtual_address = physical_address + offset;

        return SystemDescriptionTable { 
            sdt_ptr: virtual_address as *mut SystemDescriptionTableHeader,
            mem_offset: offset,
        }
    }

    pub fn get_signature(&self) -> [u8;4] {
        return unsafe { (*self.sdt_ptr).signature };
    }
}