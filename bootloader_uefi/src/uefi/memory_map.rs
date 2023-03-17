use x86_64_hardware::memory::{PhysicalAddress, PAGE_SIZE};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct EfiMemoryDescriptor {
    pub mem_type: u32,
    pub phys_addr: PhysicalAddress,
    pub virt_addr: *mut core::ffi::c_void,
    pub num_pages: u64,
    pub attribs: u64,
}

#[derive(PartialEq, Debug)]
pub enum DescriptorType {
    EfiReservedMemoryType,      //Restricted by firmware
    EfiLoaderCode,              //Used but reclaimable memory
    EfiLoaderData,              //Used but reclaimable memory
    EfiBootServicesCode,        //Used but reclaimable memory
    EfiBootServicesData,        //Used but reclaimable memory
    EfiRuntimeServicesCode,     //Restricted by firmware
    EfiRuntimeServicesData,     //Restricted by firmware
    EfiConventionalMemory,      //Unused memory
    EfiUnusableMemory,          //Restricted by firmware
    EfiACPIReclaimMemory,       //Reclaimable after retrieving ACPI tables
    EfiACPIMemoryNVS,           //Restricted by firmware. Must be restored after a sleep
    EfiMemoryMappedIO,          //Restricted by firmware
    EfiMemoryMappedIOPortSpace, //Restricted by firmware
    EfiPalCode,                 //Restricted by firmware
    EfiPersistentMemory,        //Byte addressable non-volatile memory. Usable by OS
    EfiUnknown,
}

impl EfiMemoryDescriptor {
    
    pub fn num_bytes(&self) -> u64 {
        return self.num_pages * PAGE_SIZE;
    }

    ///Returns the base address of the page following this range
    pub fn max_physical_address(&self) -> PhysicalAddress {
        return self.phys_addr.increment_page(self.num_pages);
    }

    //True if the memory is potentially usable (i.e. isn't reserved, IO mapped or similar)
    fn is_usable_memory(&self) -> bool {
        if self.mem_type() == DescriptorType::EfiConventionalMemory {
            return true;
        }
        return false;
    }

    pub fn mem_type(&self) -> DescriptorType {
        match self.mem_type {
             0 => { return DescriptorType::EfiReservedMemoryType; },
             1 => { return DescriptorType::EfiLoaderCode; },
             2 => { return DescriptorType::EfiLoaderData; },
             3 => { return DescriptorType::EfiBootServicesCode; },
             4 => { return DescriptorType::EfiBootServicesData; },
             5 => { return DescriptorType::EfiRuntimeServicesCode; },
             6 => { return DescriptorType::EfiRuntimeServicesData; },
             7 => { return DescriptorType::EfiConventionalMemory; },
             8 => { return DescriptorType::EfiUnusableMemory; },
             9 => { return DescriptorType::EfiACPIReclaimMemory; },
            10 => { return DescriptorType::EfiACPIMemoryNVS; },
            11 => { return DescriptorType::EfiMemoryMappedIO; },
            12 => { return DescriptorType::EfiMemoryMappedIOPortSpace; },
            13 => { return DescriptorType::EfiPalCode; },
            14 => { return DescriptorType::EfiPersistentMemory; },
            _ => { return DescriptorType::EfiUnknown; },
        }
    }
}

#[repr(C)]
pub struct EfiMemoryMap {
    pub descriptors: *mut EfiMemoryDescriptor,
    pub map_size: usize,
    pub descriptor_size: usize,
    pub num_pages: usize,
}



impl EfiMemoryMap {
    pub fn get_descriptor(&self, index: usize) -> Result<EfiMemoryDescriptor, ()> {
        if index >= self.num_entries() {
            return Err(());
        }

        let raw_ptr: *mut u8 = self.descriptors as *mut u8;
        unsafe {
            return Ok(*(raw_ptr.offset(self.descriptor_size as isize * index as isize) as *mut EfiMemoryDescriptor));
        }
    }

    pub fn iter(&self) -> EfiMemoryMapIterator {
        return EfiMemoryMapIterator {
            mem_map: self,
            current_index: 0,
            max_index: self.num_entries(),
        }
    }

    pub fn num_entries(&self) -> usize {
        return self.map_size / self.descriptor_size;
    }

    pub fn get_usable_memory_size_pages(&self) -> u64 {
        return self.max_usable_physical_address().as_u64() / PAGE_SIZE;
    }

    pub fn get_usable_memory_size_bytes(&self) -> u64 {
        return self.get_usable_memory_size_pages() * PAGE_SIZE;
    }

    ///Returns the base address of the page following the highest memory in the map
    pub fn max_usable_physical_address(&self) -> PhysicalAddress {
        let mut highest_address = PhysicalAddress::new(0);

        for descriptor in self.iter() {
            if descriptor.is_usable_memory() && descriptor.max_physical_address() > highest_address {
                highest_address = descriptor.max_physical_address();
            }
        }
        return highest_address;
    }

    ///Returns the base address of the page following the highest memory in the map
    pub fn max_physical_address(&self) -> PhysicalAddress {
        let mut highest_address = PhysicalAddress::new(0);

        for descriptor in self.iter() {
            if descriptor.max_physical_address() > highest_address {
                highest_address = descriptor.max_physical_address();
            }
        }
        return highest_address;
    }
}

impl Default for EfiMemoryMap {
    fn default() -> EfiMemoryMap {
        EfiMemoryMap {
            descriptors: core::ptr::null_mut(),
            map_size: 0,
            descriptor_size: 0,
            num_pages: 0,
        }
    }
}

pub struct EfiMemoryMapIterator<'a> {
    mem_map: &'a EfiMemoryMap,
    current_index: usize,
    max_index: usize,
}

impl<'a> Iterator for EfiMemoryMapIterator<'a> {
    type Item = EfiMemoryDescriptor;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index == self.max_index {
            return None;
        } else {
            let output = match self.mem_map.get_descriptor(self.current_index) {
                Ok(descriptor) => descriptor,
                Err(()) => { return None; } //Impossible
            };
            self.current_index += 1;
            return Some(output);
        }
    }
}

pub struct GetMemoryMapOutput {
    pub map: EfiMemoryMap,
    pub map_key: usize,
    pub descriptor_version: u32,
}

impl Default for GetMemoryMapOutput {
    fn default() -> GetMemoryMapOutput {
        GetMemoryMapOutput {
            map: EfiMemoryMap::default(),
            map_key: 0,
            descriptor_version: 0,
        }
    }
}
