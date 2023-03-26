use x86_64_hardware::memory::paging::{PageFrameAllocator, PageTableManager};
use x86_64_hardware::{com1_println, devices::uart_16550::COM1};
use x86_64_hardware::tables::*;

use crate::temp_allocator::{TEMP_ALLOC, get_temp_allocator};


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

    unsafe { TEMP_ALLOC.lock().init(&meminfo.bitmap, meminfo.free_memory, meminfo.reserved_memory, meminfo.used_memory) };
    let mut allocator =  get_temp_allocator();

    let page_table_manager = PageTableManager::new_from_cr3(unsafe { (*bootinfo).page_table_memory_offset});
    for index in 0..255usize {
        page_table_manager.unmap_p4_index(index, &mut allocator);
    }
    com1_println!("After identity map cleared!");

    loop { }
}