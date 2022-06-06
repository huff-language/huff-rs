use bytes::BytesMut;

/// Helper function to create literal from ref string
pub fn create_literal_from_str(string: &str) -> [u8; 32] {
    let mut arr: [u8; 32] = Default::default();
    let mut buf = BytesMut::from(string);
    buf.resize(32, 0);
    arr.copy_from_slice(buf.as_ref());
    arr
}
