pub mod set1 {

    use std::{
        fs::{self, File},
        io::{self, BufRead, BufReader},
    };

    use crypto::crypto_lib_set_1::{self, decipher_text, fixed_xor};
    use crypto::crypto_utils;
    use std::collections::HashMap;

    pub fn c_1_convert_hex_to_base64(input: &str) -> String {
        let y = crypto_utils::parse_hex_from_string(&input);
        let res = crypto_lib_set_1::calculate_base_64(&y)
            .into_iter()
            .map(|i| i as char)
            .collect::<String>();
        res
    }

    pub fn c_2_fixed_xor() {
        let buffer_1 =
            crypto_utils::parse_hex_from_string("0x1c0111001f010100061a024b53535009181c");
        let buffer_2 =
            crypto_utils::parse_hex_from_string("0x686974207468652062756c6c277320657965");
        let result = fixed_xor(&buffer_1, &buffer_2);
        crypto_utils::parse_string_from_hex(&result);
    }

    pub fn c_3_single_byte_xor_cipher(encrypted_text: &str) -> (String, u32) {
        let encrypted_text = crypto_utils::parse_hex_from_string(&encrypted_text);
        let (decrypted_text, _, encryption_score) =
            crypto_lib_set_1::decipher_text(&encrypted_text);
        (decrypted_text, encryption_score)
    }

    pub fn c_4_detect_single_character_xor(file_path: &str) -> (String, u32, usize) {
        let input =
            File::open(file_path).expect(format!("Failed to open file {}", file_path).as_str());

        let buffered = BufReader::new(input);

        let mut score = 0;
        let mut line = 0;
        let mut chosen = "".to_string();

        for (index, input_line) in buffered.lines().enumerate() {
            let hex_form = "0x".to_owned() + &input_line.unwrap();
            let (text, rating) = c_3_single_byte_xor_cipher(&hex_form);

            if rating > score {
                line = index;
                chosen = text;
                score = rating;
            }
        }

        return (chosen, score, line);
    }

    pub fn c_5_implement_repeating_key_xor(input: &str, key: &str) -> Vec<u8> {
        let input = crypto_utils::parse_hex_from_string(&input);
        let key = crypto_utils::parse_hex_from_string(&key);
        return crypto_lib_set_1::repeat_xor(&input, &key);
    }

    pub fn reverse_base64(input: &str) -> String {
        let y = crypto_utils::parse_hex_from_string(&input);
        let res = crypto_lib_set_1::reverse_base_64(&y);
        return crypto_utils::parse_string_from_hex(&res);
    }

    pub fn c_6_break_repeating_key_xor(file_path: &str) -> Result<String, io::Error> {
        let file_str = fs::read_to_string(file_path)?;
        let file_str = file_str.replace('\n', "");

        let mut files_bytes: Vec<u8> = file_str.bytes().map(|f| f).collect();
        files_bytes = crypto_lib_set_1::reverse_base_64(&files_bytes);

        let mut normalized_distance = calculate_normalized_distance(&files_bytes);

        let mut results_vec = HashMap::new();

        for _i in 0..4 {
            let candicate = normalized_distance.pop().unwrap();
            let key_size = candicate.0;

            let blocks: Vec<Vec<u8>> = split_to_byte_blocks(&files_bytes, key_size);

            let mut transposed_blocks: Vec<Vec<u8>> = Vec::new();

            for j in 0..key_size {
                let mut tblock = Vec::new();
                for block in (&blocks).into_iter() {
                    if j < block.len() {
                        let value = block[j];
                        tblock.push(value);
                    }
                }
                transposed_blocks.push(tblock);
            }

            let mut result = String::from("");
            let mut score = 0u32;

            for tblock in transposed_blocks {
                let decoded = decipher_text(&tblock);
                result.push(decoded.1);
                score += decoded.2;
            }

            results_vec.insert(key_size, (result, score));

            println!();
        }

        for a in results_vec {
            println!("{} {} {}", a.0, a.1 .0, a.1 .1);
        }

        let test: Vec<u8> = "Terminator X: Bring the noise"
            .to_string()
            .bytes()
            .map(|f| f)
            .collect();
        let y =
            crypto_utils::parse_utf8_from_hex(&crypto_lib_set_1::repeat_xor(&files_bytes, &test));
        println!("{}", y);

        //println!("{:?}", results_vec);

        return Ok("".to_string());
    }

    fn split_to_byte_blocks(files_bytes: &Vec<u8>, block_size: usize) -> Vec<Vec<u8>> {
        let mut blocks: Vec<Vec<u8>> = Vec::new();
        let mut block: Vec<u8> = Vec::new();
        for i in 0..files_bytes.len() {
            let push_to_blocks = i % block_size == 0 && i > 0;

            if push_to_blocks {
                blocks.push(block.clone());
                block.clear();
            }
            block.push(files_bytes[i]);
        }

        if block.len() > 0 {
            blocks.push(block);
        }

        blocks
    }

    fn calculate_normalized_distance(files_bytes: &Vec<u8>) -> Vec<(usize, f64)> {
        const MIN_KEY_SIZE: usize = 2;
        const MAX_KEY_SIZE: usize = 40;
        let mut normalized_distance = vec![(0usize, 0f64); 0];

        for i in MIN_KEY_SIZE..=MAX_KEY_SIZE {
            let mut distance_count = 0;
            let mut distance = 0.0;
            let upper_bound = files_bytes.len();
            for j in 0..10 {
                let start_1 = j * i;
                let end_1 = (j + 1) * i;
                let start_2 = end_1;
                let end_2 = (j + 2) * i;

                if start_1 >= upper_bound || {
                    end_1 >= upper_bound || start_2 >= upper_bound || end_2 >= upper_bound
                } {
                    break;
                }

                let buffer_1 = &files_bytes[start_1..end_1].to_vec();
                let buffer_2 = &files_bytes[start_2..end_2].to_vec();

                distance += crypto_lib_set_1::calculate_hemming_distance(buffer_1, buffer_2) as f64
                    / i as f64;
                distance_count += 1;
            }

            let avg_distance = distance / distance_count as f64;

            normalized_distance.push((i, avg_distance));
        }
        normalized_distance.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        normalized_distance
    }
}
