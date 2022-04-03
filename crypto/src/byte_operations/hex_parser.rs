pub fn parse_from_string(s: &str) -> Vec<u8> {
    let mut s = s.clone();
    let is_hex = s.starts_with("0x");

    if !is_hex {
        return conver_string_to_bytes(s);
    } else {
        s = &s[2..]
    }

    let hexs = split_hex_from_str(s);
    let hex_bytes = pair_hex_to_bytes(hexs);
    return hex_bytes;
}

pub fn parse_from_buffer(hex: &Vec<u8>) -> String {
    let f = hex
        .clone()
        .into_iter()
        .map(|i| format!("{:02x}", i))
        .collect::<String>();

    return f;
}

fn split_hex_from_str(hex_str: &str) -> Vec<u8> {
    let mut raise_power = false;
    let mut counter = 0;

    let individual_hexs: Vec<u8> = hex_str
        .chars()
        .map(|f| {
            if let Some(hex) = f.to_digit(16) {
                raise_power = match raise_power {
                    false => match counter {
                        _ if counter == hex_str.len() - 1 => false,
                        _ => true,
                    },
                    _ => false,
                };
                counter += 1;
                hex as u8 * 16u8.pow(raise_power as u32)
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

fn conver_string_to_bytes(s: &str) -> Vec<u8> {
    return s.bytes().into_iter().map(|b| b).collect::<Vec<u8>>();
}
