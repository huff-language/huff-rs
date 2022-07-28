use huff_utils::bytes_util::*;

#[test]
fn test_bytes32_to_string() {

    let byte_arr: [u8; 32] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 254];
    let converted_string = bytes32_to_string(&byte_arr, false);
    assert_eq!(converted_string, "fe");
}