use std::io;

fn encode(encoding_size: u8, reader: &mut dyn io::Read, writer: &mut dyn io::Write) -> std::io::Result<usize> {
    let mut input_buffer = [0u8];
    let mut processed_bytes: usize = 0;

    let mut output_buffer = [0u8];
    let mut output_buffer_size = 8u8;
    let mut input_buffer_size = 0;

    loop {
        if input_buffer_size == 0 {
            let size = reader.read(&mut input_buffer)?;
            if size == 0 {
                //no more data to read
                break;
            }

            processed_bytes += size;
            input_buffer_size = encoding_size;
        }

        let mask_size = std::cmp::min(input_buffer_size, output_buffer_size);
        let mask_shift = encoding_size - mask_size;
        let mask: u8 = ((1 << (mask_size)) - 1) << mask_shift;
        output_buffer[0] = output_buffer[0] | (((input_buffer[0] & mask) >> mask_shift) << (output_buffer_size - mask_size)); 

        input_buffer_size -= mask_size;
        output_buffer_size -= mask_size;
        input_buffer[0] <<= mask_size;
        if output_buffer_size == 0 {
            writer.write(&mut output_buffer)?;
            output_buffer_size = 8u8;
            output_buffer[0] = 0u8;
        }
    }

    if output_buffer_size > 0 {
        writer.write(&mut output_buffer)?;
    }

    Ok(processed_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_5_length_one_byte() {
        let input_byte: u8 = (1 << 7) | 7u8;
        let input = [input_byte];
        let mut output = [0u8; 1];
        let result = encode(5, &mut &input[..], &mut &mut output[..]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
        assert_eq!(output[0], 0b00111000u8);

        let input_byte: u8 = (1 << 7) | 8u8;
        let input = [input_byte];
        let mut output = [0u8; 1];
        let result = encode(5, &mut &input[..], &mut &mut output[..]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
        assert_eq!(output[0], 0b01000000u8);
    }

    #[test]
    fn test_3_length_one_byte() {
        let input_byte: u8 = (1 << 7) | 7u8;
        let input = [input_byte];
        let mut output = [0u8; 1];
        let result = encode(3, &mut &input[..], &mut &mut output[..]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
        assert_eq!(output[0], 0b11100000u8);

        let input_byte: u8 = (1 << 7) | 8u8;
        let input = [input_byte];
        let mut output = [0u8; 1];
        let result = encode(3, &mut &input[..], &mut &mut output[..]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
        assert_eq!(output[0], 0b00000000u8);
    }

    #[test]
    fn test_encode_3_length_sequence() {
        let input = [7u8, 1, 2, 4, 7, 7, 7, 1, 1, 1, 2, 3, 4];
        let mut output = [0u8; 5]; 

        //second output byte is to be checked
        let result = encode(3, &mut &input[..], &mut &mut output[..]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 13);
        assert_eq!(output[0], 0b11100101u8);
        assert_eq!(output[1], 0b01001111u8); 
        assert_eq!(output[2], 0b11111001u8); 
        assert_eq!(output[3], 0b00100101u8); 
        assert_eq!(output[4], 0b00111000u8); 
    }

    #[test]
    fn test_encode_5_length_sequence() {
        let input = [2u8, 0x17, 0x16, 0x1F];
        let mut output = [0u8; 3]; 

        //second output byte is to be checked
        let result = encode(5, &mut &input[..], &mut &mut output[..]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 4);
        assert_eq!(output[0], 0b00010101u8);
        assert_eq!(output[1], 0b11101101u8); 
        assert_eq!(output[2], 0b11110000u8); 
    }
}
