#![no_main]
#![no_std]

use core::fmt::Write;
use loaded_asset_list::LoadedAssetList;
use r_efi::efi;
use elf;
use x86_64_hardware::{memory::{PAGE_SIZE, VirtualAddress, PhysicalAddress, paging::PageTableManager}, com1_println};

mod uefi;
mod unicode;
mod loaded_asset_list;

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[export_name = "efi_main"]
pub extern "C" fn efi_main(h: efi::Handle, st: *mut efi::SystemTable) -> efi::Status {
    //This is safe since we retrieved the SystemTable pointer directly from efi_main
    let system_table = unsafe { uefi::SystemTableWrapper::new(st)};

    match main(h, system_table) {
        Ok(()) => { 
            loop {}
            return efi::Status::SUCCESS;
        }
        Err(s) => { 
            let mut out = system_table.con_out();
            efi_write!(out, "Error: {:#x}\r\n", s.as_usize());
            return s; 
        }
    }

}

fn main(h: efi::Handle, system_table: uefi::SystemTableWrapper) -> Result<(),efi::Status> {
    let mut out = system_table.con_out();

    let (kernel_asset_list, entry_point) = load_kernel(h, system_table)?;

    let mut mem_info = system_table.boot_services().get_memory_map()?;

    system_table.boot_services().exit_boot_services(h, mem_info.map_key)?;

    let mut allocator = mem_info.map.init_frame_allocator();
    let max_physical_address = mem_info.map.max_physical_address();
    //We're done with the mem_map so free the pages
    mem_info.map.free_pages(&mut allocator);

    for asset in kernel_asset_list.iter() {
        allocator.lock_pages(asset.physical_address, asset.num_pages);
    }

    let page_table_manager = PageTableManager::new_from_allocator(&mut allocator, 0);

    //Identity map the entire memory range
    let num_mem_pages = max_physical_address.as_u64() / PAGE_SIZE;
    page_table_manager.map_memory_pages(VirtualAddress::new(0), PhysicalAddress::new(0), num_mem_pages, &mut allocator);

    //Map the kernel into the new page table
    for asset in kernel_asset_list.iter() {
        page_table_manager.map_memory_pages(asset.virtual_address, asset.physical_address, asset.num_pages as u64, &mut allocator);
    }

    unsafe { page_table_manager.activate_page_table(); }
    com1_println!("New page table activated");

    return Ok(());
}

fn load_kernel(h: efi::Handle, system_table: uefi::SystemTableWrapper) -> Result<(LoadedAssetList, VirtualAddress), efi::Status> {
    let mut out = system_table.con_out();

    let file_volume = system_table.boot_services().open_volume(h)?;
    let kernel_file = file_volume.open("kernel.elf", r_efi::protocols::file::MODE_READ, r_efi::protocols::file::READ_ONLY)?;
    efi_write!(out, "Opened kernel file\r\n");

    let elf_common: elf::ElfHeaderCommon  = kernel_file.read_struct::<elf::ElfHeaderCommon>()?;

    validate_elf(&elf_common, &mut out)?;
    efi_write!(out, "Kernel header verified successfully!\r\n");

    kernel_file.set_position(0)?;

    let elf_64: elf::ElfHeader64 = kernel_file.read_struct::<elf::ElfHeader64>()?;

    let mut kernel_asset_list = crate::loaded_asset_list::LoadedAssetList::new(elf_64.e_phnum as usize, system_table)?;
    for header_index in 0..elf_64.e_phnum {
        kernel_file.set_position(elf_64.e_phoff + (u64::from(header_index) * u64::from(elf_64.e_phentsize)));
        let phdr: elf::ElfPhysicalHeader64 = kernel_file.read_struct::<elf::ElfPhysicalHeader64>()?;

        match phdr.p_type() {
            elf::ElfPhysicalType::ElfPhysicalTypeLoad => {
                let pages: usize = ((phdr.p_memsz as usize) + PAGE_SIZE as usize - 1)  / PAGE_SIZE as usize;
                let kernel_mem = system_table.boot_services().allocate_pages::<core::ffi::c_void>(r_efi::system::LOADER_DATA, pages)?;

                kernel_file.set_position(phdr.p_offset);
                let mut psize = phdr.p_filesz as usize;
                kernel_file.read(&mut psize, kernel_mem)?;
                kernel_asset_list.add_asset(PhysicalAddress::new(kernel_mem as u64), pages, VirtualAddress::new(phdr.p_vaddr));
            },
            _ => {}
        }
    }

    return Ok((kernel_asset_list, VirtualAddress::new(elf_64.e_entry)));
}

fn validate_elf(header: &elf::ElfHeaderCommon, out: &mut uefi::SimpleTextOutputProtocol) -> Result<(),efi::Status> {
    if !header.valid_magic() {
        efi_write!(out, "Invalid magic \r\n");
        return Err(efi::Status::LOAD_ERROR);
    }

    if header.class() != elf::ElfClass::ElfClass64 {
        efi_write!(out, "Invalid class {:?} \r\n", header.class());
        return Err(efi::Status::LOAD_ERROR);
    }

    if header.data_encoding() != elf::ElfData::ElfData2Lsb {
        efi_write!(out, "Invalid data encoding {:?} \r\n", header.data_encoding());
        return Err(efi::Status::LOAD_ERROR);
    }

    if header.e_type() != elf::ElfType::ElfTypeExec {
        efi_write!(out, "Invalid type {:?} \r\n", header.e_type());
        return Err(efi::Status::LOAD_ERROR);
    }

    if header.e_machine() != elf::ElfMachine::ElfMachineX8664 {
        efi_write!(out, "Invalid machine {:?} \r\n", header.e_machine());
        return Err(efi::Status::LOAD_ERROR);
    }

    if header.e_version() != elf::ElfVersion::ElfVersionCurrent {
        efi_write!(out, "Invalid elf version {:?} \r\n", header.e_version());
        return Err(efi::Status::LOAD_ERROR);
    }
    
    return Ok(());
}
