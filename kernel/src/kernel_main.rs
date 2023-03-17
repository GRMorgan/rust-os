use x86_64_hardware::com1_println;


#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() {
    com1_println!("Hello, world!");
    loop { }
}