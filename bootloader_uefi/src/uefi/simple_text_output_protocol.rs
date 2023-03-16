use r_efi::efi;
use r_efi::protocols::simple_text_output;
use core::fmt;

use crate::unicode::*;

pub struct SimpleTextOutputProtocol {
    output_ptr: *mut simple_text_output::Protocol,
}

impl SimpleTextOutputProtocol {
    pub fn new(output_ptr: *mut simple_text_output::Protocol) -> SimpleTextOutputProtocol {
        return SimpleTextOutputProtocol {
            output_ptr: output_ptr
        };
    }

    pub fn output_string(&self, text: &str)  -> efi::Status {
        let input_len = text.as_bytes().len();
        let mut buffer: [u16;256] = [0;256];
        let mut start_pos: usize = 0;

        loop {
            let EncodeStatus {input_read, ..} = encode_str_as_ucs2(&text, start_pos, &mut buffer);
            let status = self.output_string_arr(&buffer);
            
            if status.is_error() {
                return status;
            }

            if input_read >= input_len
            {
                return status;
            }
            else
            {
                start_pos += input_read;
            }
        }
    }

    fn output_string_arr(&self, text: &[u16]) -> efi::Status {
        return self.output_string_ptr(text.as_ptr());
    }

    fn output_string_ptr(&self, text: *const u16) -> efi::Status {
        let r = unsafe {
            ((*self.output_ptr).output_string)(self.output_ptr, text as *mut efi::Char16)
        };
        return r;
    }
}

impl fmt::Write for SimpleTextOutputProtocol {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.output_string(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! efi_write {
    ($dst:expr, $($arg:tt)*) => {
        $dst.write_fmt(format_args!($($arg)*)).unwrap()
    };
}
