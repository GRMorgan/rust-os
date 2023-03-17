#[repr(C)]
#[derive(Clone, Copy)]
pub struct Bitmap {
    size: usize,
    buffer: *mut BitmapByte
}

impl Bitmap {
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
