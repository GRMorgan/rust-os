use x86_64_hardware::memory::{paging::PageFrameAllocator, PhysicalAddress};

pub struct MemInfo {
    pub bitmap: bitmap::Bitmap,
    pub free_memory: u64,
    pub reserved_memory: u64,
    pub used_memory: u64,
    pub max_physical_address: PhysicalAddress,
}

impl MemInfo {
    pub fn new_from_allocator(allocator: &mut PageFrameAllocator, max_physical_address: PhysicalAddress) -> MemInfo {
        MemInfo {
            bitmap: allocator.page_bitmap,
            free_memory: allocator.get_free_ram(),
            reserved_memory: allocator.get_reserved_ram(),
            used_memory: allocator.get_used_ram(),
            max_physical_address: max_physical_address,
        }
    }

    pub fn move_out(&mut self) -> MemInfo {
        let output = MemInfo {
            bitmap: self.bitmap,
            free_memory: self.free_memory,
            reserved_memory: self.reserved_memory,
            used_memory: self.used_memory,
            max_physical_address: self.max_physical_address,
        };

        *self = MemInfo::default();

        return output;
    }
    
}

impl Default for MemInfo {
    fn default() -> MemInfo {
        MemInfo {
            bitmap: unsafe { bitmap::Bitmap::new(0, core::ptr::null_mut::<u8>()) },
            free_memory: 0,
            reserved_memory: 0,
            used_memory: 0,
            max_physical_address: PhysicalAddress::new(0),
        }   
    }
}