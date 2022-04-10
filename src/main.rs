use crypto::crypto_utils::parse_string_from_hex;
use crypto_pals_1::encrypt_with_repeating_xor;

fn main() {
    let input = "Burning 'em, if you ain't quick and nimble
    I go crazy when I hear a cymbal";

    let key = "ICE";

    let result = encrypt_with_repeating_xor(input, key);

    let result = parse_string_from_hex(&result);

    println!("{}", result);
}
