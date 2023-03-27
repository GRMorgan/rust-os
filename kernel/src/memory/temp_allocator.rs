use x86_64_hardware::memory::{paging::{PageFrameAllocator, FrameAllocator}, PhysicalAddress};

use super::PhysicalMemoryManagerFunctions;

pub static TEMP_ALLOC: PageFrameAllocator = PageFrameAllocator::new_uninit();


pub fn get_pmm_functions() -> PhysicalMemoryManagerFunctions {
    return PhysicalMemoryManagerFunctions::new(bulk_alloc, free);
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

