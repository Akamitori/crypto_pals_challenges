mod base64;
mod byte_operations;
mod set_1;

pub mod crypto_lib_set_1 {
    use crate::base64;
    use crate::byte_operations::xor;
    use crate::set_1::decipherer;

    pub fn calculate_base_64(hex: &Vec<u8>) -> Vec<u8> {
        return base64::converter::convert(hex);
    }

    pub fn fixed_xor(buffer_1: &Vec<u8>, buffer_2: &Vec<u8>) -> Vec<u8> {
        return xor::buffer_xor(&buffer_1, &buffer_2);
    }

    pub fn repeat_xor(buffer_1: &Vec<u8>, repeating_key: &Vec<u8>) -> Vec<u8> {
        return xor::repeating_key_xor(buffer_1, repeating_key);
    }

    pub fn decipher_text(text: &Vec<u8>) -> (String, u32) {
        return decipherer::decipher_text(&text);
    }
}

pub mod crypto_utils {
    use crate::byte_operations::hex_parser;

    pub fn parse_hex_from_string(s: &str) -> Vec<u8> {
        return hex_parser::from_string(s);
    }

    pub fn parse_string_from_hex(hex: &Vec<u8>) -> String {
        return hex_parser::from_buffer(&hex);
    }

    pub fn parse_utf8_from_hex(hex: &Vec<u8>) -> String {
        return hex_parser::to_utf8_string(&hex);
    }
}
