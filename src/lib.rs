mod base64_converter;

pub mod crypto_lib_set_1 {
    use crate::base64_converter::hex_parser;
    use crate::base64_converter::hex_to_64;

    pub fn parse_hex_from_string(s: &str) -> Vec<u8> {
        return hex_parser::parse_hex(s);
    }

    pub fn calculate_base_64(hex: &Vec<u8>) -> Vec<u8> {
        return hex_to_64::convert(hex);
    }
}
