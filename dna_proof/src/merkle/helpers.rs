pub fn bp_to_bits(bp: &u8) -> u8 {
    match bp {
        b'A' => 0b00,
        b'C' => 0b01,
        b'G' => 0b10,
        b'T' => 0b11,
        _ => panic!("Invalid DNA base"),
    }
}
