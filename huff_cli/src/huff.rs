//! Huff
//!
//! The Huff Compiler CLI.

#![allow(dead_code)]
#![allow(clippy::enum_variant_names)]

use clap::Parser as ClapParser;
use huff_codegen::*;
use huff_lexer::*;
use huff_parser::*;
use huff_utils::prelude::*;
use rayon::prelude::*;
use std::path::Path;

/// Efficient Huff compiler.
#[derive(ClapParser, Debug, Clone)]
#[clap(version, about, long_about = None)]
pub struct Huff {
    path: Option<String>,

    /// The source path to the contracts (default: "./src").
    #[clap(short = 's', long = "source-path", default_value = "./src")]
    source: String,

    /// The output file path.
    #[clap(short = 'o', long = "output")]
    output: Option<String>,

    /// The output directory (default: "./artifacts").
    #[clap(short = 'd', long = "output-directory", default_value = "./artifacts")]
    outputdir: String,

    /// Optimize compilation.
    #[clap(short = 'z', long = "optimize")]
    optimize: bool,

    /// Generate and log bytecode (default: false).
    #[clap(short = 'b', long = "bytecode")]
    bytecode: bool,

    /// Print the output to the terminal.
    #[clap(short = 'p', long = "print")]
    print: bool,
}

/// An aliased output location to derive from the cli arguments.
#[derive(Debug, Clone)]
pub struct OutputLocation(pub(crate) String);

/// ExecutionError
#[derive(Debug, PartialEq, Eq)]
pub enum ExecutionError<'a> {
    /// Failed to Lex Source
    LexicalError(LexicalError<'a>),
    /// File unpacking error
    FileUnpackError(UnpackError),
    /// Parsing Error
    ParserError(ParserError),
}

impl<'a> Huff {
    /// Executes the main compilation process
    pub fn execute(&self) -> Result<(), ExecutionError<'a>> {
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
            let cg = Codegen::new(true);
            println!("Created a new codegen instance");

            let write_res = cg.write(&contract);
            println!("Codegen writing result: {:?}", write_res);

            // Gracefully derive the output from the cli
            let output: OutputLocation = self.get_outputs();
            println!("Cli got output location: {:?}", output);

            let export_res = cg.export(&contract, &output.0, "INPUT");
            println!("Codegen export result: {:?}", export_res);
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
