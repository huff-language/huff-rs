use huff_lexer::Lexer;
use huff_utils::prelude::*;

#[cfg(test)]
use std::println as info;

#[test]
fn padded_with_simple_body() {
    let source =
        "#define macro HELLO_WORLD() = takes(3) returns(0) {\n #define padded(32) {\n 0x00 mstore\n 0x01 0x02 add \n} 0x69 0x69 return\n}";
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source.source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    info!("{:#?}", tokens);
}
