use crate::memory::paging::*;
use crate::memory::*;

//Note this only supports identity mapping currently
pub struct PageTableManager {
    p4: PhysicalAddress,
}

impl PageTableManager {
    pub fn new_from_allocator(allocator: &mut impl FrameAllocator, mem_offset: u64) -> PageTableManager {
        let p4_paddr = allocator.request_page();
        let p4_vaddr = p4_paddr.get_virtual_address_at_offset(mem_offset);
        let p4_table = unsafe{ p4_vaddr.get_mut_ptr::<PageTable>() };
        unsafe { (*p4_table).make_unused(); }
        return PageTableManager::new(p4_paddr);
    }

    pub fn new_from_cr3() -> PageTableManager {
        let p4_addr: u64;

        unsafe {
            core::arch::asm!("mov {}, cr3", out(reg) p4_addr, options(nomem, nostack, preserves_flags));
        }

        return Self::new(PhysicalAddress::new(p4_addr));
    }

    pub fn new(p4: PhysicalAddress) -> PageTableManager {
        return PageTableManager {
            p4: p4,
        }
    }

    pub fn get_p4(&self) -> &PageTable {
        let virt_address = self.translate_address(self.p4);
        let p4_ptr = unsafe { virt_address.get_mut_ptr::<PageTable>() };

        unsafe { return &(*p4_ptr); }
    }

    //Just identity map for now but this will be used for offset mapping
    fn translate_address(&self, physical_addr: PhysicalAddress) -> VirtualAddress {
        return VirtualAddress::new(physical_addr.as_u64());
    }

    pub unsafe fn activate_page_table(&self) {
        core::arch::asm!("mov cr3, {}", in(reg) self.p4.as_u64(), options(nostack, preserves_flags));
    }

    pub fn map_memory_pages(&self, virtual_addr : VirtualAddress, physical_addr: PhysicalAddress, num_pages: u64, allocator: &mut impl FrameAllocator) {
        for page in 0..num_pages {
            let cur_paddr = physical_addr.increment_page_4kb(page);
            let cur_vaddr = virtual_addr.increment_page_4kb(page);
            self.map_memory(cur_vaddr, cur_paddr, allocator);
        }
    }

    pub fn map_memory(&self, virtual_addr : VirtualAddress, physical_addr: PhysicalAddress, allocator: &mut impl FrameAllocator) {
        let p4_ptr = unsafe { self.translate_address(self.p4).get_mut_ptr::<PageTable>() };

        let mut p4_table_entry = unsafe { (*p4_ptr).table[virtual_addr.p4_index()] };
        if !p4_table_entry.present() {
            let p3_addr = self.create_and_map_p3(virtual_addr, physical_addr, allocator);
            p4_table_entry.make_unused();
            p4_table_entry.set_address(p3_addr);
            p4_table_entry.set_present(true);
            p4_table_entry.set_read_write(true);
            unsafe { (*p4_ptr).table[virtual_addr.p4_index()] = p4_table_entry; }
        } else {
            let p3_ptr: *mut PageTable = unsafe { self.translate_address(p4_table_entry.address()).get_mut_ptr::<PageTable>() };
            self.map_p3(p3_ptr, virtual_addr, physical_addr, allocator);
        }
    }

    fn create_and_map_p3(&self, virtual_addr : VirtualAddress, physical_addr: PhysicalAddress, allocator: &mut impl FrameAllocator) -> PhysicalAddress {
        let output = allocator.request_page();
        let p3_ptr = unsafe { self.translate_address(output).get_mut_ptr::<PageTable>() };
        unsafe { (*p3_ptr).make_unused() }
        self.map_p3(p3_ptr, virtual_addr, physical_addr, allocator);

        return output;
    }

    fn map_p3(&self, p3_ptr: *mut PageTable, virtual_addr : VirtualAddress, physical_addr: PhysicalAddress, allocator: &mut impl FrameAllocator) {
        let mut p3_table_entry = unsafe { (*p3_ptr).table[virtual_addr.p3_index()] };
        if !p3_table_entry.present() {
            let p2_addr = self.create_and_map_p2(virtual_addr, physical_addr, allocator);
            p3_table_entry.set_address(p2_addr);
            p3_table_entry.set_present(true);
            p3_table_entry.set_read_write(true);
            unsafe { (*p3_ptr).table[virtual_addr.p3_index()] = p3_table_entry; }
        } else {
            let p2_ptr: *mut PageTable = unsafe { self.translate_address(p3_table_entry.address()).get_mut_ptr::<PageTable>() };
            self.map_p2(p2_ptr, virtual_addr, physical_addr, allocator);
        }
    }

