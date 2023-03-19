use x86_64_hardware::{com1_println, devices::uart_16550::COM1};
use x86_64_hardware::tables::*;


#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn kernel_main(bootinfo: *mut bootinfo::BootInfo) {
    COM1.lock().initialise();
    com1_println!("Hello from kernel!");
    com1_println!("Next kernel virtual address: {:#x}", unsafe { (*bootinfo).next_available_kernel_page.as_u64() } );
    init_default_gdt();
    com1_println!("Loaded GDT!");
    loop { }
}