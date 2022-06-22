## Codegen

Bytecode Generation Module for the Huff Language.

#### Architecture

The `huff_codegen` module exposes a few main bytecode generation functions. It is expected that both the [huff_lexer](../huff_lexer/) and [huff_parser](../huff_parser/) are executed before `huff_codegen` to produce a valid abstract syntax tree ([Contract](../huff_utils/ast/struct.Contract.html) in our case) that `huff_codegen` can _walk_.

Once the AST ([Contract](../huff_utils/ast/struct.Contract.html)) is produced, [Codegen](struct.Codegen.html) can be used to produce the **MAIN** and **CONSTRUCTOR** bytecode.

The `generate_main_bytecode` function takes a reference of [Contract](../huff_utils/ast/struct.Contract.html) and produces a bytecode `String` on success or a [CodegenError](../huff_utils/error/struct.CodegenError.html) on failure.

Likewise, the `generate_constructor_bytecode` function takes a reference of [Contract](../huff_utils/ast/struct.Contract.html) and produces a bytecode `String` on success or a [CodegenError](../huff_utils/error/struct.CodegenError.html) on failure.

`churn` takes the generated **CONSTRUCTOR** and **MAIN** macros' bytecode and produces an [Artifact](../huff_utils/artifact/struct.Artifact.html) containing:
- The file source: `Artifact.file`
- The deployed bytecode: `Artifact.deployed`
- The runtime bytecode: `Artifact.runtime`
- The contract ABI: `Artifact.abi`


#### Usage

Below we showcase generating a compile artifact from compiled bytecode using `huff_codegen`.

```rust
use huff_codegen::*;
use huff_utils::files::FileSource;
use std::sync::Arc;

// Instantiate an empty Codegen
let mut cg = Codegen::new();
assert!(cg.ast.is_none());
assert!(cg.artifact.is_none());

// ERC20 Bytecode
let main_bytecode = "60003560E01c8063a9059cbb1461004857806340c10f19146100de57806370a082311461014e57806318160ddd1461016b578063095ea7b314610177578063dd62ed3e1461018e575b600435336024358160016000526000602001526040600020548082116100d8578190038260016000526000602001526040600020558281906001600052600060200152604060002054018360016000526000602001526040600020556000527fDDF252AD1BE2C89B69C2B068FC378DAA952BA7F163C4A11628F55A4DF523B3EF60206000a3600160005260206000f35b60006000fd5b60005433146100ed5760006000fd5b600435600060243582819060016000526000602001526040600020540183600160005260006020015260406000205580600254016002556000527fDDF252AD1BE2C89B69C2B068FC378DAA952BA7F163C4A11628F55A4DF523B3EF60206000a35b600435600160005260006020015260406000205460005260206000f35b60025460005260206000f35b602435600435336000526000602001526040600020555b60243560043560005260006020015260406000205460005260206000f3";
let constructor_bytecode = "33600055";
let inputs = vec![];
let churn_res = cg.churn(Arc::new(FileSource::default()), inputs, main_bytecode, constructor_bytecode);

// Validate the output bytecode
assert_eq!(churn_res.unwrap().bytecode, "336000556101ac806100116000396000f360003560E01c8063a9059cbb1461004857806340c10f19146100de57806370a082311461014e57806318160ddd1461016b578063095ea7b314610177578063dd62ed3e1461018e575b600435336024358160016000526000602001526040600020548082116100d8578190038260016000526000602001526040600020558281906001600052600060200152604060002054018360016000526000602001526040600020556000527fDDF252AD1BE2C89B69C2B068FC378DAA952BA7F163C4A11628F55A4DF523B3EF60206000a3600160005260206000f35b60006000fd5b60005433146100ed5760006000fd5b600435600060243582819060016000526000602001526040600020540183600160005260006020015260406000205580600254016002556000527fDDF252AD1BE2C89B69C2B068FC378DAA952BA7F163C4A11628F55A4DF523B3EF60206000a35b600435600160005260006020015260406000205460005260206000f35b60025460005260206000f35b602435600435336000526000602001526040600020555b60243560043560005260006020015260406000205460005260206000f3".to_lowercase());

// Write the compile artifact out to a file
// cg.export("./output.json");
```