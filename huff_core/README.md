## Huff Core

Core Compiler for the Huff Language.

#### Usage

The following example steps through compiling source code in the [examples](./examples/) directory.

```rust
use huff_core::Compiler;
use huff_utils::error::CompilerError;
use huff_utils::artifact::Artifact;


// Read in the ERC20 example
// let sources: Vec<FileSource> = Compiler::fetch_sources(vec![PathBuf::from("./examples/ERC20.huff")]);

// Instantiate the Compiler Instance
// The path is `../examples/ERC20.huff` since the doc-string is run from inside the `src/` directory
let mut compiler = Compiler::new(vec!["../examples/ERC20.huff".to_string()], None, None, false);

// Execute the compiler
let res: Result<Vec<Artifact>, CompilerError<'_>> = compiler.execute();
assert!(res.is_ok());
```
