mod tests {
    use data_structures::ringbuffer::RingBuffer;

    #[test]
    fn test_none_on_init() {
        let mut buffer: [u64; 512] = [0;512];
        let mut ringbuffer = unsafe { RingBuffer::new(buffer.as_mut_ptr(), 4096) };
        let expected_none = ringbuffer.read();
        
        assert_eq!(None, expected_none);
        assert_eq!(true, ringbuffer.is_empty());
        assert_eq!(0, ringbuffer.num_items());
    }

    #[test]
    fn test_add_1_item_1() {
        let mut buffer: [u64; 512] = [0;512];
        let mut ringbuffer = unsafe { RingBuffer::new(buffer.as_mut_ptr(), 4096) };
        
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
        let mut ringbuffer = unsafe { RingBuffer::new(buffer.as_mut_ptr(), 4096) };
        
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
        let mut ringbuffer = unsafe { RingBuffer::new(buffer.as_mut_ptr(), 4096) };
        
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
        let mut ringbuffer = unsafe { RingBuffer::new(buffer.as_mut_ptr(), 4096) };
        
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
        let mut ringbuffer = unsafe { RingBuffer::new(buffer.as_mut_ptr(), 4096) };
        
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
}