use ethers_core::{
    abi::{Token, Tokenizable},
    types::*,
};
use huff_codegen::Codegen;
use huff_utils::bytes_util::*;

#[test]
fn encode_simple_constructor_args() {
    let expected_address: [u8; 20] = [
        100, 109, 184, 255, 194, 30, 125, 220, 43, 99, 39, 68, 141, 217, 250, 86, 13, 244, 16, 135,
    ];
    let expected_bytes32: Vec<u8> =
        str_to_vec("87674fa174add091f082eab424cc60625118fa4c553592a4e54a76fb9e8512f6").unwrap();
    // Bogus constructors args
    let args: Vec<String> = [
        "Hello",
        "10000",
        "false",
        "0x646dB8ffC21e7ddc2B6327448dd9Fa560Df41087",
        "0x87674fa174add091f082eab424cc60625118fa4c553592a4e54a76fb9e8512f6",
        "-10",
        "+55",
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
    assert_eq!(results[5], "-10".parse::<i128>().unwrap().into_token());
    assert_eq!(results[6], "+55".parse::<i128>().unwrap().into_token());
}

#[test]
fn encode_array_constructor_args() {
    let expected_address: [u8; 20] = [
        100, 109, 184, 255, 194, 30, 125, 220, 43, 99, 39, 68, 141, 217, 250, 86, 13, 244, 16, 135,
    ];
    let _expected_bytes32: Vec<u8> =
        str_to_vec("87674fa174add091f082eab424cc60625118fa4c553592a4e54a76fb9e8512f6").unwrap();
    // Bogus constructors args
    let args: Vec<String> = [
        "[100, 200, 300]",
        "[0x646dB8ffC21e7ddc2B6327448dd9Fa560Df41087, 0x646dB8ffC21e7ddc2B6327448dd9Fa560Df41087]",
        "[true, false, false]",
        "[Hello, World, Yes]",
        "[\"Hello\", \"World\", \"Yes\"]",
        "['Hello', 'World', 'Yes']",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    let results = Codegen::encode_constructor_args(args);

    assert_eq!(
        results[0],
        Token::Array(vec![
            Token::Uint(U256::from_dec_str("100").unwrap()),
            Token::Uint(U256::from_dec_str("200").unwrap()),
            Token::Uint(U256::from_dec_str("300").unwrap()),
        ])
    );
    assert_eq!(
        results[1],
        Token::Array(vec![
            Token::Address(H160::from(expected_address)),
            Token::Address(H160::from(expected_address)),
        ])
    );
    assert_eq!(
        results[2],
        Token::Array(vec![Token::Bool(true), Token::Bool(false), Token::Bool(false),])
    );
    let expected_array = Token::Array(vec![
        Token::String("Hello".to_string()),
        Token::String("World".to_string()),
        Token::String("Yes".to_string()),
    ]);
    assert_eq!(results[3], expected_array);
    assert_eq!(results[4], expected_array);
    assert_eq!(results[5], expected_array);
}

#[test]
fn encode_missing_brackets_array_constructor_args() {
    let expected_address: [u8; 20] = [
        100, 109, 184, 255, 194, 30, 125, 220, 43, 99, 39, 68, 141, 217, 250, 86, 13, 244, 16, 135,
    ];
    let _expected_bytes32: Vec<u8> =
        str_to_vec("87674fa174add091f082eab424cc60625118fa4c553592a4e54a76fb9e8512f6").unwrap();
    // Bogus constructors args
    let args: Vec<String> = ["  100,  200,  300   ",
        " 0x646dB8ffC21e7ddc2B6327448dd9Fa560Df41087,    0x646dB8ffC21e7ddc2B6327448dd9Fa560Df41087",
        "true,  false,   false",
        "Hello, World, Yes",
        "   \"Hello\", \"World\", \"Yes\"",
        "'Hello',  'World', 'Yes'   "]
    .iter()
    .map(|s| s.to_string())
    .collect();

    let results = Codegen::encode_constructor_args(args);

    assert_eq!(
        results[0],
        Token::Array(vec![
            Token::Uint(U256::from_dec_str("100").unwrap()),
            Token::Uint(U256::from_dec_str("200").unwrap()),
            Token::Uint(U256::from_dec_str("300").unwrap()),
        ])
    );
    assert_eq!(
        results[1],
        Token::Array(vec![
            Token::Address(H160::from(expected_address)),
            Token::Address(H160::from(expected_address)),
        ])
    );
    assert_eq!(
        results[2],
        Token::Array(vec![Token::Bool(true), Token::Bool(false), Token::Bool(false),])
    );
    let expected_array = Token::Array(vec![
        Token::String("Hello".to_string()),
        Token::String("World".to_string()),
        Token::String("Yes".to_string()),
    ]);
    assert_eq!(results[3], expected_array);
    assert_eq!(results[4], expected_array);
    assert_eq!(results[5], expected_array);
}
