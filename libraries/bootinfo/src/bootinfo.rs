use x86_64_hardware::memory::VirtualAddress;

use crate::{MemInfo, FrameBuffer};

//Randomly generated magic values. Replace with something fancy like the OS name once it has a name.
const BOOTINFO_MAGIC: [u8;4] = [15, 106, 86, 167];

#[repr(C)]
pub struct BootInfo {
    magic: [u8;4],
    pub framebuffer: FrameBuffer,
    pub page_table_memory_offset: u64,
    pub next_available_kernel_page: VirtualAddress,
    pub meminfo: MemInfo,
}

impl BootInfo {
    /// A simple sanity check to show we've passed the right thing to the kernel
    pub fn valid_magic(&self) -> bool {
        return self.magic == BOOTINFO_MAGIC;
    }
}

impl Default for BootInfo {
    fn default() -> BootInfo {
        BootInfo {
            magic: BOOTINFO_MAGIC,
            framebuffer: FrameBuffer::default(),
            page_table_memory_offset: 0,
            next_available_kernel_page: VirtualAddress::new(0),
            meminfo: MemInfo::default(),
        }   
    }
}