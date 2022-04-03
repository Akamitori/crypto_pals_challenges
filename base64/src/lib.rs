mod byte_operations;
mod set_1;

pub mod crypto_lib_set_1 {
    use crate::byte_operations::hex_parser;
    use crate::set_1::base64_converter;

    pub fn parse_hex_from_string(s: &str) -> Vec<u8> {
        return hex_parser::parse(s);
    }

    pub fn calculate_base_64(hex: &Vec<u8>) -> Vec<u8> {
        return base64_converter::convert(hex);
    }
}
