use std::num::ParseIntError;

use tiny_keccak::{Hasher, Keccak};

/// Convert a string slice to a `[u8; 32]`
/// Pads zeros to the left of significant bytes in the `[u8; 32]` slice.
/// i.e. 0xa57b becomes `[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
/// 0, 0, 0, 0, 0, 165, 123]`
pub fn str_to_bytes32(s: &str) -> [u8; 32] {
    let s = format_even_bytes(String::from(s));

    let bytes: Vec<u8> =
        (0..s.len()).step_by(2).map(|c| u8::from_str_radix(&s[c..c + 2], 16).unwrap()).collect();

    let mut padded = [0u8; 32];

    for i in 32 - bytes.len()..32 {
        padded[i] = bytes[bytes.len() - (32 - i)];
    }

    padded
}

/// Convert a `[u8; 32]` to a bytes string.
pub fn bytes32_to_string(bytes: &[u8; 32], prefixed: bool) -> String {
    let mut s = String::default();
    let start = bytes.iter().position(|b| *b != 0).unwrap_or(bytes.len() - 1);
    tracing::debug!(target: "bytes_util", "got start: {}", start);
    for b in &bytes[start..bytes.len()] {
        tracing::debug!(target: "bytes_util", "Converting byte: {}", b);
        s = format!("{}{:02x}", s, *b);
        tracing::debug!(target: "bytes_util", "Converted byte: {} to string {}", b, s);
    }
    format!("{}{}", if prefixed { "0x" } else { "" }, s)
}

/// Wrapper to convert a hex string to a usize.
pub fn hex_to_usize(s: &str) -> Result<usize, ParseIntError> {
    usize::from_str_radix(s, 16)
}

/// Pad a hex string with n 0 bytes to the left. Will not pad a hex string that has a length
/// greater than or equal to `num_bytes * 2`
pub fn pad_n_bytes(hex: &str, num_bytes: usize) -> String {
    let mut hex = hex.to_owned();
    while hex.len() < num_bytes * 2 {
        hex = format!("0{}", hex);
    }
    hex
}

/// Pad odd-length byte string with a leading 0
pub fn format_even_bytes(hex: String) -> String {
    if hex.len() % 2 == 1 {
        format!("0{}", hex)
    } else {
        hex
    }
}

/// Convert string slice to Vec<u8>, size not capped
pub fn str_to_vec(s: &str) -> Result<Vec<u8>, std::num::ParseIntError> {
    let bytes: Result<Vec<u8>, _> =
        (0..s.len()).step_by(2).map(|c| u8::from_str_radix(&s[c..c + 2], 16)).collect();
    bytes
}

/// Hash a string with Keccak256
pub fn hash_bytes(dest: &mut [u8], to_hash: &String) {
    let mut hasher = Keccak::v256();
    hasher.update(to_hash.as_bytes());
    hasher.finalize(dest);
}
