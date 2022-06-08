/// Convert a string slice to a `[u8; 32]`
/// Will panic on odd-length byte strings. TODO: Pad odd-length hex strings
pub fn str_to_bytes32(s: &str) -> [u8; 32] {
    let mut v: Vec<u8> =
        (0..s.len()).step_by(2).map(|c| u8::from_str_radix(&s[c..c + 2], 16).unwrap()).collect();
    v.resize(32, 0); // If the hex string is not 32 bytes, resize it
    v.try_into().unwrap()
}

// TODO: create a bytes32 (`[u8; 32]`) -> hex String function
