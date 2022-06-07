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

#![warn(missing_docs)]
#![warn(unused_extern_crates)]
#![forbid(unsafe_code)]
#![forbid(where_clauses_object_safety)]

use huff_utils::{abi::*, artifact::*, ast::*, error::CodegenError};
use std::io::{self, Write};

/// ### Codegen
///
/// Code Generation Manager responsible for generating the code for the Huff Language.
pub struct Codegen<'a> {
    /// Whether to output the abi
    pub abiout: bool,
    /// The Input AST
    pub ast: Option<Contract<'a>>,
    /// A cached codegen output artifact
    pub artifact: Option<Artifact>,
}

impl<'a> Codegen<'a> {
    /// Public associated function to instantiate a new Codegen instance.
    pub fn new(abiout: bool) -> Self {
        Self { abiout, ast: None, artifact: None }
    }

    /// Generate a codegen artifact
    ///
    /// Takes in a vector of strings representing constructor arguments.
    pub fn churn(&mut self, args: Vec<String>) -> Result<Artifact, CodegenError<'a>> {
        let mut artifact = Artifact::default();

        // TODO: actually generate the bytecode
        // TODO: see huffc: https://github.com/huff-language/huffc/blob/2e5287afbfdf9cc977b204a4fd1e89c27375b040/src/compiler/processor.ts
        let main_bytecode = "";
        let constructor_bytecode = "";

        let contract_length = main_bytecode.len() / 2;
        let constructor_length = constructor_bytecode.len() / 2;

        let contract_size = contract_length; // padNBytes(toHex(contractLength), 2);
        let contract_code_offset = constructor_length; // padNBytes(toHex(13 + constructorLength), 2);

        // TODO: Properly encode the args
        let constructor_args =
            args.iter().fold("".to_string(), |acc, arg| format!("{},{}", acc, arg));

        let bootstrap_code = format!("61{}8061{}6000396000f3", contract_size, contract_code_offset);
        let constructor_code = format!("{}{}", constructor_bytecode, bootstrap_code);
        artifact.bytecode = format!("{}{}{}", constructor_code, main_bytecode, constructor_args);
        Ok(artifact)
    }

    /// #### `write`
    ///
    /// Write the generated code to the output writer.
    pub fn write(&self, ast: &Contract) -> Result<Vec<u8>, CodegenError> {
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
    pub fn export(&self, ast: &Contract, target: &str, input: &str) -> Result<(), CodegenError> {
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
    pub fn abigen(&mut self, ast: Contract<'a>) -> Result<Abi, CodegenError> {
        let abi: Abi = ast.into();

        // Set the abi on self
        match &mut self.artifact {
            Some(artifact) => {
                artifact.abi = Some(abi.clone());
            }
            None => {
                self.artifact = Some(Artifact { abi: Some(abi.clone()), ..Default::default() });
            }
        }

        // Return the abi
        Ok(abi)
    }
}
