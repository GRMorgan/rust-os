use r_efi::efi;
use r_efi::protocols::loaded_image;

pub struct LoadedImageProtocol {
    loaded_image_protocol_ptr: *mut loaded_image::Protocol,
}

impl LoadedImageProtocol {
    pub fn new(loaded_image_protocol_ptr: *mut loaded_image::Protocol) -> LoadedImageProtocol {
        return LoadedImageProtocol {
            loaded_image_protocol_ptr: loaded_image_protocol_ptr
        };
    }

    pub fn device_handle(self) -> efi::Handle {
        return unsafe {
            (*self.loaded_image_protocol_ptr).device_handle
        };
    }
}
