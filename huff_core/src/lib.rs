#![doc = include_str!("../README.md")]

#![warn(missing_docs)]
#![warn(unused_extern_crates)]
#![forbid(unsafe_code)]
#![forbid(where_clauses_object_safety)]

use huff_codegen::*;
use huff_lexer::*;
use huff_parser::*;
use huff_utils::prelude::*;
use rayon::prelude::*;
use std::{path::{Path, PathBuf}, ffi::OsString, collections::BTreeMap, time::SystemTime};

/// The core compiler
#[derive(Default, Debug, Clone)]
pub struct Compiler {
    /// The location of the files to compile
    pub sources: Vec<String>,
    /// The output location
    pub output: Option<String>,
    /// Whether to optimize compilation or not.
    pub optimize: bool,
    /// Generate and log bytecode
    pub bytecode: bool,
}

/// Enables tracing
pub fn init_tracing_subscriber() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .try_init()
        .ok();
}

/// An aliased output location to derive from the cli arguments.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct OutputLocation(pub(crate) String);

impl Default for OutputLocation {
    fn default() -> Self {
        Self("./artifacts/".to_string())
    }
}

/// File Encapsulation
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct FileSource {
    /// File ID
    pub id: Uuid,
    /// File Path
    pub path: String,
    /// File Source
    pub source: Option<String>,
    /// Last File Access Time
    pub access: Option<SystemTime>,
}

/// File Dependencies
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct FileDependencies {
    /// The File Identifier
    pub file: Uuid,
    /// An Ordered List of File Dependencies
    pub dependencies: Vec<FileSource>
}

/// CompilerError
#[derive(Debug, PartialEq, Eq)]
pub enum CompilerError<'a> {
    /// Failed to Lex Source
    LexicalError(LexicalError<'a>),
    /// File unpacking error
    FileUnpackError(UnpackError),
    /// Parsing Error
    ParserError(ParserError),
    /// Reading PathBuf Failed
    PathBufRead(OsString),
}

impl<'a> Compiler {
    /// Public associated function to instantiate a new compiler.
    pub fn new(sources: Vec<String>, output: Option<String>) -> Self {
        if cfg!(feature="verbose") {
            init_tracing_subscriber();
        }
        Self {
            sources,
            output,
            optimize: false,
            bytecode: false
        }
    }

    /// Get the file sources for a vec of PathBufs
    pub fn fetch_sources(&self, paths: Vec<PathBuf>) -> Vec<String> {
        paths.into_par_iter().map(|pb| {
            match pb.into_os_string().into_string() {
                Ok(file_loc) => match std::fs::read_to_string(file_loc) {
                    Ok(source) => Some(source),
                    Err(e) => {
                        tracing::error!("Failed to read file at \"{}\"!", file_loc);
                        None
                    }
                }
                Err(e) => {
                    tracing::error!("Converting PathBuf \"{:?}\" Failed!", pb);
                    None
                }
            }
        }).filter(|f| f.is_some()).map(|f| f.unwrap_or_default()).collect()
    }

    /// Executes the main compilation process
    pub fn execute(&self) -> Result<(), CompilerError<'a>> {
        // Grab the input files
        let file_paths: Vec<PathBuf> = self.get_inputs()?;

        // Parallel file fetching
        let now = SystemTime::now();
        let file_sources: Vec<String> = self.fetch_sources(file_paths);

        // Parallelized Import Fetching per-compile-file
        let file_deps: BTreeMap<String, FileSources> = file_sources.into_par_iter().map(|file| {

        });
        // TODO: grab the imports and optimistically load

        // Parallel compilation
        files.into_par_iter().for_each(|file| {
            // Perform Lexical Analysis
            let lexer: Lexer = Lexer::new(&file);

            // Grab the tokens from the lexer
            let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();

            // Parser incantation
            let mut parser = Parser::new(tokens);

            // Parse into an AST
            let parse_res = parser.parse().map_err(CompilerError::ParserError);
            let contract = parse_res.unwrap();

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
    pub fn get_inputs(&self) -> Result<Vec<PathBuf>, CompilerError<'a>> {
        let mut paths = vec![];
        for f in self.sources {
            // If the file is huff, use the path, otherwise unpack
            let ext = Path::new(&f).extension().unwrap_or_default();
            if ext.eq("huff") {
                paths.push(Path::new(&f).to_path_buf())
            } else {
                // Otherwise, override the source files and use all files in the provided dir
                match unpack_files(&f) {
                    Ok(files) => files.iter().for_each(|fil| paths.push(Path::new(&f).to_path_buf())),
                    Err(e) => return Err(CompilerError::FileUnpackError(e)),
                }
            }
        }
        Ok(paths)
    }

    /// Derives an output location
    pub fn get_outputs(&self) -> OutputLocation {
        match &self.output {
            Some(o) => OutputLocation(o.clone()),
            None => OutputLocation::default(),
        }
    }
}
