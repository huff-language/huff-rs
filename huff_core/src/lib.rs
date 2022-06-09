//! ## Huff Core
//!
//! Core Compiler for the Huff Language.
//!
//! #### Usage
//!
//! The following example steps through compiling source code in the ./examples/ directory.
//!
//! ```rust
//! use huff_core::{Compiler};
//! use huff_lexer::{Lexer};
//!
//! // Instantiate the Compiler Instance
//! let compiler = Compiler::new();
//!
//! // Mock run the compiler on examples
//! ```

#![warn(missing_docs)]
#![warn(unused_extern_crates)]
#![forbid(unsafe_code)]
#![forbid(where_clauses_object_safety)]

use huff_codegen::*;
use huff_lexer::*;
use huff_parser::*;
use huff_utils::prelude::*;
use rayon::prelude::*;
use std::path::Path;

/// The core compiler
pub struct Compiler {

}

/// An aliased output location to derive from the cli arguments.
#[derive(Debug, Clone)]
pub struct OutputLocation(pub(crate) String);

/// CompilerError
#[derive(Debug, PartialEq, Eq)]
pub enum CompilerError<'a> {
    /// Failed to Lex Source
    LexicalError(LexicalError<'a>),
    /// File unpacking error
    FileUnpackError(UnpackError),
    /// Parsing Error
    ParserError(ParserError),
}

impl<'a> Compiler {
    /// Executes the main compilation process
    pub fn execute(&self) -> Result<(), CompilerError<'a>> {
        // Grab the input files
        let files: Vec<String> = self.get_inputs()?;

        // Parallel compilation
        files.into_par_iter().for_each(|file| {
            // Perform Lexical Analysis
            let lexer: Lexer = Lexer::new(&file);

            // Grab the tokens from the lexer
            let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();

            // Parser incantation
            let mut parser = Parser::new(tokens);

            // Parse into an AST
            let parse_res = parser.parse().map_err(ExecutionError::ParserError);
            let contract = parse_res.unwrap();
            println!("Parsed AST: {:?}", contract);

            // Run code generation
            let mut cg = Codegen::new();

            // Gracefully derive the output from the cli
            let output: OutputLocation = self.get_outputs();

            // TODO: actually generate the bytecode
            // TODO: see huffc: https://github.com/huff-language/huffc/blob/2e5287afbfdf9cc977b204a4fd1e89c27375b040/src/compiler/processor.ts
            let main_bytecode = "";
            let constructor_bytecode = "";
            let inputs = vec![];
            let churn_res = cg.churn(inputs, main_bytecode, constructor_bytecode);
            match churn_res {
                Ok(_) => {
                    println!("Successfully compiled {}!", file);
                    // Then we can have the code gen output the artifact
                    let abiout = cg.abigen(
                        contract,
                        Some(format!("./{}/{}.json", output.0, file.to_uppercase())),
                    );
                    if let Err(e) = abiout {
                        tracing::error!("Failed to generate artifact!\nError: {:?}", e);
                    }
                }
                Err(e) => {
                    println!("Failed to compile!\nError: {:?}", e);
                    tracing::error!("Failed to compile!\nError: {:?}", e);
                }
            }
        });

        Ok(())
    }

    /// Preprocesses input files for compiling
    pub fn get_inputs(&self) -> Result<Vec<String>, ExecutionError<'a>> {
        match &self.path {
            Some(path) => {
                // If the file is huff, we can use it
                let ext = Path::new(&path).extension().unwrap_or_default();
                if ext.eq("huff") {
                    Ok(vec![path.clone()])
                } else {
                    // Otherwise, override the source files and use all files in the provided dir
                    unpack_files(path).map_err(ExecutionError::FileUnpackError)
                }
            }
            None => {
                // If there's no path, unpack source files
                let source: String = self.source.clone();
                unpack_files(&source).map_err(ExecutionError::FileUnpackError)
            }
        }
    }

    /// Derives an output location
    pub fn get_outputs(&self) -> OutputLocation {
        match &self.output {
            Some(o) => OutputLocation(o.clone()),
            None => OutputLocation(self.outputdir.clone()),
        }
    }
}
