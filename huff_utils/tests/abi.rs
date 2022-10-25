use huff_utils::abi::*;
use std::collections::HashMap;

#[test]
fn convert_function_param_type() {
    let func_types = [
        "bool",
        "string",
        "address",
        "uint198",
        "bytes32",
        "bytes",
        "uint[][]",
        "address[5][]",
        "bool[][1][][2]",
    ];

    let expected_fn_types = HashMap::from([
        (0, FunctionParamType::Bool),
        (1, FunctionParamType::String),
        (2, FunctionParamType::Address),
        (3, FunctionParamType::Uint(198)),
        (4, FunctionParamType::FixedBytes(32)),
        (5, FunctionParamType::Bytes),
        (6, FunctionParamType::Array(Box::new(FunctionParamType::Uint(256)), vec![0, 0])),
        (7, FunctionParamType::Array(Box::new(FunctionParamType::Address), vec![5, 0])),
        (8, FunctionParamType::Array(Box::new(FunctionParamType::Bool), vec![0, 1, 0, 2])),
    ]);

    for (index, func_type) in func_types.into_iter().enumerate() {
        let ac_func_type = FunctionParamType::convert_string_to_type(func_type).unwrap();
        assert_eq!(ac_func_type, *expected_fn_types.get(&index).unwrap());
    }
}

#[test]
fn test_display_func_param_type() {
    let func_types = [
        "bool",
        "string",
        "address",
        "uint198",
        "uint",
        "bytes32",
        "bytes",
        "uint256[][]",
        "address[5][]",
        "bool[][1][][2]",
    ];

    let expected_fn_types = HashMap::from([
        (0, FunctionParamType::Bool),
        (1, FunctionParamType::String),
        (2, FunctionParamType::Address),
        (3, FunctionParamType::Uint(198)),
        (4, FunctionParamType::Uint(256)),
        (5, FunctionParamType::FixedBytes(32)),
        (6, FunctionParamType::Bytes),
        (7, FunctionParamType::Array(Box::new(FunctionParamType::Uint(256)), vec![0, 0])),
        (8, FunctionParamType::Array(Box::new(FunctionParamType::Address), vec![5, 0])),
        (9, FunctionParamType::Array(Box::new(FunctionParamType::Bool), vec![0, 1, 0, 2])),
    ]);

    for (index, func_type) in func_types.into_iter().enumerate() {
        let ac_func_type = FunctionParamType::convert_string_to_type(func_type).unwrap();
        assert_eq!(ac_func_type, *expected_fn_types.get(&index).unwrap());
        // manual check for uints since "uint" is coalesced to "uint256"
        match ac_func_type {
            FunctionParamType::Uint(s) => {
                assert_eq!(ac_func_type.to_string(), format!("uint{s}"));
                assert_eq!(format!("{ac_func_type:?}"), format!("uint{s}"));
            }
            _ => {
                assert_eq!(ac_func_type.to_string(), func_types[index]);
                assert_eq!(format!("{ac_func_type:?}"), func_types[index]);
            }
        }
    }
}
