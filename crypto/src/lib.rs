mod byte_operations;
mod set_1;

pub mod crypto_lib_set_1 {
    use crate::set_1::base64_converter;
    use crate::set_1::decipher;
    use crate::set_1::xor_operations;

    pub fn calculate_base_64(hex: &Vec<u8>) -> Vec<u8> {
        return base64_converter::convert(hex);
    }

    pub fn fixed_xor(buffer_1: &Vec<u8>, buffer_2: &Vec<u8>) -> Vec<u8> {
        return xor_operations::buffer_xor(&buffer_1, &buffer_2);
    }

    pub fn repeat_xor(buffer_1: &Vec<u8>, repeating_key: &Vec<u8>) -> Vec<u8> {
        return xor_operations::repeating_key_xor(buffer_1, repeating_key);
    }

    pub fn decipher_text(text: &Vec<u8>) -> (String, u32) {
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
