use core::cell::UnsafeCell;

use data_structures::ringbuffer::RingBuffer;
use spin::Mutex;
use x86_64_hardware::memory::{PhysicalAddress, paging::FrameAllocator};

#[derive(Clone, Copy)]
pub struct PhysicalMemoryManagerFunctions {
    pub bulk_alloc: fn(store: &mut[PhysicalAddress], count: usize) -> usize,
    pub free: fn(page: PhysicalAddress),
}

impl PhysicalMemoryManagerFunctions {
    pub fn new(bulk_alloc: fn(store: &mut[PhysicalAddress], count: usize) -> usize, free: fn(page: PhysicalAddress)) -> PhysicalMemoryManagerFunctions {
        return PhysicalMemoryManagerFunctions { bulk_alloc: bulk_alloc, free: free };
    }
}

pub struct PhysicalFrameAllocator {
    buffer: RingBuffer<PhysicalAddress, 512>,
    mem_manager: UnsafeCell<Option<PhysicalMemoryManagerFunctions>>,
    fill_lock: Mutex<()>,
}

impl PhysicalFrameAllocator {
    pub fn new(mem_manager: PhysicalMemoryManagerFunctions) -> PhysicalFrameAllocator {
        PhysicalFrameAllocator { 
            buffer: RingBuffer::new(PhysicalAddress::new(0)),
            mem_manager: UnsafeCell::new(Some(mem_manager)),
            fill_lock: Mutex::new(()),
        }
    }
    pub const fn new_uninit() -> PhysicalFrameAllocator {
        PhysicalFrameAllocator { 
            buffer: RingBuffer::new(PhysicalAddress::new(0)),
            mem_manager: UnsafeCell::new(None),
            fill_lock: Mutex::new(()),
        }
    }

    pub fn set_mem_manager(&self, mem_manager: PhysicalMemoryManagerFunctions) {
        unsafe { *self.mem_manager.get() = Some(mem_manager) }
    }

    fn mem_manager(&self) -> PhysicalMemoryManagerFunctions {
        return unsafe { (*self.mem_manager.get()).unwrap() };
    }

    pub fn fill_buffer(&self) {
        let _lock_guard = self.fill_lock.lock();

        if self.buffer.is_empty() {
            let mut required_count = self.buffer.max_items() as usize / 2;

            while required_count > 0 {
                let mut alloc_buffer: [PhysicalAddress;256] = [PhysicalAddress::new(0);256];
                let alloced_count = self.bulk_alloc(&mut alloc_buffer, required_count);

                for index in 0..alloced_count {
                    self.buffer.write(alloc_buffer[index]);
                }
                required_count -= alloced_count;
            }
        }
    }

    fn bulk_alloc(&self, store: &mut [PhysicalAddress], count: usize) -> usize {
        return (self.mem_manager().bulk_alloc)(store, count);
    }

    fn free(&self, page: PhysicalAddress) {
        return (self.mem_manager().free)(page);
    }
}

impl FrameAllocator for PhysicalFrameAllocator {
    fn request_page(&self) -> PhysicalAddress {
        loop {
            match self.buffer.read() {
                Some(address) => { return address },
                None => {
                    self.fill_buffer();
                }
            }
        }
    }

    fn free_page(&self, address: PhysicalAddress) {
        match self.buffer.write(address) {
            Some(_) => {},
            None => { self.free(address); }
        }
    }
}

unsafe impl Sync for PhysicalFrameAllocator {}
unsafe impl Send for PhysicalFrameAllocator {}

pub static FRAME_ALLOCATOR: PhysicalFrameAllocator = PhysicalFrameAllocator::new_uninit();