    fn create_and_map_p2(&self, virtual_addr : VirtualAddress, physical_addr: PhysicalAddress, allocator: &mut impl FrameAllocator) -> PhysicalAddress {
        let output = allocator.request_page();
        let p2_ptr = unsafe { self.translate_address(output).get_mut_ptr::<PageTable>() };
        unsafe { (*p2_ptr).make_unused() }
        self.map_p2(p2_ptr, virtual_addr, physical_addr, allocator);

        return output;
    }

    fn map_p2(&self, p2_ptr: *mut PageTable, virtual_addr : VirtualAddress, physical_addr: PhysicalAddress, allocator: &mut impl FrameAllocator) {
        let mut p2_table_entry = unsafe { (*p2_ptr).table[virtual_addr.p2_index()] };
        
        if !p2_table_entry.present() {
            let p1_addr = self.create_and_map_p1(virtual_addr, physical_addr, allocator);
            p2_table_entry.set_address(p1_addr);
            p2_table_entry.set_present(true);
            p2_table_entry.set_read_write(true);
            unsafe { (*p2_ptr).table[virtual_addr.p2_index()] = p2_table_entry; }
        } else {
            let p1_ptr: *mut PageTable = unsafe { self.translate_address(p2_table_entry.address()).get_mut_ptr::<PageTable>() };
            self.map_p1(p1_ptr, virtual_addr, physical_addr);
        }
    }

    fn create_and_map_p1(&self, virtual_addr : VirtualAddress, physical_addr: PhysicalAddress, allocator: &mut impl FrameAllocator) -> PhysicalAddress {
        let output = allocator.request_page();
        let p1_ptr = unsafe { self.translate_address(output).get_mut_ptr::<PageTable>() };
        unsafe { (*p1_ptr).make_unused() }
        self.map_p1(p1_ptr, virtual_addr, physical_addr);

        return output;
    }

    fn map_p1(&self, p1_ptr: *mut PageTable, virtual_addr : VirtualAddress, physical_addr: PhysicalAddress) {
        let mut p1_table_entry = unsafe { (*p1_ptr).table[virtual_addr.p1_index()] };
        p1_table_entry.set_address(physical_addr);
        p1_table_entry.set_present(true);
        p1_table_entry.set_read_write(true);
        unsafe { (*p1_ptr).table[virtual_addr.p1_index()] = p1_table_entry; }
    }

    fn get_page_table_entry(&self, virtual_addr : VirtualAddress) -> Option<&mut PageTableEntry> {
        let p4_ptr = unsafe { self.translate_address(self.p4).get_mut_ptr::<PageTable>() };
        
        let p4_table_entry = unsafe { (*p4_ptr).table[virtual_addr.p4_index()] };
        let p3_ptr: *mut PageTable = if !p4_table_entry.present() {
            return None;
        } else {
            unsafe { self.translate_address(p4_table_entry.address()).get_mut_ptr::<PageTable>() }
        };

        let p3_table_entry = unsafe { (*p3_ptr).table[virtual_addr.p3_index()] };
        let p2_ptr: *mut PageTable = if !p3_table_entry.present() {
            return None;
        } else {
            if p3_table_entry.page_size() { //1GB big page
                return unsafe { Some(&mut (*p3_ptr).table[virtual_addr.p3_index()]) };
            } else {
                unsafe { self.translate_address(p3_table_entry.address()).get_mut_ptr::<PageTable>() }
            }
        };

        let p2_table_entry = unsafe { (*p2_ptr).table[virtual_addr.p2_index()] };
        let p1_ptr: *mut PageTable = if !p2_table_entry.present() {
            return None;
        } else {
            if p2_table_entry.page_size() { //2MB big page
                return unsafe { Some(&mut (*p2_ptr).table[virtual_addr.p2_index()]) }; 
            } else {
                unsafe { self.translate_address(p2_table_entry.address()).get_mut_ptr::<PageTable>() }
            }
        };

        return unsafe { Some(&mut (*p1_ptr).table[virtual_addr.p1_index()]) };
    }

    pub fn get_page_physical_address(&self, virtual_addr : VirtualAddress) -> Option<PhysicalAddress> {
        let page_table_entry = self.get_page_table_entry(virtual_addr)?;

        return Some(page_table_entry.address());
    }
}
