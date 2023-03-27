use x86_64_hardware::memory::paging::{PageTableManager, FrameAllocator};
use x86_64_hardware::{com1_println, devices::uart_16550::COM1};
use x86_64_hardware::tables::*;

use crate::memory::{TEMP_ALLOC, get_temp_allocator, TempAllocWrapper, FRAME_ALLOCATOR};


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
    FRAME_ALLOCATOR.set_mem_manager(TempAllocWrapper::get_pmm_functions());
    //TODO - Kernel crashes if we don't pre-allocate this buffer. We need to investigate this because it shouldn't crash
    FRAME_ALLOCATOR.fill_buffer();

    let page_table_manager = PageTableManager::new_from_cr3(unsafe { (*bootinfo).page_table_memory_offset});
    for index in 0..255usize {
        page_table_manager.unmap_p4_index(index, & FRAME_ALLOCATOR);
    }
    com1_println!("After identity map cleared!");

    loop { }
}