use r_efi::protocols::graphics_output;
use bootinfo::FrameBuffer;
use x86_64_hardware::memory::PhysicalAddress;

pub struct GraphicsOutputProtocol {
    graphics_output_protocol_ptr: *mut graphics_output::Protocol,
}

impl GraphicsOutputProtocol {
    pub fn new(graphics_output_protocol_ptr: *mut graphics_output::Protocol) -> GraphicsOutputProtocol {
        return GraphicsOutputProtocol {
            graphics_output_protocol_ptr: graphics_output_protocol_ptr
        };
    }

    pub fn get_framebuffer(&self) -> FrameBuffer {
        return FrameBuffer::new (
            self.mode().frame_buffer_base(),
            self.mode().frame_buffer_size(),
            self.mode().info().horizontal_resolution(),
            self.mode().info().vertical_resolution(),
            self.mode().info().pixels_per_scan_line(),
        );
    }

    fn mode(&self) -> GopMode {
        GopMode::new (
            unsafe {
                (*self.graphics_output_protocol_ptr).mode
            }
        )
    }
}

struct GopMode {
    mode_ptr: *mut graphics_output::Mode,
}

impl GopMode {
    pub fn new(mode_ptr: *mut graphics_output::Mode) -> GopMode {
        return GopMode {
            mode_ptr: mode_ptr
        };
    }

    pub fn frame_buffer_base(&self) -> PhysicalAddress {
        unsafe {
            return PhysicalAddress::new((*self.mode_ptr).frame_buffer_base as *mut core::ffi::c_void as u64);
        }
    }

    pub fn frame_buffer_size(&self) -> usize {
        unsafe {
            return (*self.mode_ptr).frame_buffer_size;
        }
    }

    pub fn info(&self) -> GopModeInfo {
        GopModeInfo::new (
            unsafe {
                (*self.mode_ptr).info
            }
        )
    }
}

struct GopModeInfo {
    info_ptr: *mut graphics_output::ModeInformation,
}

impl GopModeInfo {
    pub fn new(info_ptr: *mut graphics_output::ModeInformation) -> GopModeInfo {
        return GopModeInfo {
            info_ptr: info_ptr
        };
    }

    pub fn horizontal_resolution(&self) -> u32 {
        unsafe {
            return (*self.info_ptr).horizontal_resolution;
        }
    }

    pub fn vertical_resolution(&self) -> u32 {
        unsafe {
            return (*self.info_ptr).vertical_resolution;
        }
    }

    pub fn pixels_per_scan_line(&self) -> u32 {
        unsafe {
            return (*self.info_ptr).pixels_per_scan_line;
        }
    }
}
