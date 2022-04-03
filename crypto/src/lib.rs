mod byte_operations;
mod set_1;

pub mod crypto_lib_set_1 {
    use std::collections::HashMap;

    use crate::set_1::base64_converter;
    use crate::set_1::decipher;
    use crate::set_1::fixed_xor;
    use crate::set_1::plaintext_scorer;

    pub fn calculate_base_64(hex: &Vec<u8>) -> Vec<u8> {
        return base64_converter::convert(hex);
    }

    pub fn fixed_xor(hex1: &Vec<u8>, hex2: &Vec<u8>) -> Vec<u8> {
        return fixed_xor::execute(&hex1, &hex2);
    }

    pub fn decipher_text(text: &Vec<u8>) -> String {
        return decipher::decipher_text(&text);
    }
}

pub mod crypto_utils {
    use crate::byte_operations::hex_parser;

    pub fn parse_hex_from_string(s: &str) -> Vec<u8> {
        return hex_parser::parse_from_string(s);
    }

    pub fn parse_string_from_hex(hex: &Vec<u8>) -> String {
        return hex_parser::parse_from_buffer(&hex);
    }

    pub fn parse_utf8_from_hex(hex: &Vec<u8>) -> String {
        return hex_parser::parse_utf8_from_buffer(&hex);
    }
}
