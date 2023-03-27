use x86_64_hardware::memory::{paging::{PageFrameAllocator, FrameAllocator}, PhysicalAddress};

use super::PhysicalMemoryManagerFunctions;

pub static TEMP_ALLOC: PageFrameAllocator = PageFrameAllocator::new_uninit();

pub fn get_temp_allocator() -> TempAllocWrapper {
    return TempAllocWrapper {};
}

pub struct TempAllocWrapper {}

impl TempAllocWrapper {
    pub fn get_pmm_functions() -> PhysicalMemoryManagerFunctions {
        return PhysicalMemoryManagerFunctions::new(TempAllocWrapper::bulk_alloc, TempAllocWrapper::free);
    }

    pub fn bulk_alloc(store: &mut[PhysicalAddress], count: usize) -> usize {
        for index in 0..count {
            store[index] = TEMP_ALLOC.request_page();
        }

        return count;
    }

    pub fn free(address: PhysicalAddress) {
        TEMP_ALLOC.free_page(address);
    }
}

impl FrameAllocator for TempAllocWrapper {
    fn free_page(&self, address: x86_64_hardware::memory::PhysicalAddress) {
        TEMP_ALLOC.free_page(address);
    }

    fn request_page(&self) -> x86_64_hardware::memory::PhysicalAddress {
        return TEMP_ALLOC.request_page();
    }
}
