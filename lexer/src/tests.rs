
use proptest::prelude::*;

use crate::{
    Lexer,
    Span,
};

// proptest! {
//     #[test]
//     fn doesnt_crash(s in "\\PC*") {
//         parse_date(&s);
//     }
// }

#[test]
fn instantiates() {
    let source = "";
    let lexer = Lexer::new(source);
    assert_eq!(lexer.source, source);
    assert_eq!(lexer.span, Span::default());
    assert!(!lexer.eof);
}