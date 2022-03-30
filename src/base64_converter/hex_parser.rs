pub fn parse_hex(s: &str) -> Vec<u8> {
    let hexs = split_hex_from_str(s);
    let hex_bytes = pair_hex_to_bytes(hexs);
    return hex_bytes;
}

fn split_hex_from_str(hex_str: &str) -> Vec<u8> {
    let mut power: u32 = 0;
    let mut counter = 0;

    let individual_hexs: Vec<u8> = hex_str
        .chars()
        .map(|f| {
            if let Some(hex) = f.to_digit(16) {
                power = match power {
                    0 => match counter {
                        _ if counter == hex_str.len() - 1 => 0,
                        _ => 1,
                    },
                    _ => 0,
                };
                counter += 1;
                hex as u8 * 16u8.pow(power)
            } else {
                panic!("Invalid character {}", f);
            }
        })
        .collect();
    return individual_hexs;
}

fn pair_hex_to_bytes(hexs: Vec<u8>) -> Vec<u8> {
    let mut hex_bytes: Vec<u8> = Vec::new();
    let mut sum: u8 = 0;

    let total_elements = hexs.len();
    for (index, number) in hexs.into_iter().enumerate() {
        let reset_sum = index % 2 == 0 && index != 0;

        if reset_sum {
            hex_bytes.push(sum);
            sum = 0;
        }

        sum += number;

        let reached_end = index == total_elements - 1;

        if reached_end {
            hex_bytes.push(sum);
        }
    }
    return hex_bytes;
}
