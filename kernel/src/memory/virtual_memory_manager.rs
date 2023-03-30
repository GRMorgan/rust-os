use core::{panic, cell::UnsafeCell};

use spin::Mutex;
use x86_64_hardware::memory::{PhysicalAddress, VirtualAddress, paging::{FrameAllocator, PageTableManager}};

use super::FRAME_ALLOCATOR;

struct MemorySpace {
    p4_addr: PhysicalAddress,
    is_wired: bool,
    heap_base: VirtualAddress,
    heap_end: VirtualAddress,
}

impl MemorySpace {
    pub const fn new(p4_addr: PhysicalAddress, is_wired: bool, heap_base: VirtualAddress) -> MemorySpace {
        MemorySpace { p4_addr: p4_addr, is_wired: is_wired, heap_base: heap_base, heap_end: heap_base }
    }

    pub fn reinit(&mut self, p4_addr: PhysicalAddress, is_wired: bool, heap_base: VirtualAddress) {
        self.p4_addr = p4_addr;
        self.is_wired = is_wired;
        self.heap_base = heap_base;
        self.heap_end = heap_base;
    }

    pub fn alter_heap(&mut self, mapped_offset: u64, page_increment: isize) -> VirtualAddress {
        if page_increment == 0 {
            return self.heap_end;
        }

        let old_heap_end = self.heap_end;
        if page_increment >  0 {
            // Grow heap
            let page_table_manager = PageTableManager::new(self.p4_addr, mapped_offset);
            if self.is_wired {
                for page_no in 0..page_increment {
                    let cur_virtual_addr = old_heap_end.increment_page_4kb(page_no as u64);
                    let cur_phys_addr = FRAME_ALLOCATOR.request_page();
                    page_table_manager.map_memory(cur_virtual_addr, cur_phys_addr, &FRAME_ALLOCATOR);
                }
            }
            
            self.heap_end = self.heap_end.increment_page_4kb(page_increment as u64);
            return self.heap_end;
        } else {
            // TODO - Shrink heap
            return self.heap_end;
        }
    }
}

pub struct VirtualMemoryManager {
    vmem0: Mutex<MemorySpace>,
    mapped_mem_offset: UnsafeCell<u64>,
}

impl VirtualMemoryManager {
    pub const fn new_uninit() -> VirtualMemoryManager {
        return VirtualMemoryManager::new(0, PhysicalAddress::new(0), false, VirtualAddress::new(0));
    }

    pub const fn new(mapped_mem_offset: u64, vmem0_p4_addr: PhysicalAddress, is_wired: bool, heap_base: VirtualAddress) -> VirtualMemoryManager {
        VirtualMemoryManager {
            vmem0: Mutex::new(MemorySpace::new(vmem0_p4_addr, true, heap_base)),
            mapped_mem_offset: UnsafeCell::new(mapped_mem_offset),
        }
    }

    pub fn init(&self, mapped_mem_offset: u64, vmem0_p4_addr: PhysicalAddress, is_wired: bool, heap_base: VirtualAddress) {
        self.set_mapped_mem_offset(mapped_mem_offset);
        self.vmem0.lock().reinit(vmem0_p4_addr, is_wired, heap_base);
    }

    pub fn alter_heap(&self, mem_space: usize, page_increment: isize) -> VirtualAddress {
        if mem_space == 0 {
            return self.vmem0.lock().alter_heap(self.mapped_mem_offset(), page_increment);
        } else {
            panic!("Unknown mem_space: {}", mem_space);
        }
    }

    fn set_mapped_mem_offset(&self, value: u64) {
        unsafe { *self.mapped_mem_offset.get() = value };
    }

    fn mapped_mem_offset(&self) -> u64 {
        return unsafe { *self.mapped_mem_offset.get() };
    }
}


unsafe impl Sync for VirtualMemoryManager {}

pub static VIRTUAL_MEMORY_MANAGER: VirtualMemoryManager = VirtualMemoryManager::new_uninit();
