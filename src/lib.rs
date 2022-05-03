use std::io;

fn encode(reader: &mut dyn io::Read, writer: &mut dyn io::Write) -> std::io::Result<usize> {
    let mut input = [0u8];
    let mut processed_bytes: usize = 0;

    let encoding_size = 5;

    //todo what happens when 1u8 << 8
    //let mask = (1u8 << (mask_size + 1)) - 1;
    let mut output_buffer = [0u8];
    let mut output_buffer_size = 8u8;
    let mut input_buffer_size = 0;

    loop {
        if input_buffer_size == 0 {
            let size = reader.read(&mut input)?;
            if size == 0 {
                //no more data to read
                break;
            }

            processed_bytes += size;
            input_buffer_size = encoding_size;
        }

        let mask_size = std::cmp::min(input_buffer_size, output_buffer_size);
        let mask_left_shift = encoding_size - mask_size;
        let mask = ((1 << (mask_size + 1)) - 1) << mask_left_shift;
        output_buffer[0] = output_buffer[0] << mask_size;
        output_buffer[0] = output_buffer[0] | ((input[0] & mask) >> mask_left_shift); 

        input_buffer_size = input_buffer_size - mask_size;
        output_buffer_size = output_buffer_size - mask_size;
        if output_buffer_size == 0 {
            writer.write(&mut output_buffer)?;
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
    fn test_encode() {
        let mut input = [0u8, 1u8, 2u8, 3u8, 4u8];
        let mut output = [0u8; 5];
        let result = encode(&mut &input[..], &mut &mut output[..]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), input.len());
        assert_eq!(input, output);

        println!("input = {:?}", input);
        println!("output = {:?}", output);
    }
}
