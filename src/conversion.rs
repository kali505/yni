const HEX_MAP: &[char] = &[
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

pub fn convert_str(src: &str, format: u32) -> String {
    assert!(format >= 2);
    assert!(format <= 36);

    if format.is_power_of_two() {
        let mut digitsz = 0;
        let mut x = format;
        let mut ret = String::new();

        loop {
            x = x >> 1;
            if x == 0 {
                break;
            }
            digitsz += 1;
        }

        let mask = 2_u32.pow(digitsz) - 1;

        for x_ in src.as_bytes() {
            if *x_ == 0 {
                ret.push_str("0 ");
            }
            let mut x = *x_ as u32;

            for _i in 0..8 / digitsz {
                let index = (x & mask) as usize;
                x = x >> digitsz;
                ret.push(HEX_MAP[index]);
            }

            ret.push(' ');
        }
        if ret.as_bytes().last() == Some(&b' ') {
            ret.pop();
        }

        return ret;
    } else {
        // min format = 3 => 3^6 = 273
        let mut map: Vec<u32> = Vec::with_capacity(6);
        let mut ret = String::new();

        map.push(1);
        for i in 1.. {
            if map[i - 1] * format > 255 {
                break;
            }
            map.push(map[i - 1] * format);
        }

        for x_ in src.as_bytes() {
            if *x_ == 0 {
                ret.push_str("0 ");
            }

            let mut x = *x_ as u32;
            let start = map
                .iter()
                .enumerate()
                .filter(|(_i, e)| **e <= x)
                .max()
                .unwrap()
                .0;

            for i in (0..=start).rev() {
                ret.push(HEX_MAP[(x / map[i]) as usize]);
                x = x % map[i];
            }

            ret.push(' ');
        }
        if ret.as_bytes().last() == Some(&b' ') {
            ret.pop();
        }

        return ret;
    }
}
