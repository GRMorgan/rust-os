use x86_64_hardware::memory::VirtualAddress;
use x86_64_hardware::memory::paging::PageTableManager;
use x86_64_hardware::{com1_println, devices::uart_16550::COM1};
use x86_64_hardware::tables::*;

use crate::memory::{TEMP_ALLOC, FRAME_ALLOCATOR, get_pmm_functions, VIRTUAL_MEMORY_MANAGER};


#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn kernel_main(bootinfo: *mut bootinfo::BootInfo) {
    COM1.lock().initialise();

    if !unsafe { (*bootinfo).valid_magic() } {
        com1_println!("Invalid BootInfo header!");
        loop { }
    }

    com1_println!("Starting kernel initialisation!");
    init_default_gdt();
    com1_println!("Loaded GDT!");

    let meminfo = unsafe { (*bootinfo).meminfo.move_out() };

    unsafe { TEMP_ALLOC.init(&meminfo.bitmap, meminfo.free_memory, meminfo.reserved_memory, meminfo.used_memory) };
    FRAME_ALLOCATOR.set_mem_manager(get_pmm_functions());


    let mem_map_offset = unsafe { (*bootinfo).page_table_memory_offset };
    let page_table_manager = PageTableManager::new_from_cr3(mem_map_offset);
    for index in 0..256usize {
        page_table_manager.unmap_p4_index(index, &FRAME_ALLOCATOR);
    }
    com1_println!("After identity map cleared!");

    let kernel_heap_base = VirtualAddress::new(0xFFFF800000000000);
    VIRTUAL_MEMORY_MANAGER.init(mem_map_offset, page_table_manager.get_p4_address(), true, kernel_heap_base);
    com1_println!("After VMM initialised!");
    VIRTUAL_MEMORY_MANAGER.alter_heap(0, 1);

    unsafe {
        let test_ptr = kernel_heap_base.get_mut_ptr::<u8>();
        *test_ptr = 5;
    }
    com1_println!("After heap access!");

    loop { }
}