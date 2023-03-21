use x86_64_hardware::memory::PhysicalAddress;

#[repr(C)]
pub struct FrameBuffer {
    pub base_address: PhysicalAddress,
    pub buffer_size: usize,
    pub width: u32,
    pub height: u32,
    pub pixels_per_scan_line: u32,
}

impl FrameBuffer {
    pub fn new(base_address: PhysicalAddress, buffer_size: usize, width: u32, height: u32, pixels_per_scan_line: u32) -> FrameBuffer {
        FrameBuffer {
            base_address: base_address,
            buffer_size: buffer_size,
            width: width,
            height: height,
            pixels_per_scan_line: pixels_per_scan_line,
        }
    }

    /// Clears the framebuffer
    /// 
    /// ## Safety
    /// 
    /// This is unsafe as it makes an assumption that the virtual address is at a simple offset to the physical address.
    /// The caller must assure this is the case before making this call.
    pub unsafe fn clear_framebuffer(&self, colour: u32, memory_offset: u64) {
        let virt_addr = self.base_address.get_virtual_address_at_offset(memory_offset);
        let base_pixel: *mut u32 = virt_addr.get_mut_ptr::<u32>();
        for x_pos in 0..self.width {
            for y_pos in 0..self.height {
                unsafe {
                    let pixel: *mut u32 = base_pixel.add((y_pos * self.pixels_per_scan_line + x_pos) as usize);
                    (*pixel) = colour;
                }
            }
        }
    }
}

impl Default for FrameBuffer {
    fn default() -> Self {
        FrameBuffer {
            base_address: PhysicalAddress::new(0),
            buffer_size: 0,
            width: 0,
            height: 0,
            pixels_per_scan_line: 0,
        }
    }
}