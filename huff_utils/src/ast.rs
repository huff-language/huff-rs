type Literal = [u8; 32];
type FilePath = &str;

struct Contract<'a> {
    macros: Vec<MacroDefinition<'a>>,
    invocations: Vec<MacroInvocation<'a>>
    imports: Vec<FilePath>;
}

struct MacroDefinition<'a> {
    name: String,
    arguments: Vec<String>,
    statements: Vec<Statement<'a>>,
    takes: usize,
    returns: usize,
}

struct MacroInvocation<'a> {
    macro_name: String,
    args: Vec<&'a Literal>,
}

enum Statement<'a> {
    Literal(Literal),
    Opcode,
    MacroInvocation(MacroInvocation<'a>),
}