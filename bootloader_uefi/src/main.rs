#![no_main]
#![no_std]

use core::fmt::Write;
use bootinfo::{BootInfo, MemInfo};
use loaded_asset_list::LoadedAssetList;
use r_efi::efi;
use elf;
use x86_64_hardware::memory::{PAGE_SIZE, VirtualAddress, PhysicalAddress, MAX_VIRTUAL_ADDRESS};
use x86_64_hardware::memory:: paging::{PageTableManager, PageFrameAllocator, MAX_MEM_SIZE, MEM_1G};
use x86_64_hardware::com1_println;
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
    let bootinfo_num_pages = (core::mem::size_of::<BootInfo>() + PAGE_SIZE as usize - 1) / PAGE_SIZE as usize;
    let mut bootinfo = system_table.boot_services().allocate_pages::<BootInfo>(r_efi::system::LOADER_DATA, bootinfo_num_pages)?;
    unsafe { (*bootinfo) = BootInfo::default(); }

    unsafe { (*bootinfo).framebuffer = initialise_gop(system_table)?; }

    let (kernel_asset_list, entry_point) = load_kernel(h, system_table)?;

    let configuration_table = system_table.get_configuration_table();

    let mut mem_info = system_table.boot_services().get_memory_map()?;

    system_table.boot_services().exit_boot_services(h, mem_info.map_key)?;

    unsafe {
        (*bootinfo).framebuffer.clear_framebuffer(0, 0);
    }

    let mut allocator = mem_info.map.init_frame_allocator();
    let max_physical_address = mem_info.map.max_physical_address();
    //We're done with the mem_map so free the pages
    mem_info.map.free_pages(&mut allocator);

    let mut kernel_base_address = VirtualAddress::new(MAX_VIRTUAL_ADDRESS);
    for asset in kernel_asset_list.iter() {
        allocator.lock_pages(asset.physical_address, asset.num_pages);
        if asset.virtual_address < kernel_base_address {
            kernel_base_address = asset.virtual_address;
        }
    }

    let firmware_page_table_manager = PageTableManager::new_from_cr3(0);
    let (mut page_table_manager, offset) = match init_page_table_manager(&mut allocator, max_physical_address, kernel_base_address) {
        Some(ptm) => ptm,
        None => {
            com1_println!("Memsize too large");
            return Err(efi::Status::ABORTED);
        }
    };

    unsafe { (*bootinfo).page_table_memory_offset = offset; }

    //activate the new page table before turning on offset mapping
    unsafe { 
        page_table_manager.activate_page_table();
        page_table_manager.set_offset(offset);
    }

    firmware_page_table_manager.release_tables(&mut allocator);

    //Map the kernel into the new page table
    for asset in kernel_asset_list.iter() {
        page_table_manager.map_memory_pages(asset.virtual_address, asset.physical_address, asset.num_pages as u64, &mut allocator);
        let max_address = asset.virtual_address.increment_page_4kb(asset.num_pages as u64);
        if max_address > unsafe {(*bootinfo).next_available_kernel_page} {
            unsafe { (*bootinfo).next_available_kernel_page = max_address; }
        }
    }

    //Map the bootinfo into kernel space
    let bootinfo_virtual_address = unsafe { (*bootinfo).next_available_kernel_page };
    let bootinfo_physical_address = PhysicalAddress::new(bootinfo as u64);
    page_table_manager.map_memory_pages(bootinfo_virtual_address, bootinfo_physical_address, bootinfo_num_pages as u64, &mut allocator);
    unsafe { (*bootinfo).next_available_kernel_page = bootinfo_virtual_address.increment_page_4kb(bootinfo_num_pages as u64); }

    unsafe { page_table_manager.activate_page_table(); }

    //Update boot info pointer to point to the kernel mapped address
    bootinfo = unsafe { bootinfo_virtual_address.get_mut_ptr::<BootInfo>() };

    //Map bitmap into kernel space
    let num_bitmap_pages = (allocator.page_bitmap().size() as u64 + PAGE_SIZE - 1) / PAGE_SIZE;
    let bitmap_buffer_physical_addr = PhysicalAddress::new(unsafe { allocator.page_bitmap().get_buffer() as u64 });
    let bitmap_buffer_virtual_addr = unsafe { (*bootinfo).next_available_kernel_page };
    page_table_manager.map_memory_pages(bitmap_buffer_virtual_addr, bitmap_buffer_physical_addr, num_bitmap_pages, &mut allocator);
    unsafe { (*bootinfo).next_available_kernel_page = bitmap_buffer_virtual_addr.increment_page_4kb(num_bitmap_pages as u64); }

    unsafe { page_table_manager.activate_page_table(); }
    //Pass new kernel space bitmap location to kernel
    let output_bitmap = unsafe { bitmap::Bitmap::new(allocator.page_bitmap().size(), bitmap_buffer_virtual_addr.get_mut_ptr::<u8>()) };
    unsafe {  (*bootinfo).meminfo = MemInfo::new(output_bitmap, allocator.get_free_ram(), allocator.get_reserved_ram(), allocator.get_used_ram(), max_physical_address); }

    let kernel_start: unsafe extern "sysv64" fn(*mut BootInfo) = unsafe { core::mem::transmute(entry_point.get_mut_ptr::<core::ffi::c_void>()) };
    unsafe { (kernel_start)(bootinfo) };

    return Ok(());
}

