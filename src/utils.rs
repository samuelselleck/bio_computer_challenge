const NUM_BASE_LENGTH: usize = 16; //32 bits

pub fn to_u32(seq: &str) -> Option<u32> {
    //assert_eq!(seq.len(), NUM_BASE_LENGTH);
    seq.char_indices()
        .map(|(i, c)| {
            let digit = match c {
                'A' => 0,
                'C' => 1,
                'G' => 2,
                'T' => 3,
                _ => return None,
            };
            Some(digit << 2 * i)
        })
        .sum()
}

pub fn to_seq(val: u32) -> String {
    let mut seq = String::new();
    let mut part = val;
    for _ in 0..NUM_BASE_LENGTH {
        let digit = part % 4;
        seq.push(match digit {
            0 => 'A',
            1 => 'C',
            2 => 'G',
            3 => 'T',
            _ => unreachable!(),
        });

        part /= 4;
    }
    seq
}
