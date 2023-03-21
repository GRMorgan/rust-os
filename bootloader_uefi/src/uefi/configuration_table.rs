use r_efi::efi::Guid;

//eb9d2d30-2d88-11d3-9a16-0090273fc14d
pub const ACPI_V1_0_RSDP_GUID: Guid = Guid::from_fields(
    0xeb9d2d30,
    0x2d88,
    0x11d3,
    0x9a,
    0x16,
    &[0x00, 0x90, 0x27, 0x3f, 0xc1, 0x4d],
);

//8868e871-e4f1-11d3-bc22-0080c73c8881
pub const ACPI_V2_0_RSDP_GUID: Guid = Guid::from_fields(
    0x8868e871,
    0xe4f1,
    0x11d3,
    0xbc,
    0x22,
    &[0x00, 0x80, 0xc7, 0x3c, 0x88, 0x81],
);

pub struct ConfigurationTable {
    num_entries: usize,
    configuration_table: *mut ConfigurationTableEntry,
}

impl ConfigurationTable {
    /// Create a new UEFI ConfigurationTable
    /// 
    /// ## Safety
    /// 
    /// This is unsafe as the num_entries might be too large for the memory block pointed to by the
    /// configuration_table pointer. The caller must ensure the size is correct before calling this
    /// function
    pub unsafe fn new(num_entries: usize, configuration_table: *mut ConfigurationTableEntry) -> ConfigurationTable {
        ConfigurationTable {
            num_entries: num_entries,
            configuration_table: configuration_table
        }
    }

    pub fn get_entry(&self, index: usize) -> Option<ConfigurationTableEntry> {
        if index >= self.num_entries {
            return None;
        }

        //This is safe because we've checked the index isn't beyond the end of the list
        //configuration_table is guaranteed to be valid by the constructor
        unsafe { return Some(*(self.configuration_table.offset(index as isize))); }
    }

    pub fn iter(&self) -> ConfigurationTableIterator {
        ConfigurationTableIterator {
            asset_list: self,
            current_index: 0,
            max_index: self.num_entries
        }
    }
}

#[derive(Clone, Copy)]
pub struct ConfigurationTableEntry {
    pub vendor_guid: r_efi::efi::Guid,
    pub vendor_table: *mut core::ffi::c_void,
}

pub struct ConfigurationTableIterator<'a> {
    asset_list: &'a ConfigurationTable,
    current_index: usize,
    max_index: usize,
}

impl<'a> Iterator for ConfigurationTableIterator<'a> {
    type Item = ConfigurationTableEntry;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index == self.max_index {
            return None;
        } else {
            let output = self.asset_list.get_entry(self.current_index);
            if output.is_some() {
                self.current_index += 1;
            }
            return output;
        }
    }
}