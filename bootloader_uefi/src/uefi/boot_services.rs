use r_efi::efi;
use r_efi::protocols::*;

use crate::uefi::*;

pub struct BootServices {
    boot_services_ptr: *mut efi::BootServices,
}

impl BootServices {
    pub fn new(boot_services_ptr: *mut efi::BootServices) -> BootServices {
        return BootServices {
            boot_services_ptr: boot_services_ptr
        };
    }

    pub fn allocate_pages<T>(&self, mem_type: r_efi::system::MemoryType, pages: usize) -> Result<*mut T, efi::Status> {
        let mut address: u64 = 0;
        let status = self.allocate_pages_raw(r_efi::system::ALLOCATE_ANY_PAGES, mem_type, pages, &mut address);
        if status != efi::Status::SUCCESS {
            return Err(status);
        } else {
            return Ok(address as *mut T);
        }
    }

    pub fn allocate_pages_raw(&self, alloc_type: r_efi::system::AllocateType, mem_type: r_efi::system::MemoryType, pages: usize, memory: *mut r_efi::base::PhysicalAddress) -> efi::Status {
        unsafe {
            ((*self.boot_services_ptr).allocate_pages)(alloc_type, mem_type, pages, memory)
        }
    }

    pub fn open_volume(&self, h: efi::Handle) -> Result<FileProtocol, efi::Status> {
        let loaded_image = self.get_loaded_image_protocol(h)?;
        let file_system = self.get_simple_file_system_protocol(loaded_image.device_handle())?;
        return file_system.open_volume();
    }

    pub fn get_loaded_image_protocol(&self, h: efi::Handle) -> Result<LoadedImageProtocol, efi::Status> {
        let mut guid: efi::Guid = loaded_image::PROTOCOL_GUID;
        let fs_ptr: *mut loaded_image::Protocol = self.handle_protocol(h, &mut guid)? as *mut loaded_image::Protocol;
        return Ok(LoadedImageProtocol::new(fs_ptr));
    }

    pub fn get_simple_file_system_protocol(&self, h: efi::Handle) -> Result<SimpleFileSystemProtocol, efi::Status> {
        let mut guid: efi::Guid = simple_file_system::PROTOCOL_GUID;
        let fs_ptr: *mut simple_file_system::Protocol = self.handle_protocol(h, &mut guid)? as *mut simple_file_system::Protocol;
        return Ok(SimpleFileSystemProtocol::new(fs_ptr));
    }

    fn handle_protocol(&self, h: efi::Handle, guid: *mut efi::Guid) -> Result<*mut core::ffi::c_void, efi::Status> {
        let mut output: *mut core::ffi::c_void = core::ptr::null_mut::<core::ffi::c_void>();
        let s = unsafe {
            ((*self.boot_services_ptr).handle_protocol)(h, guid, &mut output)
        };

        if s == efi::Status::SUCCESS {
            return Ok(output);
        } else {
            return Err(s);
        }
    }
}