use ethers::{abi::Token, types::*};
use huff_codegen::Codegen;
use huff_utils::bytes_util::*;

#[test]
fn encode_simple_constructor_args() {
    let expected_address: [u8; 20] = [
        100, 109, 184, 255, 194, 30, 125, 220, 43, 99, 39, 68, 141, 217, 250, 86, 13, 244, 16, 135,
    ];
    let expected_bytes32: Vec<u8> =
        str_to_vec("87674fa174add091f082eab424cc60625118fa4c553592a4e54a76fb9e8512f6");
    // Bogus constructors args
    let args: Vec<String> = vec![
        "Hello",
        "10000",
        "false",
        "0x646dB8ffC21e7ddc2B6327448dd9Fa560Df41087",
        "0x87674fa174add091f082eab424cc60625118fa4c553592a4e54a76fb9e8512f6",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    let results = Codegen::encode_constructor_args(args);

    assert_eq!(results[0], Token::String("Hello".to_string()));
    assert_eq!(results[1], Token::Uint(U256::from_dec_str("10000").unwrap()));
    assert_eq!(results[2], Token::Bool(false));
    assert_eq!(results[3], Token::Address(H160::from(expected_address)));
    assert_eq!(results[4], Token::FixedBytes(expected_bytes32));
}