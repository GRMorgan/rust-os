use r_efi::efi;
use r_efi::protocols::simple_file_system;
use r_efi::protocols::file;

use crate::uefi::*;

pub struct SimpleFileSystemProtocol {
    file_system_ptr: *mut simple_file_system::Protocol,
}

impl SimpleFileSystemProtocol {
    pub fn new(file_system_ptr: *mut simple_file_system::Protocol) -> SimpleFileSystemProtocol {
        return SimpleFileSystemProtocol {
            file_system_ptr: file_system_ptr
        };
    }

    pub fn open_volume(&self) -> Result<FileProtocol, efi::Status> {
        let mut volume: *mut file::Protocol = core::ptr::null_mut::<file::Protocol>();
        let s = unsafe {
            ((*self.file_system_ptr).open_volume)(self.file_system_ptr, &mut volume)
        };

        if s == efi::Status::SUCCESS {
            return Ok(FileProtocol::new(volume));
        } else {
            return Err(s);
        }
    }
}