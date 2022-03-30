pub fn convert(hex: &Vec<u8>) {
    let mut hex = hex.clone();
    let extra_bytes = pad_with_extra_bytes(&mut hex);

    //utilize a bit mask for every 6 bits
}

fn pad_with_extra_bytes(hex: &mut Vec<u8>) -> u32 {
    println!("{}", hex.len() % 3);
    for c in &*hex {
        println!("{:b}", c);
    }

    let missing_bytes = (3 - hex.len() % 3) as u32;

    for _ in 0..missing_bytes {
        hex.push(0);
    }

    missing_bytes
}

fn seperate_to_sixlets(hex: &Vec<u8>) -> Vec<u8> {
    let sixlet = Vec::new();

    return sixlet;
}

// fn pad_last_byte(hex: &mut Vec<u8>) {
//     let pad_zeros = ((hex.len() * 8) % 24) as u32;

//     println!("len is {}", pad_zeros);
//     let add_padding = pad_zeros != 0;

//     for c in &*hex {
//         println!("{:b}", c);
//     }

//     if add_padding {
//         if hex.len() < 3 {}
//         if let Some(last) = hex.last_mut() {
//             *last = *last * (2 as u8).pow(pad_zeros);
//         }
//     }

//     // for c in &*hex {
//     //     println!("{:b}", c);
//     // }
//     //add least significant digits

//     //add padding
// }
