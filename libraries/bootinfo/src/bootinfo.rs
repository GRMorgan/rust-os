use x86_64_hardware::memory::VirtualAddress;

#[repr(C)]
pub struct BootInfo {
    pub page_table_memory_offset: u64,
    pub next_available_kernel_page: VirtualAddress,
}