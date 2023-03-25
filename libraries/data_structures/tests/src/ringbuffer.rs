#[cfg(test)]
mod tests {
    use data_structures::ringbuffer::RingBuffer;

    #[test]
    fn test_none_on_init() {
        let ringbuffer = RingBuffer::<u64,512>::new(0);
        let expected_none = ringbuffer.read();
        
        assert_eq!(None, expected_none);
        assert_eq!(true, ringbuffer.is_empty());
        assert_eq!(0, ringbuffer.num_items());
    }

    #[test]
    fn test_add_1_item_1() {
        let ringbuffer = RingBuffer::<u64,512>::new(0);
        
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
        let ringbuffer = RingBuffer::<u64,512>::new(0);
        
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
        let ringbuffer = RingBuffer::<u64,512>::new(0);
        
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
        let ringbuffer = RingBuffer::<u64,512>::new(0);
        
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
        let ringbuffer = RingBuffer::<u64,512>::new(0);
        
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
        let ringbuffer = RingBuffer::<u64,512>::new(0);

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
        let ringbuffer = RingBuffer::<u64,1024>::new(0);

        for i in 0..1023u64 {
            let response = ringbuffer.write(i);
            assert_eq!(true, response.is_some());
        }
        
        assert_eq!(true, ringbuffer.is_full());
        let response = ringbuffer.write(1023);
        assert_eq!(None, response);
    }

    static RING_BUFFER: RingBuffer<u64, 512> = RingBuffer::new(0);
    use rand::Rng;

    const ITER_COUNT: usize = 100000;
    #[test]
    pub fn test_threaded_access_write_aggressive() {
        fn task(start_index: u64, num_items: u64, iter_count: usize) -> std::vec::Vec<u64>  {
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
                        for _ in 0..5 {
                            match RING_BUFFER.read() {
                                Some(val) => {vec.push(val); },
                                _ => {},
                            }
                        }
                    }
                } else {
                    for _ in 0..5 {
                        match RING_BUFFER.read() {
                            Some(val) => {vec.push(val); },
                            _ => {},
                        }
                    }
                }
            }
            return vec;
        }
        let t1 = std::thread::spawn(move || {
            return task(0, 512, ITER_COUNT);
        });

        let t2 = std::thread::spawn(move || {
            return task(512, 512, ITER_COUNT);
        });

        let t3 = std::thread::spawn(move || {
            return task(1024, 512, ITER_COUNT);
        });

        let t4 = std::thread::spawn(move || {
            return task(1536, 512, ITER_COUNT);
        });

        let mut vec1 = t1.join().unwrap();
        let mut vec2 = t2.join().unwrap();
        let mut vec3 = t3.join().unwrap();
        let mut vec4 = t4.join().unwrap();

        let mut vec_test: std::vec::Vec<u64> = Vec::new();
        vec_test.append(&mut vec1);
        vec_test.append(&mut vec2);
        vec_test.append(&mut vec3);
        vec_test.append(&mut vec4);

        assert_eq!(false, RING_BUFFER.is_empty());
        while !RING_BUFFER.is_empty() {
            match RING_BUFFER.read() {
                Some(val) => { vec_test.push(val); }
                _ => {}
            }
        }

        vec_test.sort();
        
        assert_eq!(2048, vec_test.len());

        for i in 0..2048 {
            assert_eq!(i  as u64, vec_test[i]);
        }
    }
    

    static RING_BUFFER2: RingBuffer<u64, 2048> = RingBuffer::new(0);

    const ITER_COUNT2: usize = 100000;
    #[test]
    pub fn test_threaded_access_read_aggressive() {
        for value in 0..RING_BUFFER2.remaining_space() as u64 {
            RING_BUFFER2.write(value);
        }
        fn task(iter_count: usize) -> std::vec::Vec<u64>  {
            let mut vec: std::vec::Vec<u64> = Vec::new();

            for _ in 0..iter_count {
                if RING_BUFFER2.is_empty() {
                    for _ in 0..25 {
                        if vec.len() > 0 {
                            let index = rand::thread_rng().gen_range(0..vec.len());
                            let val = vec.remove(index);
                            let result = RING_BUFFER2.write(val);
                            if result.is_none() {
                                vec.push(val);
                            }
                        }
                    }
                } else {
                    for _ in 0..5 {
                        match RING_BUFFER2.read() {
                            Some(val) => { vec.push(val); }
                            _ => {}
                        }
                    }
                }
            }
            return vec;
        }
        let t1 = std::thread::spawn(move || {
            return task(ITER_COUNT2);
        });

        let t2 = std::thread::spawn(move || {
            return task(ITER_COUNT2);
        });

        let t3 = std::thread::spawn(move || {
            return task(ITER_COUNT2);
        });

        let t4 = std::thread::spawn(move || {
            return task(ITER_COUNT2);
        });

        let mut vec1 = t1.join().unwrap();
        let mut vec2 = t2.join().unwrap();
        let mut vec3 = t3.join().unwrap();
        let mut vec4 = t4.join().unwrap();

        let mut vec_test: std::vec::Vec<u64> = Vec::new();
         vec_test.append(&mut vec1);
         vec_test.append(&mut vec2);
         vec_test.append(&mut vec3);
         vec_test.append(&mut vec4);

         while !RING_BUFFER2.is_empty() {
             match RING_BUFFER2.read() {
                 Some(val) => { vec_test.push(val); }
                 _ => {}
             }
         }

        vec_test.sort();
        
        assert_eq!(2047, vec_test.len());

        for i in 0..2047 {
            assert_eq!(i  as u64, vec_test[i]);
        }
    }
}