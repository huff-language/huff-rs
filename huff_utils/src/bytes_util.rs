/// Convert a string slice to a `[u8; 32]`
pub fn str_to_bytes32(s: &str) -> [u8; 32] {
    let mut s = String::from(s);
    // Pad odd-length byte string with a leading 0
    if s.len() % 2 != 0 {
        s = format!("0{}", s);
    }

    let mut v: Vec<u8> =
        (0..s.len()).step_by(2).map(|c| u8::from_str_radix(&s[c..c + 2], 16).unwrap()).collect();
    v.resize(32, 0); // If the hex string is not 32 bytes, resize it
    v.try_into().unwrap()
}

/// Convert a `[u8; 32]` to a bytes string. Does not retain zeroed-out bytes.
pub fn bytes32_to_string(bytes: &[u8; 32]) -> String {
    let mut s = String::default();
    for &b in bytes {
        if b == 0 {
            break
        } // TODO: sometimes, the zeros are significant. This would break 0x1000 at 0x10, for example.
        s = format!("{}{:02x}", s, b);
    }
    format!("0x{}", s)
}
