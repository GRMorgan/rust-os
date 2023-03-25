#[cfg(test)]
mod tests {
    use data_structures::ringbuffer::RingBuffer;

    #[test]
    fn test_none_on_init() {
        let mut buffer: [u64; 512] = [0;512];
        let ringbuffer = unsafe { RingBuffer::new(buffer.as_mut_ptr(), 4096) };
        let expected_none = ringbuffer.read();
        
        assert_eq!(None, expected_none);
        assert_eq!(true, ringbuffer.is_empty());
        assert_eq!(0, ringbuffer.num_items());
    }

    #[test]
    fn test_add_1_item_1() {
        let mut buffer: [u64; 512] = [0;512];
        let ringbuffer = unsafe { RingBuffer::new(buffer.as_mut_ptr(), 4096) };
        
        let expected_value: u64 = 5;
        ringbuffer.write(expected_value);
        assert_eq!(1, ringbuffer.num_items());
        assert_eq!(false, ringbuffer.is_empty());
        let read_value = ringbuffer.read();
        assert_eq!(true, ringbuffer.is_empty());
        assert_eq!(Some(expected_value), read_value);
        assert_eq!(0, ringbuffer.num_items());
    }

    #[test]
    fn test_add_1_item_2() {
        let mut buffer: [u64; 512] = [0;512];
        let ringbuffer = unsafe { RingBuffer::new(buffer.as_mut_ptr(), 4096) };
        
        let expected_value: u64 = 7;
        ringbuffer.write(expected_value);
        assert_eq!(1, ringbuffer.num_items());
        assert_eq!(false, ringbuffer.is_empty());
        let read_value = ringbuffer.read();
        assert_eq!(true, ringbuffer.is_empty());
        assert_eq!(Some(expected_value), read_value);
        assert_eq!(0, ringbuffer.num_items());
    }

    #[test]
    fn test_add_2_items_1() {
        let mut buffer: [u64; 512] = [0;512];
        let ringbuffer = unsafe { RingBuffer::new(buffer.as_mut_ptr(), 4096) };
        
        let expected_value1: u64 = 5;
        let expected_value2: u64 = 7;

        ringbuffer.write(expected_value1);
        ringbuffer.write(expected_value2);
        assert_eq!(2, ringbuffer.num_items());
        assert_eq!(false, ringbuffer.is_empty());
        let read_value1 = ringbuffer.read();
        assert_eq!(false, ringbuffer.is_empty());
        assert_eq!(Some(expected_value1), read_value1);
        assert_eq!(1, ringbuffer.num_items());
        let read_value2 = ringbuffer.read();
        assert_eq!(true, ringbuffer.is_empty());
        assert_eq!(Some(expected_value2), read_value2);
        assert_eq!(0, ringbuffer.num_items());
    }

    #[test]
    fn test_add_2_items_2() {
        let mut buffer: [u64; 512] = [0;512];
        let ringbuffer = unsafe { RingBuffer::new(buffer.as_mut_ptr(), 4096) };
        
        let expected_value1: u64 = 89;
        let expected_value2: u64 = 21;

        ringbuffer.write(expected_value1);
        ringbuffer.write(expected_value2);
        assert_eq!(2, ringbuffer.num_items());
        assert_eq!(false, ringbuffer.is_empty());
        let read_value1 = ringbuffer.read();
        assert_eq!(false, ringbuffer.is_empty());
        assert_eq!(Some(expected_value1), read_value1);
        assert_eq!(1, ringbuffer.num_items());
        let read_value2 = ringbuffer.read();
        assert_eq!(true, ringbuffer.is_empty());
        assert_eq!(Some(expected_value2), read_value2);
        assert_eq!(0, ringbuffer.num_items());
    }

    #[test]
    fn test_add_2_items_remove_add_item_1() {
        let mut buffer: [u64; 512] = [0;512];
        let ringbuffer = unsafe { RingBuffer::new(buffer.as_mut_ptr(), 4096) };
        
        let expected_value1: u64 = 5;
        let expected_value2: u64 = 7;
        let expected_value3: u64 = 39027;

        ringbuffer.write(expected_value1);
        ringbuffer.write(expected_value2);
        assert_eq!(false, ringbuffer.is_empty());
        let read_value1 = ringbuffer.read();
        assert_eq!(false, ringbuffer.is_empty());
        assert_eq!(Some(expected_value1), read_value1);
        let read_value2 = ringbuffer.read();
        assert_eq!(true, ringbuffer.is_empty());
        assert_eq!(Some(expected_value2), read_value2);

        ringbuffer.write(expected_value3);
        assert_eq!(false, ringbuffer.is_empty());
        let read_value3 = ringbuffer.read();
        assert_eq!(true, ringbuffer.is_empty());
        assert_eq!(Some(expected_value3), read_value3);
    }

