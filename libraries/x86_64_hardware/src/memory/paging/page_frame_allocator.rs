use bitmap;
use spin::Mutex;
use crate::memory::{PAGE_SIZE,PhysicalAddress};


pub trait FrameAllocator {
    fn request_page(&self) -> PhysicalAddress;

    fn free_page(&self, address: PhysicalAddress);
}

struct PageFrameAllocatorInner {
    pub page_bitmap: bitmap::Bitmap,
    free_memory: u64,
    reserved_memory: u64,
    used_memory: u64,
    last_allocated_page: usize,
}

impl PageFrameAllocatorInner {
    pub const unsafe fn new_uninit() -> PageFrameAllocatorInner {
        PageFrameAllocatorInner { 
            page_bitmap: bitmap::Bitmap::new_uninit(),
            free_memory: 0,
            reserved_memory: 0,
            used_memory: 0,
            last_allocated_page: 0, 
        }
    }

    pub unsafe fn init(&mut self, page_bitmap: &bitmap::Bitmap, free_memory: u64, reserved_memory: u64, used_memory: u64) {
        self.page_bitmap = *page_bitmap;
        self.free_memory = free_memory;
        self.reserved_memory = reserved_memory;
        self.used_memory = used_memory;
        self.last_allocated_page = 0;
    }

    pub fn free_pages(&mut self, address: PhysicalAddress, page_count: usize) {
        for i in 0..page_count {
            self.free_page(address.increment_page_4kb(i as u64));
        }
    }

    pub fn lock_page(&mut self, address: PhysicalAddress) {
        let page_number = address.as_usize() / PAGE_SIZE as usize;
        if self.page_bitmap.get(page_number) {
            return;
        } else {
            if self.page_bitmap.set(page_number, true) {
                self.free_memory -= PAGE_SIZE;
                self.used_memory += PAGE_SIZE;
            }
        }
    }

    pub fn lock_pages(&mut self, address: PhysicalAddress, page_count: usize) {
        for i in 0..page_count {
            self.lock_page(address.increment_page_4kb(i as u64));
        }
    }

    pub fn unreserve_page(&mut self, address: PhysicalAddress) {
        let page_number = address.as_usize() / PAGE_SIZE as usize;
        if !self.page_bitmap.get(page_number) {
            return;
        } else {
            if self.page_bitmap.set(page_number, false) {
                self.free_memory += PAGE_SIZE;
                self.reserved_memory -= PAGE_SIZE;
                if self.last_allocated_page > page_number {
                    self.last_allocated_page = page_number;
                }
            }
        }
    }

    pub fn unreserve_pages(&mut self, address: PhysicalAddress, page_count: usize) {
        for i in 0..page_count {
            self.unreserve_page(address.increment_page_4kb(i as u64));
        }
    }

    #[allow(dead_code)]
    pub fn reserve_page(&mut self, address: PhysicalAddress) {
        let page_number = address.as_usize() / PAGE_SIZE as usize;
        if self.page_bitmap.get(page_number) {
            return;
        } else {
            if self.page_bitmap.set(page_number, true) {
                self.free_memory -= PAGE_SIZE;
                self.reserved_memory += PAGE_SIZE;
            }
        }
    }

    #[allow(dead_code)]
    pub fn reserve_pages(&mut self, address: PhysicalAddress, page_count: usize) {
        for i in 0..page_count {
            self.reserve_page(address.increment_page_4kb(i as u64));
        }
    }

    fn request_page(&mut self) -> PhysicalAddress {
        for index in self.last_allocated_page..self.page_bitmap.size() * 8 {
            if !self.page_bitmap.get(index) {
                self.last_allocated_page = index;
                let addr = PhysicalAddress::new(index as u64 * PAGE_SIZE);
                self.lock_page(addr);
                return addr;
            }
        }

        return PhysicalAddress::new(0);
    }

    fn free_page(&mut self, address: PhysicalAddress) {
        let page_number = address.as_usize() / PAGE_SIZE as usize;
        if !self.page_bitmap.get(page_number) {
            return;
        } else {
            if self.page_bitmap.set(page_number, false) {
                self.free_memory += PAGE_SIZE;
                self.used_memory -= PAGE_SIZE;
                if self.last_allocated_page > page_number {
                    self.last_allocated_page = page_number;
                }
            }
        }
    }
}

pub struct PageFrameAllocator {
    lockable_allocator: Mutex<PageFrameAllocatorInner>,
}

impl PageFrameAllocator {
    pub unsafe fn new_from_bitmap(page_bitmap: &bitmap::Bitmap, free_memory: u64, reserved_memory: u64, used_memory: u64) -> PageFrameAllocator {
        return PageFrameAllocator::new(PageFrameAllocatorInner{
            page_bitmap: *page_bitmap,
            free_memory: free_memory,
            reserved_memory: reserved_memory,
            used_memory: used_memory,
            last_allocated_page: 0,
        });
    }

    const fn new(inner: PageFrameAllocatorInner) -> PageFrameAllocator {
        PageFrameAllocator { lockable_allocator: Mutex::new(inner) }
    }

    pub const fn new_uninit() -> PageFrameAllocator {
        return PageFrameAllocator::new(unsafe { PageFrameAllocatorInner::new_uninit() });
    }

    pub unsafe fn init(&self, page_bitmap: &bitmap::Bitmap, free_memory: u64, reserved_memory: u64, used_memory: u64) {
        self.lockable_allocator.lock().init(page_bitmap, free_memory, reserved_memory, used_memory);
    }

    pub fn page_bitmap(&self) -> bitmap::Bitmap {
        return self.lockable_allocator.lock().page_bitmap;
    }

    pub fn get_free_ram(&self) -> u64 {
        return self.lockable_allocator.lock().free_memory;
    }

    pub fn get_used_ram(&self) -> u64 {
        return self.lockable_allocator.lock().used_memory;
    }

    pub fn get_reserved_ram(&self) -> u64 {
        return self.lockable_allocator.lock().reserved_memory;
    }

    pub fn free_pages(&self, address: PhysicalAddress, page_count: usize) {
        self.lockable_allocator.lock().free_pages(address, page_count);
    }

    pub fn lock_page(&self, address: PhysicalAddress) {
        self.lockable_allocator.lock().lock_page(address);
    }

    pub fn lock_pages(&self, address: PhysicalAddress, page_count: usize) {
        self.lockable_allocator.lock().lock_pages(address, page_count);
    }

    pub fn unreserve_page(&self, address: PhysicalAddress) {
        self.lockable_allocator.lock().unreserve_page(address);
    }

    pub fn unreserve_pages(&self, address: PhysicalAddress, page_count: usize) {
        self.lockable_allocator.lock().unreserve_pages(address, page_count);
    }
}

impl FrameAllocator for PageFrameAllocator {
    fn request_page(&self) -> PhysicalAddress {
        return self.lockable_allocator.lock().request_page();
    }

    fn free_page(&self, address: PhysicalAddress) {
        return self.lockable_allocator.lock().free_page(address);
    }
}
