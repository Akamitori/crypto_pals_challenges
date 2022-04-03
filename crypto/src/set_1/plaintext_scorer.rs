use crate::crypto_lib_set_1;
use crate::crypto_utils;
use std::collections::HashMap;

pub fn produce_scoring(hex: &Vec<u8>) -> HashMap<char, u32> {
    let mut scores = HashMap::new();

    let buffer_size = hex.len();

    for c in 0..=255 {
        let cipher = vec![c; buffer_size];
        let ciphered_buffer = crypto_lib_set_1::fixed_xor(&hex, &cipher);
        let ciphered_text = crypto_utils::parse_utf8_from_hex(&ciphered_buffer);

        scores
            .entry(c as char)
            .or_insert(calculate_score(&ciphered_text));
    }

    return scores;
}

fn calculate_score(input: &str) -> u32 {
    let capital_leters = 'A'..='Z';
    let lower_case_letters = 'a'..='z';
    let misc = ['.', ',', '\'', '"', ' '];

    let mut result = 0;
    for c in input.chars() {
        if lower_case_letters.contains(&c) {
            result += 1;
        }
        if capital_leters.contains(&c) {
            result += 1;
        }

        if misc.contains(&c) {
            result += 1;
        }
    }
    result
}
