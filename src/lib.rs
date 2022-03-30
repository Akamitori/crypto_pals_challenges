mod base64_converter;

pub use crate::base64_converter::hex_parser;
pub use crate::base64_converter::hex_to_64;

pub mod crypto_lib {

    pub fn parse_hex_from_string(s: &str) -> Vec<u8> {
        return super::hex_parser::parse_hex(s);
    }

    pub fn calculate_base_64(hex: &Vec<u8>) {
        super::hex_to_64::convert(hex);
    }
}
