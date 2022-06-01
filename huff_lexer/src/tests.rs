use proptest::prelude::*;

use crate::{Lexer, Span};

// proptest! {
//     #[test]
//     fn doesnt_crash(s in "\\PC*") {
//         parse_date(&s);
//     }
// }

#[test]
fn instantiates() {
    let source = "#define macro HELLO_WORLD()";
    let lexer = Lexer::new(source);
    assert_eq!(lexer.source, source);
    // let tok = lexer.next();
    // println!("{:?}", tok);
    assert_eq!(lexer.span, Span::default());
}
