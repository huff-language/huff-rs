use bytes::{BufMut, BytesMut};
use std::ops::Index;

/// Convert a string (hex value) to an array containing 32 bytes
pub fn str_to_array(s: &str) -> [u8; 32] {
    let mut arr: [u8; 32] = Default::default();
    let mut buf = BytesMut::from(s);
    buf.resize(32, 0);
    arr.copy_from_slice(buf.as_ref());
    arr
}

/// Find the lowest value in an array of Literals, starting from `val`.
pub fn find_lowest(val: i32, literals: &Vec<[u8; 32]>) -> [u8; 32] {
    let low_val = str_to_array(val.to_string().as_str()); // TODO: This seems inefficient..

    match literals.iter().position(|l| *l == low_val) {
        Some(_) => find_lowest(val + 1, literals), // The value is already used. Look again.
        _ => low_val,                              /* If the current value is not contained in
                                                     * the literals vec, return it */
    }
}
