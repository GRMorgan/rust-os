use core::sync::atomic::{AtomicU16, Ordering};

pub struct RingBuffer<T: Copy> {
    buffer: *mut T,
    buffer_size: usize,
    read_pos: AtomicU16,
    write_pos: u16,
    atom_write_post: AtomicU16,
}

impl<T: Copy> RingBuffer<T> {
    pub unsafe fn new(buffer: *mut T, mem_size: usize) -> RingBuffer<T> {
        let buffer_size: usize = mem_size / core::mem::size_of::<T>();
        return RingBuffer {
            buffer: buffer,
            buffer_size: buffer_size,
            read_pos: AtomicU16::new(0),
            write_pos: 0,
            atom_write_post: AtomicU16::new(0),
        };
    }
    
    pub fn is_empty(&self) -> bool {
        return self.read_pos.load(Ordering::Relaxed) == self.write_pos;
    }

    pub fn is_full(&self) -> bool {
        let mut read_pos = self.read_pos.load(Ordering::Relaxed);
        if read_pos == 0 {
            read_pos = self.buffer_size as u16;
        }

        return self.write_pos + 1 == read_pos;
    }

    fn read_val(&self, pos: u16) -> T {
        unsafe {
            let read_ptr = self.buffer.offset(pos as isize);
            return *read_ptr;
        }
    }

    fn set_val(&self, pos: u16, value: T) {
        unsafe {
            let read_ptr = self.buffer.offset(pos as isize);
            *read_ptr = value;
        }
    }

    pub fn read(&mut self) -> Option<T> {
        loop {
            if self.is_empty() {
                return None;
            }

            let read_pos = self.read_pos.load(Ordering::Relaxed);
            let output = self.read_val(read_pos);
            let mut new_read_pos = read_pos + 1;
            if new_read_pos == self.buffer_size as u16 {
                new_read_pos = 0;
            }
            let cas_result = self.read_pos.compare_exchange(read_pos, new_read_pos, Ordering::Relaxed, Ordering::Relaxed);

            if cas_result.is_ok() {
                return Some(output);
            }
        }
    }

    pub fn write(&mut self, item: T) -> Option<u16> {
        loop {
            if self.is_full() {
                return None;
            }

            let mut new_write_pos = self.write_pos + 1;
            if new_write_pos == self.buffer_size as u16 {
                new_write_pos = 0;
            }

            let cas_result = self.atom_write_post.compare_exchange(self.write_pos, new_write_pos, Ordering::Relaxed, Ordering::Relaxed);

            if cas_result.is_ok() {
                self.set_val(self.write_pos, item);
                self.write_pos = new_write_pos;
                return Some(new_write_pos);
            }
        }
    }
}