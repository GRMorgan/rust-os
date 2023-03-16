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

    pub fn con_out(&self) -> SimpleTextOutputProtocol {
        unsafe {
            return SimpleTextOutputProtocol::new((*self.system_table_ptr).con_out);
        }
    }
}