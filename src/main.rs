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

    let encrypted_text = "0x1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

    let encrypted_text = crypto_utils::parse_hex_from_string(&encrypted_text);

    let decrypted_text = crypto_lib_set_1::decipher_text(&encrypted_text);

    println!("{}", decrypted_text);
}
