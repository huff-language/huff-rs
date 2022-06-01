//! ## Codegen
//!
//! Code Generation Module for the Huff Language.
//!
//! #### Usage
//!
//! ```rust
//! use huff_codegen::*;
//!
//! let mut cg = Codegen::new(false);
//! assert!(!cg.abiout);
//! ```

#![deny(missing_docs)]

use huff_utils::{error::CodegenError, prelude::Abi};
use std::io::{self, Write};

/// A MOCK AST Struct
/// WARN: Should be deleted and use parser::Ast instead!
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Ast {
    /// Expressions
    pub exprs: Vec<String>,
}

impl Ast {
    /// Public associated function to instatiate a new Ast.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { exprs: vec![] }
    }
}

/// ### Codegen
///
/// Code Generation Manager responsible for generating the code for the Huff Language.
pub struct Codegen {
    /// Whether to output the abi
    pub abiout: bool,
}

impl Codegen {
    /// Public associated function to instantiate a new Codegen instance.
    pub fn new(abiout: bool) -> Self {
        Self { abiout }
    }

    /// #### `write`
    ///
    /// Write the generated code to the output writer.
    pub fn write(&self, ast: &Ast) -> Result<Vec<u8>, CodegenError> {
        let out = Vec::new();
        // self.entry();
        // self.start_main();

        // for expr in &ast.exprs {
        //     self.expr(expr)?;
        // }

        // self.end_main();

        // TODO::::

        println!("Writer got ast: {:?}", ast);

        Ok(out)
    }

    /// #### `export`
    ///
    /// Exports the output to the specified target file.
    pub fn export(&self, ast: &Ast, target: &str, input: &str) -> Result<(), CodegenError> {
        let out = self.write(ast)?;

        // TODO: validate target is in format `./target/`
        // TODO: add additional ending slash if needed

        match std::fs::create_dir(target) {
            Err(err) if err.kind() != io::ErrorKind::AlreadyExists => {
                panic!("failed to create target directory: {}", err)
            }
            _ => {}
        };

        // let hash = {
        //     let mut hasher = DefaultHasher::new();
        //     hasher.write_u64(rand::thread_rng().next_u64());
        //     hasher.finish()
        // };

        let asm_file = format!("{}{}.s", target, input);
        let out_file = format!("{}{}.o", target, input);

        std::fs::File::create(&asm_file)
            .expect("failed to open output file")
            .write_all(&out)
            .expect("failed to write output");

        std::process::Command::new("as")
            .arg(&asm_file)
            .arg("-g")
            .arg("-o")
            .arg(&out_file)
            .status()
            .expect("failed to assemble output");

        std::process::Command::new("ld")
            .arg("-o")
            .arg("out")
            .arg("--dynamic-linker")
            .arg("/lib64/ld-linux-x86-64.so.2")
            .arg(&out_file)
            .arg("-lc")
            .status()
            .expect("linking failed");

        Ok(())
    }

    /// #### `abigen`
    ///
    /// Generates an ABI for the given Ast.
    pub fn abigen(&self, _ast: &Ast) -> Result<Abi, CodegenError> {
        let abi = Abi::new();

        // TODO: Construct the abi using the ast

        Ok(abi)

        // The ABI array.
        // const abi = [];

        // // Add the functions to the ABI.
        // Object.keys(functions).forEach((name) => {
        //     // Get the function definition
        //     const { inputs, outputs, type } = functions[name].data;

        //     // Push the definition to the ABI.
        //     abi.push({
        //     name: name,
        //     type: "function",
        //     stateMutability: type,
        //     payable: type === "payable" ? true : false,
        //     inputs: inputs.map((type) => {
        //         return { name: "", type };
        //     }),
        //     outputs: outputs.map((type) => {
        //         return { name: "", type };
        //     }),
        //     });
        // });

        // // Add the events to the ABI.
        // Object.keys(events).forEach((name) => {
        //     // Get the event definition.
        //     const inputs = events[name].args;

        //     abi.push({
        //     name: name,
        //     type: "event",
        //     anonymous: false,
        //     inputs: inputs.map((type) => {
        //         let indexed;
        //         if (type.endsWith(" indexed")) {
        //         indexed = true;
        //         type = type.replace(" indexed", "");
        //         }

        //         return { name: "", type, indexed };
        //     }),
        //     });
        // });

        // // Return the ABI.
        // return JSON.stringify(abi, null, 2);
    }
}
