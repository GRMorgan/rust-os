pub struct EncodeStatus
{
    pub input_read: usize,
    pub output_written: usize,
}

const UNREADABLE_CHAR_REPLACEMENT: u16 = 0x2588u16;

pub fn encode_str_as_ucs2(input: &str, start_pos: usize, buffer: &mut [u16]) -> EncodeStatus {
    let input_bytes = input.as_bytes();
    let input_len = input_bytes.len();
    let mut current_pos: usize = start_pos;
    let buffer_max_pos = buffer.len() - 1; //Last character must be a null
    let mut buffer_pos: usize = 0;

    while current_pos < input_len && buffer_pos < buffer_max_pos {
        match encode_u8_char_as_ucs2(&input_bytes, current_pos)
        {
            EncodeCharStatus::EncodedChar{ encoded_char, input_read } => {
                buffer[buffer_pos] = encoded_char;
                buffer_pos += 1;
                current_pos += input_read;
            },
            EncodeCharStatus::InsufficientInputBytes { input_read } => {
                buffer[buffer_pos] = UNREADABLE_CHAR_REPLACEMENT;
                buffer_pos += 1;
                current_pos += input_read;
            },
            EncodeCharStatus::SurrogatePair { input_read } => {
                buffer[buffer_pos] = UNREADABLE_CHAR_REPLACEMENT;
                buffer_pos += 1;
                current_pos += input_read;
            }
        }
    }

    let mut null_pos = buffer_max_pos;
    if buffer_pos < buffer_max_pos {
        null_pos = buffer_pos;
    }

    buffer[null_pos] = 0u16;

    return EncodeStatus { input_read: current_pos, output_written: buffer_pos}
}

enum EncodeCharStatus {
    EncodedChar {
        encoded_char: u16,
        input_read: usize,
    },
    SurrogatePair {
        input_read: usize
    },
    InsufficientInputBytes {
        input_read: usize
    },
}

const HIGH_BIT_MASK: u8 = 0b10000000u8;
const HIGH_TWO_BIT_MASK: u8 = 0b11000000u8;
const HIGH_THREE_BIT_MASK: u8 = 0b11100000u8;
const HIGH_FOUR_BIT_MASK: u8 = 0b11110000u8;
const LOW_FIVE_BIT_MASK: u8 = 0b00011111u8;
const LOW_SIX_BIT_MASK: u8 = 0b00111111u8;

//TODO: Add handling of invalid chars like subsequent chars that don't start 10
/// Read a single ucs2 character from a u8 array starting at a given position.
fn encode_u8_char_as_ucs2(input: &[u8], start_pos: usize) -> EncodeCharStatus {
    let first_byte = input[start_pos];

    if first_byte & HIGH_BIT_MASK == 0u8 {
        //Single byte chars have a high bit of 0 and convert directly to u16
        return EncodeCharStatus::EncodedChar {
            encoded_char: u16::from(first_byte),
            input_read: 1,
        };
    } else if first_byte & HIGH_THREE_BIT_MASK == HIGH_TWO_BIT_MASK {
        //Two byte chars have high bits 110
        if start_pos + 1 >= input.len() {
            //No more bytes left but expected 1 more. Should be impossible
            return EncodeCharStatus::InsufficientInputBytes { input_read: 1 };
        }
        let second_byte = input[start_pos + 1];

        let out_char: u16 = u16::from(first_byte & LOW_FIVE_BIT_MASK) << 6 | u16::from(second_byte & LOW_SIX_BIT_MASK);
        return EncodeCharStatus::EncodedChar {
            encoded_char: out_char,
            input_read: 2,
        };
    } else if first_byte & HIGH_FOUR_BIT_MASK == HIGH_THREE_BIT_MASK {
        if start_pos + 1 >= input.len() {
            //No more bytes left but expected 2 more. Should be impossible
            return EncodeCharStatus::InsufficientInputBytes { input_read: 1 };
        }
        if start_pos + 2 >= input.len() {
            //No more bytes left but expected 1 more. Should be impossible
            return EncodeCharStatus::InsufficientInputBytes { input_read: 2 };
        }
        let second_byte = input[start_pos + 1];
        let third_byte = input[start_pos + 2];

        let out_char: u16 = u16::from(first_byte & LOW_FIVE_BIT_MASK) << 12 |
                            u16::from(second_byte & LOW_SIX_BIT_MASK) << 6 |
                            u16::from(third_byte & LOW_SIX_BIT_MASK);
        return EncodeCharStatus::EncodedChar {
            encoded_char: out_char,
            input_read: 3,
        };
    } else {
        if start_pos + 1 >= input.len() {
            //No more bytes left but expected 3 more. Should be impossible
            return EncodeCharStatus::InsufficientInputBytes { input_read: 1 };
        }
        if start_pos + 2 >= input.len() {
            //No more bytes left but expected 2 more. Should be impossible
            return EncodeCharStatus::InsufficientInputBytes { input_read: 2 };
        }
        if start_pos + 3 >= input.len() {
            //No more bytes left but expected 1 more. Should be impossible
            return EncodeCharStatus::InsufficientInputBytes { input_read: 3 };
        }
        return EncodeCharStatus::SurrogatePair { input_read: 4 };
    }
}
