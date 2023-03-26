use bitmap;
use crate::memory::{PAGE_SIZE,PhysicalAddress};


pub trait FrameAllocator {
    fn request_page(&mut self) -> PhysicalAddress;

    fn free_page(&mut self, address: PhysicalAddress);
}

pub struct PageFrameAllocator {
    pub page_bitmap: bitmap::Bitmap,
    free_memory: u64,
    reserved_memory: u64,
    used_memory: u64,
    last_allocated_page: usize,
}

impl PageFrameAllocator {
    pub const unsafe fn new_uninit() -> PageFrameAllocator {
        PageFrameAllocator { 
            page_bitmap: bitmap::Bitmap::new_uninit(),
            free_memory: 0,
            reserved_memory: 0,
            used_memory: 0,
            last_allocated_page: 0, 
        }
    }

    pub unsafe fn new_from_bitmap(page_bitmap: &bitmap::Bitmap, free_memory: u64, reserved_memory: u64, used_memory: u64) -> PageFrameAllocator {
        PageFrameAllocator{
            page_bitmap: *page_bitmap,
            free_memory: free_memory,
            reserved_memory: reserved_memory,
            used_memory: used_memory,
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

    pub fn get_free_ram(&self) -> u64 {
        return self.free_memory;
    }

    pub fn get_used_ram(&self) -> u64 {
        return self.used_memory;
    }

    pub fn get_reserved_ram(&self) -> u64 {
        return self.reserved_memory;
    }
}

impl FrameAllocator for PageFrameAllocator {
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
