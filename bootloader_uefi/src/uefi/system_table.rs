use r_efi::efi;

use crate::uefi::*;

#[derive(Copy, Clone)]
pub struct SystemTableWrapper {
    system_table_ptr: *const efi::SystemTable,
}

impl SystemTableWrapper {
    /// Create a SystemTableWrapper to make the use of EFI SystemTable easier and safer
    /// 
    /// ## Safety
    /// 
    /// This is safe if the passed in *system_table_ptr* is the pointer provided into
    /// efi_main
    pub unsafe fn new(system_table_ptr: *const efi::SystemTable) -> SystemTableWrapper {
        return SystemTableWrapper {
            system_table_ptr: system_table_ptr
        };
    }

    pub fn boot_services(&self) -> BootServices {
        unsafe {
            return BootServices::new((*self.system_table_ptr).boot_services);
        }
    }

    pub fn con_out(&self) -> SimpleTextOutputProtocol {
        unsafe {
            return SimpleTextOutputProtocol::new((*self.system_table_ptr).con_out);
        }
    }

    pub fn get_configuration_table(&self) -> ConfigurationTable {
        unsafe {
            let num_entries = (*self.system_table_ptr).number_of_table_entries;
            let table_entries = (*self.system_table_ptr).configuration_table as *mut ConfigurationTableEntry;
            //This is safe because we've retrieved these values directly from the firmware
            return ConfigurationTable::new(num_entries, table_entries);
        }
    }
}