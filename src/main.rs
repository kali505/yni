use std::io;
use std::io::prelude::*;

const HEX_MAP: &[char] = &[
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
];

fn to_hex_str(src: &String, sum: &mut String) {
    for x in src.as_bytes() {
        let high_nbl = (x >> 4) as usize;
        let low_nbl = (x & 0x0F) as usize;

        sum.push(HEX_MAP[high_nbl]);
        sum.push(HEX_MAP[low_nbl]);
    }
}

fn main() {
    let mut stdin = io::stdin().lock();
    let mut s = String::new();
    stdin.read_line(&mut s).unwrap();
    s.pop();

    let mut out = String::new();
    to_hex_str(&s, &mut out);
    println!("0x{}", out);

    if cfg!(target_endian = "big") {
        println!("[Big Endian, UTF-8]");
    } else {
        println!("[Little Endian, UTF-8]");
    }
}
