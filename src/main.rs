use crypto_pals_1::crypto_lib::calculate_base_64;
use crypto_pals_1::crypto_lib::parse_hex_from_string;

fn main() {
    /*
    let i: [u8; 48] = [
        0x49, 0x27, 0x6d, 0x20, 0x6b, 0x69, 0x6c, 0x6c, 0x69, 0x6e, 0x67, 0x20, 0x79, 0x6f, 0x75,
        0x72, 0x20, 0x62, 0x72, 0x61, 0x69, 0x6e, 0x20, 0x6c, 0x69, 0x6b, 0x65, 0x20, 0x61, 0x20,
        0x70, 0x6f, 0x69, 0x73, 0x6f, 0x6e, 0x6f, 0x75, 0x73, 0x20, 0x6d, 0x75, 0x73, 0x68, 0x72,
        0x6f, 0x6f, 0x6d,
    ];
    */
    let input_str = "A";
    //let input ="49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let input = (input_str
        .bytes()
        .into_iter()
        .map(|f| f.to_string())
        .collect::<Vec<String>>())
    .join("");
    println!("{}", input);

    // input.starts_with("0x");
    // //49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d
    // let z: Vec<u8> = String::from("test")
    //     .as_bytes()
    //     .into_iter()
    //     .map(|f| *f as u8)
    //     .collect();

    // for c in z {
    //     println!("{:X}", c);
    // }
    // return;
    //let test: Vec<u8> = z.into_iter().map(|f| f).collect();

    let y = parse_hex_from_string(&input);
    calculate_base_64(&y);
    // let perfect_sixlet = 8 * y.len() % 6 == 0;

    // let c = 0xF;
    // let d = c * 2 * 2;

    // println!("{:b}", c);
    // println!("{}", c.to_string());
}
