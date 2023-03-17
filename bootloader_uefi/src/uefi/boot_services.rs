use r_efi::efi;
use r_efi::protocols::*;
use x86_64_hardware::memory::PAGE_SIZE;

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

    fn allocate_pages_raw(&self, alloc_type: r_efi::system::AllocateType, mem_type: r_efi::system::MemoryType, pages: usize, memory: *mut r_efi::base::PhysicalAddress) -> efi::Status {
        unsafe {
            ((*self.boot_services_ptr).allocate_pages)(alloc_type, mem_type, pages, memory)
        }
    }

    fn free_pages<T>(&self, mem: *mut T, num_pages: usize) -> Result<(), efi::Status> {
        return self.free_pages_raw(mem as u64, num_pages);
    }

    fn free_pages_raw(&self, mem_addr: u64, num_pages: usize) -> Result<(), efi::Status> {
        let s = unsafe {
            ((*self.boot_services_ptr).free_pages)(mem_addr, num_pages)
        };

        if s != efi::Status::SUCCESS {
            return Err(s);
        } else {
            return Ok(());
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

    pub fn get_memory_map(&self) -> Result<memory_map::GetMemoryMapOutput, efi::Status> {
        let mut output: memory_map::GetMemoryMapOutput = Default::default();
        let mut mem_ptr = core::ptr::null_mut::<core::ffi::c_void>();
        let mut s = unsafe {
            ((*self.boot_services_ptr).get_memory_map)(
                &mut output.map.map_size,
                mem_ptr as *mut efi::MemoryDescriptor,
                &mut output.map_key,
                &mut output.map.descriptor_size,
                &mut output.descriptor_version
            )
        };

        //Loop here to get correct mem map size
        //The act of allocating can change the size of the mem map. So we need to loop
        //until we have a memory block large enough to fit the map.
        while s == efi::Status::BUFFER_TOO_SMALL {
            let num_pages = (output.map.map_size + PAGE_SIZE as usize - 1) / PAGE_SIZE as usize;
            mem_ptr = self.allocate_pages::<core::ffi::c_void>(efi::LOADER_DATA, num_pages)?;

            output.map.descriptors = mem_ptr as *mut memory_map::EfiMemoryDescriptor;
            s = unsafe {
                ((*self.boot_services_ptr).get_memory_map)(
                    &mut output.map.map_size,
                    mem_ptr as *mut efi::MemoryDescriptor,
                    &mut output.map_key,
                    &mut output.map.descriptor_size,
                    &mut output.descriptor_version
                )
            };
            
            if s != efi::Status::SUCCESS {
                self.free_pages(mem_ptr, num_pages)?;
            } else {
                output.map.num_pages = num_pages;
            }
        }

        output.map.descriptors = mem_ptr as *mut memory_map::EfiMemoryDescriptor;

        return Ok(output);
    }

    pub fn exit_boot_services(&self, h: efi::Handle, map_key: usize) -> Result<(),efi::Status> {
        let s = unsafe {
            ((*self.boot_services_ptr).exit_boot_services)(h, map_key)
        };

        if s == efi::Status::SUCCESS {
            return Ok(());
        } else {
            return Err(s);
        }
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