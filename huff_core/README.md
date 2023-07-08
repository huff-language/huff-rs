## Huff Core

Core [Compiler](struct.Compiler.html) for the [Huff Language](https://huff.sh).

#### Usage

Compiling source code with the [Compiler](struct.Compiler.html) is very straightforward.

Once you instantiate a [Compiler](struct.Compiler.html) (WLOG, `compiler`) with the file source, you can generate the compiled artifacts by simply running:

```rust,ignore
let artifacts: Result<Vec<Artifact>, CompilerError> = compiler.execute();
```

Below we demonstrate taking a source file `../huff-examples/erc20/contracts/ERC20.huff`, and generating the copmiled artifacts.

```rust
use huff_core::Compiler;
use huff_utils::error::CompilerError;
use huff_utils::artifact::Artifact;
use huff_utils::prelude::EVMVersion;
use std::sync::Arc;
use std::cell::RefCell;
use std::rc::Rc;

// Instantiate the Compiler Instance with a targeted evm version.
let evm_version = &EVMVersion::default();
let mut compiler = Compiler::new(evm_version, Arc::new(vec!["../huff-examples/erc20/contracts/ERC20.huff".to_string()]), None, None, None, None, None, false, false);

// Execute the compiler
let res: Result<Vec<Arc<Artifact>>, Arc<CompilerError>> = compiler.execute();
assert!(res.is_ok());
```

The [Compiler](struct.Compiler.html) is easily configurable upon instantiation.

#### Inner Workings

The [Compiler](struct.Compiler.html) is composed of several compilation phases and bundles them together in one process.

```txt

[Files] -> Lexer -> Parser -> Codegen -> [Bytecode]

```
