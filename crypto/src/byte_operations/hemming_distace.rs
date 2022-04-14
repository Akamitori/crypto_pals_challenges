use super::xor;

pub fn calculate(bytes_1: &Vec<u8>, bytes_2: &Vec<u8>) -> u32 {
    let xor_result = xor::buffer_xor(bytes_1, bytes_2);

    let mut distance = 0;
    for b in xor_result {
        distance += count_ones(b);
    }

    return distance;
}

fn count_ones(byte: u8) -> u32 {
    let mut count = 0;

    for i in 0..8 {
        let byte_mask: u8 = 0b0000_0001;
        let res = (byte >> i) & byte_mask;

        if res == 1 {
            count += 1;
        }
    }

    count
}
