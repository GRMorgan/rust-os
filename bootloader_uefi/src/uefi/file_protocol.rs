use r_efi::efi;
use r_efi::protocols::file;

use crate::unicode::*;

pub struct FileProtocol {
    file_ptr: *mut file::Protocol,
    is_open: bool,
}

impl FileProtocol {
    pub fn new(file_ptr: *mut file::Protocol) -> FileProtocol {
        return FileProtocol {
            file_ptr: file_ptr,
            is_open: true,
        };
    }

    pub fn open(&self, path: &str, open_mode: u64, attributes: u64) -> Result<FileProtocol, efi::Status> {
        let mut output_ptr: *mut file::Protocol = core::ptr::null_mut::<file::Protocol>();
        let mut path_buffer: [u16;1024] = [0;1024];
        let input_len = path.as_bytes().len();
        
        let EncodeStatus {input_read, ..} = encode_str_as_ucs2(&path, 0, &mut path_buffer);
        
        if input_read < input_len {
            return Err(efi::Status::BUFFER_TOO_SMALL);
        }

        let s = unsafe {
            ((*self.file_ptr).open)(self.file_ptr, &mut output_ptr, path_buffer.as_ptr() as *mut efi::Char16, open_mode, attributes)
        };

        if s == efi::Status::SUCCESS {
            return Ok(FileProtocol::new(output_ptr));
        } else {
            return Err(s);
        }
    }

    pub fn read(&self, buffer_size: &mut usize, buffer: *mut core::ffi::c_void) -> Result<(), efi::Status> {
        let s = unsafe {
            ((*self.file_ptr).read)(self.file_ptr, buffer_size, buffer)
        };

        if s == efi::Status::SUCCESS {
            return Ok(());
        } else {
            return Err(s);
        }
    }

    pub fn close(&mut self) -> efi::Status {
        if self.is_open {
            let status = unsafe {
                ((*self.file_ptr).close)(self.file_ptr)
            };
            self.is_open = false;
            return status;
        } else {
            return efi::Status::SUCCESS;
        }
    }

    pub fn read_struct<T: Default>(&self) -> Result<T, efi::Status> {
        let mut size: usize = core::mem::size_of::<T>();
        let output: T = Default::default();
        self.read(&mut size, core::ptr::addr_of!(output) as *mut core::ffi::c_void)?;
        return Ok(output);
    }

    pub fn set_position(&self, pos: u64) -> Result<(), efi::Status> {
        let s = unsafe {
            ((*self.file_ptr).set_position)(self.file_ptr, pos)
        };

        if s == efi::Status::SUCCESS {
            return Ok(());
        } else {
            return Err(s);
        }
    }
}

impl Drop for FileProtocol {
    fn drop(&mut self) {
        self.close();
    }
}
