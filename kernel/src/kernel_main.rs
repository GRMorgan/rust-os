use x86_64_hardware::{com1_println, devices::uart_16550::COM1};


#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn kernel_main() {
    COM1.lock().initialise();
    com1_println!("Hello from kernel!");
    loop { }
}