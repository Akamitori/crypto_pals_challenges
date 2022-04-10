use crypto::{crypto_lib_set_1, crypto_utils};
use crypto_pals_1;

fn main() {
    let input = "Burning 'em, if you ain't quick and nimble
    I go crazy when I hear a cymbal";

    let key = "ICE";

    let result = crypto_pals_1::encrypt_with_repeating_xor(input, key);

    let result2 = crypto_utils::parse_utf8_from_hex(&crypto_lib_set_1::repeat_xor(
        &result,
        &crypto_utils::parse_hex_from_string(&key),
    ));

    let result = crypto_utils::parse_string_from_hex(&result);

    println!("{}", result);
    println!("{}", result2);
}
