pub fn convert(hex: &Vec<u8>) -> Vec<u8> {
    let mut hex = hex.clone();
    let extra_bytes = pad_with_extra_bytes(&mut hex);

    let grouped_bytes = seperate_to_3_byte_groups(&hex);

    let mut sexlets = seperate_to_sexlets(&grouped_bytes);

    let vec_len = sexlets.len();

    for b in 0..extra_bytes {
        sexlets[vec_len - 1 - b] = 64;
    }

    return map_sexlets_to_bytes(sexlets, extra_bytes);
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

fn map_sexlets_to_bytes(mut sexlets: Vec<u8>, bytes_to_ignore: usize) -> Vec<u8> {
    let first_upper = 'A' as u8;
    let first_lower = 'a' as u8;
    let first_num = '0' as u8;

    let vec_len = sexlets.len();

    for b in 0..bytes_to_ignore {
        sexlets[vec_len - 1 - b] = 64;
    }

    let sexlets: Vec<u8> = sexlets
        .into_iter()
        .map(
            |f| match f {
                0..=25 => first_upper + f,
                26..=51 => first_lower + (f - 26),
                52..=61 => first_num + (f - 52),
                62 => '+' as u8,
                63 => '/' as u8,
                _ => '=' as u8,
            }, //let capitalLetter=0..26.contains(&f as i32);
        )
        .collect();

    return sexlets;
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

fn seperate_to_sexlets(grouped_bytes: &Vec<u32>) -> Vec<u8> {
    let mut result = Vec::new();
    let bit_mask = 0b111_111 as u32;

    for byte in &*grouped_bytes {
        for bit_group in 0..4 {
            let sexlet = ((byte >> (3 - bit_group) * 6) & bit_mask) as u8;
            result.push(sexlet);
        }
    }

    result
}
