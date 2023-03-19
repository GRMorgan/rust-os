use x86_64_hardware::memory::VirtualAddress;

#[repr(C)]
pub struct BootInfo {
    pub next_available_kernel_page: VirtualAddress,
}