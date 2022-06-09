## Huff Core

Core Compiler for the Huff Language.

#### Usage

The following example steps through compiling source code in the [examples](./examples/) directory.

```rust
use huff_core::{Compiler};

// Instantiate the Compiler Instance
let compiler = Compiler::new();

// Feed the compiler the examples
compiler.

// Execute the compiler
let res: Result<(), CompilerError<'_>> = compiler.execute();
assert!(res.is_ok());
```
