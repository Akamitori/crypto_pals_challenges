pub fn execute(buffer1: &Vec<u8>, buffer2: &Vec<u8>) -> Vec<u8> {
    let mut result = Vec::new();

    match buffer1.len() {
        _ if buffer1.len() != buffer2.len() => panic!("buffer lengths should be equal"),
        0 => panic!("buffer lengths should be equal"),
        _ => {}
    };

    for (i, byte) in buffer1.iter().enumerate() {
        result.push(byte ^ buffer2[i]);
    }

    return result;
}
