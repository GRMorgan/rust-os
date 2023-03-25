use core::{sync::atomic::{AtomicU16, Ordering}, cell::UnsafeCell};

pub struct RingBuffer<T: Copy, const N: usize> {
    buffer: UnsafeCell<[T;N]>,
    buffer_size: usize,
    read_pos: AtomicU16,
    write_pos: UnsafeCell<u16>,
    atom_write_post: AtomicU16,
}

unsafe impl<T: Copy + Send, const N: usize> Sync for RingBuffer<T,N> {}
unsafe impl<T: Copy + Send, const N: usize> Send for RingBuffer<T,N> {}

impl<T: Copy, const N: usize> RingBuffer<T,N> {
    /// Initialise a *RingBuffer* where the backing array are all initialised to the value
    /// *init_value*
    pub const fn new(init_value: T) -> RingBuffer<T,N> {
        return RingBuffer {
            buffer: UnsafeCell::new([init_value;N]),
            buffer_size: N,
            read_pos: AtomicU16::new(0),
            write_pos: UnsafeCell::new(0),
            atom_write_post: AtomicU16::new(0),
        };
    }
    
    pub fn is_empty(&self) -> bool {
        return self.read_pos.load(Ordering::Relaxed) == self.write_pos();
    }

    pub fn is_full(&self) -> bool {
        let mut read_pos = self.read_pos.load(Ordering::Relaxed);
        if read_pos == 0 {
            read_pos = self.buffer_size as u16;
        }

        return self.write_pos() + 1 == read_pos;
    }

    #[inline]
    pub fn num_items(&self) -> u16 {
        let read_pos = self.read_pos.load(Ordering::Relaxed);
        let mut write_pos = self.write_pos();
        if read_pos > write_pos {
            write_pos += self.buffer_size as u16;
        }
        return write_pos - read_pos;
    }

    #[inline]
    pub fn max_items(&self) -> u16 {
        return self.buffer_size as u16 - 1;
    }

    #[inline]
    pub fn remaining_space(&self) -> u16 {
        return self.max_items() - self.num_items();
    }

    #[inline]
    fn write_pos(&self) -> u16 {
        unsafe{ *self.write_pos.get() }
    }

    fn read_val(&self, pos: usize) -> T {
        unsafe {
            let t = self.buffer.get();
            return (*t)[pos];
        }
    }

    fn set_val(&self, pos: usize, value: T) {
        unsafe {
            let t = self.buffer.get();
            (*t)[pos] = value;
        }
    }

    pub fn read(&self) -> Option<T> {
        loop {
            if self.is_empty() {
                return None;
            }

            let read_pos = self.read_pos.load(Ordering::Relaxed);
            let output = self.read_val(read_pos as usize);
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

    pub fn write(&self, item: T) -> Option<u16> {
        loop {
            let cur_write_pos = self.write_pos();
            if self.is_full() {
                return None;
            }

            let mut new_write_pos = cur_write_pos + 1;
            if new_write_pos == self.buffer_size as u16 {
                new_write_pos = 0;
            }

            let cas_result = self.atom_write_post.compare_exchange(cur_write_pos, new_write_pos, Ordering::Relaxed, Ordering::Relaxed);

            if cas_result.is_ok() {
                self.set_val(cur_write_pos as usize, item);
                unsafe { *self.write_pos.get() = new_write_pos; }
                return Some(new_write_pos);
            }
        }
    }
}