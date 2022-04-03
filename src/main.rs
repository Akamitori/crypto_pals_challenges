use crypto_pals_1::crypto_lib_set_1::calculate_base_64;
use crypto_pals_1::crypto_lib_set_1::parse_hex_from_string;
fn main() {
    let input = "0x49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    println!("{}", input);

    let y = parse_hex_from_string(&input);
    let res = calculate_base_64(&y)
        .into_iter()
        .map(|i| i as char)
        .collect::<String>();

    print!("{}", res);
}
