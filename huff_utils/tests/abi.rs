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
        let ac_func_type = FunctionParamType::convert_string_to_type(func_type);
        assert_eq!(ac_func_type, *expected_fn_types.get(&index).unwrap());
    }
}
