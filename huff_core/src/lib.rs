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
use uuid::Uuid;
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
    pub fn fetch_sources(paths: &Vec<PathBuf>) -> Vec<FileSource> {
        paths.into_par_iter().map(|pb| {
            match pb.into_os_string().into_string() {
                Ok(file_loc) => match std::fs::read_to_string(file_loc) {
                    Ok(source) => Some(FileSource {
                        id: Uuid::new_v4(),
                        path: file_loc,
                        source: Some(source),
                        access: Some(SystemTime::now()),
                        dependencies: None
                    }),
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

    /// Recurses file dependencies
    pub fn recurse_deps(fs: &mut FileSource) -> Result<FileSource, CompilerError<'a>> {
        let file_source = if let Some(s) = fs.source {
            s
        } else {
            // Read from path
            let new_source = match std::fs::read_to_string(fs.path) {
                Ok(source) => source,
                Err(e) => {
                    tracing::error!("Failed to read file at \"{}\"!", file_loc);
                    return Err(CompilerError::PathBufRead(fs.path.into()))
                }
            };
            fs.source = Some(new_source);
            fs.access = Some(SystemTime::new())
        };
        let imports: Vec<String> = Lexer::lex_imports(file_source);
        let import_bufs: Vec<PathBuf> = Compiler::transform_paths(&imports)?;
        let mut file_sources: Vec<FileSource> = Compiler::fetch_sources(&import_bufs);

        // Now that we have all the file sources, we have to recurse and get their source
        file_sources = file_sources.into_par_iter().map(|inner_fs| {
            match Compiler::recurse_deps(&mut inner_fs) {
                Ok(new_fs) => new_fs,
                Err(_) => {
                    tracing::error!("Failed to resolve nested dependencies for file \"{}\"", inner_fs.path);
                    inner_fs
                }
            }
        }).collect();

        // Finally set the parent deps
        fs.dependencies = Some(file_sources);
    }

    /// Executes the main compilation process
    pub fn execute(&self) -> Result<(), CompilerError<'a>> {
        // Grab the input files
        let file_paths: Vec<PathBuf> = Compiler::transform_paths(&self.sources)?;

        // Parallel file fetching
        let mut files: Vec<FileSource> = Compiler::fetch_sources(&file_paths);

        // Parallelized Dependency Fetching
        files = files.into_par_iter().map(recurse_deps).collect();

        // Parallel compilation
        files.into_par_iter().for_each(|file| {
            // Perform Lexical Analysis
            // Create a new lexer from the FileSource, flattening dependencies
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

    /// Transforms File Strings into PathBufs
    pub fn transform_paths(sources: &Vec<String>) -> Result<Vec<PathBuf>, CompilerError<'a>> {
        let mut paths = vec![];
        for f in sources {
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
