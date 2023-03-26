use spin::Mutex;
use x86_64_hardware::memory::paging::{PageFrameAllocator, FrameAllocator};

pub static TEMP_ALLOC: Mutex<PageFrameAllocator> = Mutex::new(unsafe { PageFrameAllocator::new_uninit() });

pub fn get_temp_allocator() -> TempAllocWrapper {
    return TempAllocWrapper {};
}

pub struct TempAllocWrapper {}

impl FrameAllocator for TempAllocWrapper {
    fn free_page(&mut self, address: x86_64_hardware::memory::PhysicalAddress) {
        TEMP_ALLOC.lock().free_page(address);
    }

    fn request_page(&mut self) -> x86_64_hardware::memory::PhysicalAddress {
        return TEMP_ALLOC.lock().request_page();
    }
}
