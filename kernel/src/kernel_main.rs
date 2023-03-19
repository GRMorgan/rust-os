use x86_64_hardware::{com1_println, devices::uart_16550::COM1};
use x86_64_hardware::tables::*;


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
    loop { }
}