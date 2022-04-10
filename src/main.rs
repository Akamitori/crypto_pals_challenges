use crypto_pals_1::decipher_lines;
use crypto_pals_1::decrypt_text;
use crypto_pals_1::get_base64;

fn main() {
    let res = get_base64("0x49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");

    let (decrypted_text, encyrption_score) =
        decrypt_text("0x1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");

    println!("{} {}", decrypted_text, encyrption_score);

    let result = decipher_lines("C:\\lines.txt");

    println!("{} {} {}", result.0, result.1, result.2);
}
