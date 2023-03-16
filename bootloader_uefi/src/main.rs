#![no_main]
#![no_std]

use core::fmt::Write;

use r_efi::efi;

mod uefi;
mod unicode;

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[export_name = "efi_main"]
pub extern "C" fn efi_main(h: efi::Handle, st: *mut efi::SystemTable) -> efi::Status {
    //This is safe since we retrieved the SystemTable pointer directly from efi_main
    let system_table = unsafe { uefi::SystemTableWrapper::new(st)};

    match main(h, system_table) {
        Ok(()) => { return efi::Status::SUCCESS; }
        Err(s) => { return s; }
    }

}

fn main(h: efi::Handle, system_table: uefi::SystemTableWrapper) -> Result<(),efi::Status> {
    let mut out = system_table.con_out();

    efi_write!(out, "Hello, world!\r\n");

    return Ok(());
}
