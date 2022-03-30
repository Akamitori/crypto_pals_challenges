pub fn get_n_first_left_bits(byte: u8, n: u8) -> u8 {
    let bit_shift = (8 - n);
    let bitmask: u8 = (!0) << bit_shift;

    (byte & bitmask) >> bit_shift
}

pub fn get_n_first_right_bits(byte: u8, n: u8) -> u8 {
    let bit_shift = (8 - n);
    let bitmask: u8 = (!0) >> bit_shift;

    byte & bitmask
}

pub fn concat_bits_to_sixplets(
    bit_set_A: u8,
    A_bit_count: u8,
    bit_set_B: u8,
    B_bit_count: u8,
) -> u8 {
    if A_bit_count + B_bit_count > 6 {
        panic!("Invadlid bit count");
    }

    (bit_set_A << B_bit_count) | bit_set_B
}
