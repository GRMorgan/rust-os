#[repr(C)]
#[derive(Clone, Copy)]
pub struct Bitmap {
    size: usize,
    buffer: *mut BitmapByte
}

impl Bitmap {
    pub const unsafe fn new_uninit() -> Bitmap {
        return Bitmap { 
            size: 0,
            buffer: core::ptr::null_mut::<BitmapByte>(),
        };
    }

    //This is unsafe as it is possible to pass invalid values to the constructor.
    //It is the responsibility of the caller to ensure this is safe.
    pub unsafe fn new(size: usize, buffer: *mut u8) -> Bitmap {
        return Bitmap {
            size: size,
            buffer: buffer as *mut BitmapByte,
        }
    }

    pub unsafe fn new_init_zero(size: usize, buffer: *mut u8) -> Bitmap {
        return Self::new_init(size, buffer, 0);
    }

    pub unsafe fn new_init(size: usize, buffer: *mut u8, default_value: u8) -> Bitmap {
        let mut output = Bitmap {
            size: size,
            buffer: buffer as *mut BitmapByte,
        };

        for i in 0..output.size {
            output.set_byte(i, default_value);
        }

        return output;
    }

    pub fn get(&self, index: usize) -> bool {
        let buffer_index = (index / 8) as isize;
        let bit_index = index % 8;

        if buffer_index as usize >= self.size {
            return false; 
        } else {
            unsafe {
                return (*self.buffer.offset(buffer_index)).get(bit_index);
            }
        }
    }

    pub fn set(&mut self, index: usize, value: bool) -> bool {
        let buffer_index = (index / 8) as isize;
        let bit_index = index % 8;

        if buffer_index as usize >= self.size { 
            return false; 
        } else {
            unsafe {
                (*self.buffer.offset(buffer_index)).set(bit_index, value);
            }
            return true;
        }
    }

    /// Returns the buffer as a u8 pointer.
    /// 
    /// ## Safety
    /// 
    /// This is unsafe as the caller could store a permanent reference to the bitmap
    /// buffer. The caller must ensure that this bitmap is being discarded after
    /// calling this or that the pointer is not stored
    pub unsafe fn get_buffer(&self) -> *mut u8 {
        return self.buffer as *mut u8;
    }

    /// Sets the buffer
    /// 
    /// ## Safety
    /// 
    /// This is unsafe as the caller could pass an invalid memory block or pass one
    /// that is too smaller. The caller must ensure the passed in pointer is:
    /// 1. Only used by this Bitmap
    /// 2. Is a valid pointer
    /// 3. Points to a memory block large enough for the Bitmap
    pub unsafe fn set_buffer(&mut self, buffer: *mut u8) {
        self.buffer = buffer as *mut BitmapByte;
    }

    pub fn size(&self) -> usize {
        return self.size;
    }

    fn set_byte(&mut self, index: usize, byte: u8) {
        if index > self.size {
            return;
        } else {
            unsafe {
                (*self.buffer.offset(index as isize)).set_byte(byte);
            }
        }
    }
}

unsafe impl Send for Bitmap {}

#[repr(transparent)]
struct BitmapByte {
    pub byte: u8,
}

impl BitmapByte {
    pub fn get(&self, index: usize) -> bool {
        if index > 7 { 
            return false;
        } else {
            let bit_indexer = 0b10000000u8 >> index;
            return self.byte & bit_indexer > 0; 
        }
    }

    pub fn set(&mut self, index: usize, value: bool) {
        if index > 7 { 
            return;
        } else {
            let bit_indexer = 0b10000000u8 >> index;
            self.byte &= !bit_indexer;
            if value {
                self.byte |= bit_indexer;
            }
        }
    }

    pub fn set_byte(&mut self, byte: u8) {
        self.byte = byte;
    }
}
