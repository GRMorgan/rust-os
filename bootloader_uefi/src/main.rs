#![no_main]
#![no_std]

use r_efi::efi;

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[export_name = "efi_main"]
pub extern "C" fn main(h: efi::Handle, st: *mut efi::SystemTable) -> efi::Status {
    return efi::Status::SUCCESS;
}
