use crate::crypto_utils;

pub fn from_hex(hex: &Vec<u8>) -> Vec<u8> {
    let mut hex = hex.clone();
    let extra_bytes = pad_with_extra_bytes(&mut hex);

    let grouped_bytes = seperate_to_3_byte_groups(&hex);

    let sexlets = seperate_bytes_to_sexlets(&grouped_bytes);

    return map_sexlets_to_bytes(sexlets, extra_bytes);
}

pub fn to_hex(base64: &Vec<u8>) -> Vec<u8> {
    let mut hex = base64.clone();

    let exta_bytes = map_bytes_to_sexlets(&mut hex);

    let grouped_bytes = unite_sexlets_to_bytes(&hex);

    let raw_bytes = seperate_bytes_groups_to_single_bytes(grouped_bytes, exta_bytes);

    return raw_bytes;
}

fn pad_with_extra_bytes(hex: &mut Vec<u8>) -> usize {
    let missing_bytes = match hex.len() % 3 {
        0 => 0,
        other => 3 - other,
    };

    for _ in 0..missing_bytes {
        hex.push(0);
    }

    missing_bytes
}

fn map_bytes_to_sexlets(bytes: &mut Vec<u8>) -> usize {
    const FIRST_UPPER: u8 = 'A' as u8;
    const LAST_UPPER: u8 = FIRST_UPPER + 25;

    const FIRST_LOWER: u8 = 'a' as u8;
    const LAST_LOWER: u8 = FIRST_LOWER + 25;

    const FIRST_NUM: u8 = '0' as u8;
    const LAST_NUM: u8 = FIRST_NUM + 9;

    const PLUS_SIGN: u8 = '+' as u8;
    const SLASH: u8 = '/' as u8;

    const EQ_SIGN: u8 = '=' as u8;

    let mut extra_bytes = 0;

    for f in &mut *bytes {
        *f = match *f {
            FIRST_UPPER..=LAST_UPPER => *f - FIRST_UPPER,
            FIRST_LOWER..=LAST_LOWER => *f - FIRST_LOWER + 26,
            FIRST_NUM..=LAST_NUM => *f - FIRST_NUM + 52,
            PLUS_SIGN => 62,
            SLASH => 63,
            EQ_SIGN => {
                extra_bytes += 1;
                64
            }
            _ => panic!("Invalid byte"),
        };
    }
    return extra_bytes;
}

fn map_sexlets_to_bytes(mut sexlets: Vec<u8>, bytes_to_ignore: usize) -> Vec<u8> {
    let first_upper = 'A' as u8;
    let first_lower = 'a' as u8;
    let first_num = '0' as u8;

    let vec_len = sexlets.len();

    for b in 0..bytes_to_ignore {
        sexlets[vec_len - 1 - b] = 64;
    }

    let base64_bytes: Vec<u8> = sexlets
        .into_iter()
        .map(|f| match f {
            0..=25 => first_upper + f,
            26..=51 => first_lower + (f - 26),
            52..=61 => first_num + (f - 52),
            62 => '+' as u8,
            63 => '/' as u8,
            64 => '=' as u8,
            _ => panic!("Out of range"),
        })
        .collect();

    return base64_bytes;
}

fn seperate_to_3_byte_groups(bytes: &Vec<u8>) -> Vec<u32> {
    let mut result = Vec::new();

    let group_count = bytes.len() / 3;

    for r in 0..group_count {
        let g1 = (bytes[3 * r + 0] as u32) << 16;
        let g2 = (bytes[3 * r + 1] as u32) << 8;
        let g3 = bytes[3 * r + 2] as u32;

        result.push(g1 | g2 | g3);
    }

    result
}

fn seperate_bytes_groups_to_single_bytes(bytes: Vec<u32>, extra_bytes: usize) -> Vec<u8> {
    let mut result = Vec::new();

    for grouped_bytes in bytes {
        let g1 = (grouped_bytes >> 16) & 0b1111_1111;
        let g2 = (grouped_bytes >> 8) & 0b1111_1111;
        let g3 = grouped_bytes & 0b1111_1111;

        result.push(g1 as u8);
        result.push(g2 as u8);
        result.push(g3 as u8);
    }

    for _ in 0..extra_bytes {
        result.pop();
    }

    return result;
}

fn seperate_bytes_to_sexlets(grouped_bytes: &Vec<u32>) -> Vec<u8> {
    let mut result = Vec::new();
    let bit_mask = 0b111_111 as u32;

    for &byte in &*grouped_bytes {
        for bit_group in 0..4 {
            let sexlet = ((byte >> (3 - bit_group) * 6) & bit_mask) as u8;
            result.push(sexlet);
        }
    }

    result
}

fn unite_sexlets_to_bytes(sexlets: &Vec<u8>) -> Vec<u32> {
    let mut result = Vec::new();

    let expected_group_no = sexlets.len() / 4;

    for i in 0..expected_group_no {
        let mut byte_group = 0;
        let mut group_counter = 0;
        for j in i * 4..(i + 1) * 4 {
            byte_group |= (sexlets[j] as u32) << (3 - group_counter) * 6;
            group_counter += 1;
        }
        result.push(byte_group);
    }

    result
}
