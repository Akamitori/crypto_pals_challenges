pub fn buffer_xor(buffer1: &Vec<u8>, buffer2: &Vec<u8>) -> Vec<u8> {
    let mut result = Vec::new();

    match buffer1.len() {
        _ if buffer1.len() != buffer2.len() => {
            panic!(
                "{}",
                format!("buffer lengths should be equal {:?} {:?}", buffer1, buffer2)
            )
        }
        0 => panic!("buffer lengths should not be 0"),
        _ => {}
    };

    for (i, byte) in buffer1.iter().enumerate() {
        result.push(byte ^ buffer2[i]);
    }

    return result;
}

pub fn repeating_key_xor(buffer1: &Vec<u8>, repeating_key: &Vec<u8>) -> Vec<u8> {
    let mut result = Vec::new();

    let max_len = repeating_key.len() - 1;

    let mut cur_byte = 0;

    for i in buffer1 {
        result.push(*i ^ repeating_key[cur_byte]);

        cur_byte = match cur_byte {
            _ if cur_byte == max_len => 0,
            _ => cur_byte + 1,
        };
    }

    result
}
