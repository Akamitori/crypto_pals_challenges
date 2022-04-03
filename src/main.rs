use crypto::crypto_lib_set_1;
use crypto::crypto_utils;

fn main() {
    let input = "0x49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    println!("{}", input);

    let y = crypto_utils::parse_hex_from_string(&input);
    let res = crypto_lib_set_1::calculate_base_64(&y)
        .into_iter()
        .map(|i| i as char)
        .collect::<String>();

    println!("{}", res);

    let b1 = "0x1c0111001f010100061a024b53535009181c";
    let b2 = "0x686974207468652062756c6c277320657965";

    let b1 = crypto_utils::parse_hex_from_string(&b1);
    let b2 = crypto_utils::parse_hex_from_string(&b2);

    let b3 = crypto_lib_set_1::fixed_xor(&b1, &b2);

    let res = b3
        .clone()
        .into_iter()
        .map(|i| i as char)
        .collect::<String>();

    println!("xor is {}", res);

    let b3 = crypto_lib_set_1::fixed_xor(&b3, &b2);
}
