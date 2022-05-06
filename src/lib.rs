use std::io;

fn encode(reader: &mut dyn io::Read, writer: &mut dyn io::Write) -> std::io::Result<usize> {
    let mut input_buffer = [0u8];
    let mut processed_bytes: usize = 0;

    let encoding_size = 5;

    let mut output_buffer = [0u8];
    let mut output_buffer_size = 8u8;
    let mut input_buffer_size = 0;

    loop {
        println!("input_buffer_size={:?}", input_buffer_size);
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
        println!("mask_size={:?}", mask_size);
        let mask = ((1 << (mask_size + 1)) - 1);
        println!("output_buffer={:#b}", output_buffer[0]);
        output_buffer[0] = output_buffer[0] << mask_size;
        output_buffer[0] = output_buffer[0] | (input_buffer[0] & mask); 
        println!("output_buffer={:#b}", output_buffer[0]);

        input_buffer_size = input_buffer_size - mask_size;
        output_buffer_size = output_buffer_size - mask_size;
        println!("input_buffer={:#b}", input_buffer[0]);
        input_buffer[0] = input_buffer[0] << mask_size;
        println!("input_buffer={:#b}", input_buffer[0]);
        if output_buffer_size == 0 {
            println!("cycle {:#0b}", output_buffer[0]);
            writer.write(&mut output_buffer)?;
            output_buffer_size = 8u8;
            output_buffer[0] = 0u8;
        }
    }

    if output_buffer_size > 0 {
        println!("final {:#0b}", output_buffer[0]);
        writer.write(&mut output_buffer)?;
    }

    Ok(processed_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_one_byte() {
        let input_byte: u8 = (1 << 7) | 5u8;
        let input = [input_byte];
        let mut output = [0u8; 1];
        let result = encode(&mut &input[..], &mut &mut output[..]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
        assert_eq!(output[0], 5u8);
    }

    #[test]
    fn test_encode_two_bytes() {
        let o_byte0: u8 = 0b11;
        let i_byte0: u8 = (1 << 7) | o_byte0;

        let o_byte1: u8 = 0b101;
        let i_byte1: u8 = (1 << 7) | o_byte1;

        let input = [i_byte0, i_byte1];
        let mut output = [0u8; 2]; 

        //second output byte is to be checked
        let result = encode(&mut &input[..], &mut &mut output[..]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2);
        assert_eq!(output[0], 0b00011001u8);
        //assert_eq!(output[1], 0b01000000u8); todo
    }
}
