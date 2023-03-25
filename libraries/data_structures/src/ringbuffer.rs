use core::{sync::atomic::{AtomicU16, Ordering}, cell::UnsafeCell};

pub struct RingBuffer<T: Copy> {
    buffer: UnsafeCell<*mut T>,
    buffer_size: UnsafeCell<usize>,
    read_pos: AtomicU16,
    write_pos: UnsafeCell<u16>,
    atom_write_post: AtomicU16,
}

unsafe impl<T: Copy + Send> Sync for RingBuffer<T> {}
unsafe impl<T: Copy + Send> Send for RingBuffer<T> {}

impl<T: Copy> RingBuffer<T> {
    /// Create a *RingBuffer* uninitialised. The only reason to ever use this constructor is to
    /// setup a static variable. The *RingBuffer* must be initialised before it is used by any
    /// subsequent code
    /// 
    /// ## Safety
    /// 
    /// This is unsafe as nothing is set up. A subsequent call to write is not safe and will cause
    /// a memory error as it attempts to set a value to a null pointer. The caller must ensure that
    /// *init* has been called before any attempt to write the buffer is made
    pub const unsafe fn new_uninit() -> RingBuffer<T> {
        return RingBuffer {
            buffer: UnsafeCell::new(core::ptr::null_mut::<T>()),
            buffer_size: UnsafeCell::new(0),
            read_pos: AtomicU16::new(0),
            write_pos: UnsafeCell::new(0),
            atom_write_post: AtomicU16::new(0),
        };
    }

    /// Create a *RingBuffer* with the backing memory set by the *buffer* parameter. The *buffer*
    /// must be at least *mem_size* in length.
    /// 
    /// ## Safety
    /// 
    /// This is unsafe because the constructor cannot guarantee the *buffer* is not being shared
    /// somewhere else nor can it guarantee that *mem_size* is accurate. The caller must ensure
    /// that the *buffer* is not reused elsewhere and that it is at least *mem_size* bytes long.
    pub unsafe fn new(buffer: *mut T, mem_size: usize) -> RingBuffer<T> {
        let buffer_size: usize = mem_size / core::mem::size_of::<T>();
        return RingBuffer {
            buffer: UnsafeCell::new(buffer),
            buffer_size: UnsafeCell::new(buffer_size),
            read_pos: AtomicU16::new(0),
            write_pos: UnsafeCell::new(0),
            atom_write_post: AtomicU16::new(0),
        };
    }

    /// Initialise the ringbuffer. This is intended to be used with *new_uninit* to allow for a
    /// static ringbuffer to be created and then initialised with real values.
    /// 
    /// ## Safety
    /// 
    /// This is potentially unsafe because it overwrites the struct with a new buffer. This will
    /// have the following consequences:
    /// 1. Any item within the ringbuffer is lost
    /// 2. The memory pointed to by the buffer may be leaked
    /// The caller must ensure that either the ringbuffer is uninitialised or all the items have
    /// been retrieved and the buffer memory discarded prior to calling this
    pub unsafe fn init(&self, buffer: *mut T, mem_size: usize) {
        let buffer_size: usize = mem_size / core::mem::size_of::<T>();
        *self.buffer.get() = buffer;
        *self.buffer_size.get() = buffer_size;
        self.atom_write_post.store(0, Ordering::Relaxed);
        *self.write_pos.get() = 0;
        self.read_pos.store(0, Ordering::Relaxed);
    }
    
    pub fn is_empty(&self) -> bool {
        return self.read_pos.load(Ordering::Relaxed) == self.write_pos();
    }

    pub fn is_full(&self) -> bool {
        let mut read_pos = self.read_pos.load(Ordering::Relaxed);
        if read_pos == 0 {
            read_pos = self.buffer_size() as u16;
        }

        return self.write_pos() + 1 == read_pos;
    }

    #[inline]
    pub fn num_items(&self) -> u16 {
        let read_pos = self.read_pos.load(Ordering::Relaxed);
        let mut write_pos = self.write_pos();
        if read_pos > write_pos {
            write_pos += self.buffer_size() as u16;
        }
        return write_pos - read_pos;
    }

    #[inline]
    pub fn max_items(&self) -> u16 {
        return self.buffer_size() as u16 - 1;
    }

    #[inline]
    pub fn remaining_space(&self) -> u16 {
        return self.max_items() - self.num_items();
    }

    #[inline]
    fn write_pos(&self) -> u16 {
        unsafe{ *self.write_pos.get() }
    }

    #[inline]
    fn buffer(&self) -> *mut T {
        unsafe { return *self.buffer.get(); }
    }

    #[inline]
    fn buffer_size(&self) -> usize {
        unsafe { return *self.buffer_size.get(); }
    }

    fn read_val(&self, pos: u16) -> T {
        unsafe {
            let read_ptr = self.buffer().offset(pos as isize);
            return *read_ptr;
        }
    }

    fn set_val(&self, pos: u16, value: T) {
        unsafe {
            let read_ptr = self.buffer().offset(pos as isize);
            *read_ptr = value;
        }
    }

    pub fn read(&self) -> Option<T> {
        loop {
            if self.is_empty() {
                return None;
            }

            let read_pos = self.read_pos.load(Ordering::Relaxed);
            let output = self.read_val(read_pos);
            let mut new_read_pos = read_pos + 1;
            if new_read_pos == self.buffer_size() as u16 {
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
            if new_write_pos == self.buffer_size() as u16 {
                new_write_pos = 0;
            }

            let cas_result = self.atom_write_post.compare_exchange(cur_write_pos, new_write_pos, Ordering::Relaxed, Ordering::Relaxed);

            if cas_result.is_ok() {
                self.set_val(cur_write_pos, item);
                unsafe { *self.write_pos.get() = new_write_pos; }
                return Some(new_write_pos);
            }
        }
    }
}