fn load_kernel(h: efi::Handle, system_table: uefi::SystemTableWrapper) -> Result<(LoadedAssetList, VirtualAddress), efi::Status> {
    let file_volume = system_table.boot_services().open_volume(h)?;
    let kernel_file = file_volume.open("kernel.elf", r_efi::protocols::file::MODE_READ, r_efi::protocols::file::READ_ONLY)?;
    com1_println!("Opened kernel file");

    let elf_common: elf::ElfHeaderCommon  = kernel_file.read_struct::<elf::ElfHeaderCommon>()?;

    validate_elf(&elf_common)?;
    com1_println!("Kernel header verified successfully!");

    kernel_file.set_position(0)?;

    let elf_64: elf::ElfHeader64 = kernel_file.read_struct::<elf::ElfHeader64>()?;

    let mut kernel_asset_list = crate::loaded_asset_list::LoadedAssetList::new(elf_64.e_phnum as usize, system_table)?;
    for header_index in 0..elf_64.e_phnum {
        kernel_file.set_position(elf_64.e_phoff + (u64::from(header_index) * u64::from(elf_64.e_phentsize)))?;
        let phdr: elf::ElfPhysicalHeader64 = kernel_file.read_struct::<elf::ElfPhysicalHeader64>()?;

        match phdr.p_type() {
            elf::ElfPhysicalType::ElfPhysicalTypeLoad => {
                let pages: usize = ((phdr.p_memsz as usize) + PAGE_SIZE as usize - 1)  / PAGE_SIZE as usize;
                let kernel_mem = system_table.boot_services().allocate_pages::<core::ffi::c_void>(r_efi::system::LOADER_DATA, pages)?;

                kernel_file.set_position(phdr.p_offset)?;
                let mut psize = phdr.p_filesz as usize;
                kernel_file.read(&mut psize, kernel_mem)?;
                kernel_asset_list.add_asset(PhysicalAddress::new(kernel_mem as u64), pages, VirtualAddress::new(phdr.p_vaddr));
            },
            _ => {}
        }
    }

    return Ok((kernel_asset_list, VirtualAddress::new(elf_64.e_entry)));
}

fn init_page_table_manager(mut allocator: &mut PageFrameAllocator, max_physical_address: PhysicalAddress, kernel_base_address: VirtualAddress) -> Option<(PageTableManager, u64)> {
    if max_physical_address.as_u64() > MAX_MEM_SIZE {
        return None;
    }
    let page_table_manager = PageTableManager::new_from_allocator(allocator, 0);

    //Identity map the entire memory range
    let num_mem_pages = max_physical_address.as_u64() / PAGE_SIZE;
    page_table_manager.map_memory_pages(VirtualAddress::new(0), PhysicalAddress::new(0), num_mem_pages, allocator);

    //The size of the address space set aside in GB
    let num_gb = (max_physical_address.as_u64() + MEM_1G - 1) / MEM_1G;
    //Map the memory before the kernel
    let offset = kernel_base_address.as_u64() - num_gb * MEM_1G;

    page_table_manager.map_memory_pages(VirtualAddress::new(offset), PhysicalAddress::new(0), num_mem_pages, allocator);

    return Some((page_table_manager, offset));
}

fn validate_elf(header: &elf::ElfHeaderCommon) -> Result<(),efi::Status> {
    if !header.valid_magic() {
        com1_println!("Invalid magic");
        return Err(efi::Status::LOAD_ERROR);
    }

    if header.class() != elf::ElfClass::ElfClass64 {
        com1_println!("Invalid class {:?}", header.class());
        return Err(efi::Status::LOAD_ERROR);
    }

    if header.data_encoding() != elf::ElfData::ElfData2Lsb {
        com1_println!("Invalid data encoding {:?}", header.data_encoding());
        return Err(efi::Status::LOAD_ERROR);
    }

    if header.e_type() != elf::ElfType::ElfTypeExec {
        com1_println!("Invalid type {:?}", header.e_type());
        return Err(efi::Status::LOAD_ERROR);
    }

    if header.e_machine() != elf::ElfMachine::ElfMachineX8664 {
        com1_println!("Invalid machine {:?}", header.e_machine());
        return Err(efi::Status::LOAD_ERROR);
    }

    if header.e_version() != elf::ElfVersion::ElfVersionCurrent {
        com1_println!("Invalid elf version {:?}", header.e_version());
        return Err(efi::Status::LOAD_ERROR);
    }
    
    return Ok(());
}

fn initialise_gop(system_table: uefi::SystemTableWrapper) -> Result<bootinfo::FrameBuffer, efi::Status>{
    let gop = match system_table.boot_services().get_graphics_output_protocol() {
        Ok(gop) => gop,
        Err(s) => { 
            com1_println!("Cannot load GOP. Status{}", s.as_usize());
            return Err(s);
        }
    };
    com1_println!("GOP loaded");

    return Ok(gop.get_framebuffer());
}