    #[test]
    fn test_max_capacity() {
        let mut buffer: [u64;512] = [0;512];
        let ringbuffer = unsafe { RingBuffer::new(buffer.as_mut_ptr(), 4096) };

        for i in 0..511u64 {
            let response = ringbuffer.write(i);
            assert_eq!(true, response.is_some());
        }

        assert_eq!(true, ringbuffer.is_full());
        let response = ringbuffer.write(511);
        assert_eq!(None, response);
    }

    #[test]
    fn test_max_capacity2() {
        let mut buffer: [u64;1024] = [0;1024];
        let ringbuffer = unsafe { RingBuffer::new(buffer.as_mut_ptr(), 8192) };

        for i in 0..1023u64 {
            let response = ringbuffer.write(i);
            assert_eq!(true, response.is_some());
        }
        
        assert_eq!(true, ringbuffer.is_full());
        let response = ringbuffer.write(1023);
        assert_eq!(None, response);
    }

    static mut BUFFER_ARRAY: [u64;512] = [0;512];
    static RING_BUFFER: RingBuffer<u64> = unsafe { RingBuffer::new_uninit() };
    static mut VEC1: std::vec::Vec<u64> = Vec::new();
    static mut VEC2: std::vec::Vec<u64> = Vec::new();
    static mut VEC3: std::vec::Vec<u64> = Vec::new();
    static mut VEC4: std::vec::Vec<u64> = Vec::new();

    use rand::Rng;

    #[test]
    pub fn test_threaded_access() {
        unsafe { RING_BUFFER.init(BUFFER_ARRAY.as_mut_ptr(), 4096) };

        let t1 = std::thread::spawn(move || {
            let mut vec = test_buffer(0, 512, 100000);
            unsafe {VEC1.append( &mut vec);}
        });

        let t2 = std::thread::spawn(move || {
            let mut vec = test_buffer(512, 512, 100000);
            unsafe {VEC2.append( &mut vec);}
        });

        let t3 = std::thread::spawn(move || {
            let mut vec = test_buffer(1024, 512, 100000);
            unsafe {VEC3.append( &mut vec);}
        });

        let t4 = std::thread::spawn(move || {
            let mut vec = test_buffer(1536, 512, 100000);
            unsafe {VEC4.append(&mut vec);}
        });

        t1.join().unwrap();
        t2.join().unwrap();
        t3.join().unwrap();
        t4.join().unwrap();

        let mut vec_test: std::vec::Vec<u64> = Vec::new();
        unsafe { 
            vec_test.append(&mut VEC1);
            vec_test.append(&mut VEC2);
            vec_test.append(&mut VEC3);
            vec_test.append(&mut VEC4);

            assert_eq!(false, RING_BUFFER.is_empty());
            while !RING_BUFFER.is_empty() {
                match RING_BUFFER.read() {
                    Some(val) => { vec_test.push(val); }
                    _ => {}
                }
            }
        }

        vec_test.sort();
        
        assert_eq!(2048, vec_test.len());

        for i in 0..2048 {
            assert_eq!(i  as u64, vec_test[i]);
        }
    }

    fn test_buffer(start_index: u64, num_items: u64, iter_count: usize) -> std::vec::Vec<u64>  {
        let mut vec: std::vec::Vec<u64> = Vec::new();
        for i in start_index..(start_index + num_items) {
            vec.push(i);
        }

        for _ in 0..iter_count {
            if vec.len() > 0 {
                let index = rand::thread_rng().gen_range(0..vec.len());
                let val = vec.remove(index);
                let result = RING_BUFFER.write(val);

                if result.is_none() {
                    vec.push(val);
                    match RING_BUFFER.read() {
                        Some(val) => {vec.push(val); },
                        _ => {},
                    }
                    match RING_BUFFER.read() {
                        Some(val) => {vec.push(val); },
                        _ => {},
                    }
                    match RING_BUFFER.read() {
                        Some(val) => {vec.push(val); },
                        _ => {},
                    }
                    match RING_BUFFER.read() {
                        Some(val) => {vec.push(val); },
                        _ => {},
                    }
                }
            } else {
                match RING_BUFFER.read() {
                    Some(val) => {vec.push(val); },
                    _ => {},
                }
            }
        }

        return vec;
    }
}