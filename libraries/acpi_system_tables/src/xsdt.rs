use crate::{SystemDescriptionTableHeader, SystemDescriptionTable};


#[repr(C)]
pub struct ExtendedSystemDescriptionTableInternal {
    header: SystemDescriptionTableHeader,
    first_entry: u64,
}

pub struct ExtendedSystemDescriptionTable {
    xsdt_ptr: *mut ExtendedSystemDescriptionTableInternal,
    mem_offset: u64,
}

impl ExtendedSystemDescriptionTable {
    /// Creates a new RootSystemDescriptionTable that wraps an RSDT pointer. We use a wrapper
    /// because the data can extend off beyond the last entry in the internal struct. So if
    /// we want to implement Copy for this it must contain the pointer internally.
    /// 
    /// ## Safety
    /// 
    /// This is unsafe because we cannot know the address and offset are valid. The address
    /// must come from the RSDP and the offset from the virtual memory management logic of
    /// the OS
    pub unsafe fn new(physical_address: u64, offset: u64) -> ExtendedSystemDescriptionTable {
        let virtual_address = physical_address + offset;

        return ExtendedSystemDescriptionTable { 
            xsdt_ptr: virtual_address as *mut ExtendedSystemDescriptionTableInternal,
            mem_offset: offset,
        }
    }

    pub fn num_entries(&self) -> usize {
        let table_len = unsafe { (*self.xsdt_ptr).header.length() as usize};
        let size_of_entries = table_len - core::mem::size_of::<SystemDescriptionTableHeader>();
        return size_of_entries / core::mem::size_of::<u64>();
    }

    pub fn get_entry(&self, index: usize) -> Option<SystemDescriptionTable> {
        if index >= self.num_entries() {
            return None;
        }

        let xsdt_ptr_u8 = self.xsdt_ptr as *mut u8;

        let entry_ptr = unsafe { xsdt_ptr_u8.offset(core::mem::size_of::<SystemDescriptionTableHeader>() as isize) } as *mut u64;

        let entry = unsafe { *(entry_ptr.offset(index as isize)) };

        return unsafe { Some(SystemDescriptionTable::new(entry as u64, self.mem_offset)) };
    }

    pub fn iter(&self) -> ExtendedSystemDescriptionTableIterator {
        ExtendedSystemDescriptionTableIterator {
            xsdt: self,
            current_index: 0,
            max_index: self.num_entries(),
        }
    }
}

pub struct ExtendedSystemDescriptionTableIterator<'a> {
    xsdt: &'a ExtendedSystemDescriptionTable,
    current_index: usize,
    max_index: usize,
}

impl<'a> Iterator for ExtendedSystemDescriptionTableIterator<'a> {
    type Item = SystemDescriptionTable;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index == self.max_index {
            return None;
        } else {
            let output = self.xsdt.get_entry(self.current_index);
            if output.is_some() {
                self.current_index += 1;
            }
            return output;
        }
    }
}