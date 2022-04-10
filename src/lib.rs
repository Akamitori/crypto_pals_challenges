use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crypto::crypto_lib_set_1;
use crypto::crypto_utils;

pub fn get_base64(input: &str) -> String {
    let y = crypto_utils::parse_hex_from_string(&input);
    let res = crypto_lib_set_1::calculate_base_64(&y)
        .into_iter()
        .map(|i| i as char)
        .collect::<String>();
    res
}

pub fn decrypt_text(encrypted_text: &str) -> (String, u32) {
    let encrypted_text = crypto_utils::parse_hex_from_string(&encrypted_text);
    let (decrypted_text, encryption_score) = crypto_lib_set_1::decipher_text(&encrypted_text);
    (decrypted_text, encryption_score)
}

pub fn decipher_lines(file_path: &str) -> (String, u32, usize) {
    let input = File::open(file_path).expect(format!("Failed to open file {}", file_path).as_str());

    let buffered = BufReader::new(input);

    let mut score = 0;
    let mut line = 0;
    let mut chosen = "".to_string();

    for (index, input_line) in buffered.lines().enumerate() {
        let hex_form = "0x".to_owned() + &input_line.unwrap();
        let (text, rating) = decrypt_text(&hex_form);

        if rating > score {
            line = index;
            chosen = text;
            score = rating;
        }
    }

    return (chosen, score, line);
}

pub fn encrypt_with_repeating_xor(input: &str, key: &str) -> Vec<u8> {
    let input = crypto_utils::parse_hex_from_string(&input);
    let key = crypto_utils::parse_hex_from_string(&key);
    return crypto_lib_set_1::repeat_xor(&input, &key);
}
