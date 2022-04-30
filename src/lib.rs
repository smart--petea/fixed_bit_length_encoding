use std::io;

fn encode(reader: &mut dyn io::Read, writer: &mut dyn io::Write) -> std::io::Result<usize> {
    let mut buffer = [0u8];
    let mut processed_bytes: usize = 0;
    loop {
        let size = reader.read(&mut buffer)?;
        if size == 0 {
            //no more data to read
            break;
        }

        processed_bytes += size;
        writer.write(&mut buffer)?;
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
