use crate::{crypto_lib_set_1, crypto_utils, set_1::plaintext_scorer::produce_scoring};

pub fn decipher_text(encrypted_text: &Vec<u8>) -> String {
    let encryption_key_scores = produce_scoring(&encrypted_text);

    let encryption_byte = *encryption_key_scores
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _v)| k)
        .unwrap();

    let decryption_key = vec![encryption_byte as u8; encrypted_text.len()];

    let decrypted_text = crypto_utils::parse_utf8_from_hex(&crypto_lib_set_1::fixed_xor(
        &encrypted_text,
        &decryption_key,
    ));

    return decrypted_text;
}
