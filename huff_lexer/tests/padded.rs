use huff_lexer::Lexer;
use huff_parser::*;
use huff_utils::{evm::Opcode, prelude::*};

#[cfg(test)]
use std::println as info;

#[test]
fn padded_with_simple_body() {
    let source =
        "#define macro HELLO_WORLD() = takes(3) returns(0) {\n #define padded(32) {\n 0x00 mstore\n 0x01 0x02 add \n} \n}";
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source.source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    println!("{:#?}", tokens);
}
