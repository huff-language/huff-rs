/// Convert a string slice to a `[u8; 32]`
/// Pads zeros to the left of significant bytes in the `[u8; 32]` slice.
/// i.e. 0xa57b becomes `[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
/// 0, 0, 0, 0, 0, 165, 123]`
pub fn str_to_bytes32(s: &str) -> [u8; 32] {
    let mut s = String::from(s);
    // Pad odd-length byte string with a leading 0
    if s.len() % 2 != 0 {
        s = format!("0{}", s);
    }

    let bytes: Vec<u8> =
        (0..s.len()).step_by(2).map(|c| u8::from_str_radix(&s[c..c + 2], 16).unwrap()).collect();

    let mut padded = [0u8; 32];

    for i in 32 - bytes.len()..32 {
        padded[i] = bytes[bytes.len() - (32 - i)];
    }

    padded
}

/// Convert a `[u8; 32]` to a bytes string.
pub fn bytes32_to_string(bytes: &[u8; 32]) -> String {
    let mut s = String::default();
    let start = bytes.iter().position(|b| *b != 0).unwrap_or(bytes.len() - 1);
    for b in &bytes[start..bytes.len()] {
        s = format!("{}{:02x}", s, *b);
    }
    format!("0x{}", s)
}
