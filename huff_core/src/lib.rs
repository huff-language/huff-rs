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
use std::{
    path::{Path, PathBuf},
    time::SystemTime,
};
use uuid::Uuid;

/// The core compiler
#[derive(Default, Debug, Clone)]
pub struct Compiler {
    /// The location of the files to compile
    pub sources: Vec<String>,
    /// The output location
    pub output: Option<String>,
    /// Constructor Input Arguments
    pub inputs: Option<Vec<String>>,
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
    pub fn new(sources: Vec<String>, output: Option<String>, inputs: Option<Vec<String>>, verbose: bool) -> Self {
        if cfg!(feature = "verbose") || verbose {
            init_tracing_subscriber();
        }
        Self { sources, output, inputs, optimize: false, bytecode: false }
    }

    /// Get the file sources for a vec of PathBufs
    pub fn fetch_sources(paths: &Vec<PathBuf>) -> Vec<FileSource> {
        paths
            .into_par_iter()
            .map(|pb| match pb.clone().into_os_string().into_string() {
                Ok(file_loc) => match std::fs::read_to_string(file_loc.clone()) {
                    Ok(source) => Some(FileSource {
                        id: Uuid::new_v4(),
                        path: file_loc,
                        source: Some(source),
                        access: Some(SystemTime::now()),
                        dependencies: None,
                    }),
                    Err(_) => {
                        tracing::error!("Failed to read file at \"{}\"!", file_loc);
                        None
                    }
                },
                Err(_) => {
                    tracing::error!("Converting PathBuf \"{:?}\" Failed!", pb);
                    None
                }
            })
            .filter(|f| f.is_some())
            .map(|f| f.unwrap_or_default())
            .collect()
    }

    /// Recurses file dependencies
    pub fn recurse_deps(fs: FileSource) -> Result<FileSource, CompilerError<'a>> {
        println!("Inside recurse deps with file source: {:?}", fs.path);
        let mut new_fs = fs.clone();
        let file_source = if let Some(s) = &fs.source {
            s.clone()
        } else {
            // Read from path
            let new_source = match std::fs::read_to_string(fs.path.clone()) {
                Ok(source) => source,
                Err(_) => {
                    tracing::error!("Failed to read file at \"{}\"!", fs.path);
                    return Err(CompilerError::PathBufRead(fs.path.clone().into()))
                }
            };
            new_fs.source = Some(new_source.clone());
            new_fs.access = Some(SystemTime::now());
            new_source
        };
        let imports: Vec<String> = Lexer::lex_imports(&file_source);
        println!("Got lexed imports: {:?}", imports);
        let localized_imports = imports
            .iter()
            .map(|import| {
                FileSource::localize_file(&fs.path, import).unwrap_or_default().replacen(
                    "contracts/contracts",
                    "contracts",
                    1,
                )
            })
            .collect();
        println!("Localized imports: {:?}", localized_imports);
        let import_bufs: Vec<PathBuf> = Compiler::transform_paths(&localized_imports)?;
        let mut file_sources: Vec<FileSource> = Compiler::fetch_sources(&import_bufs);

        // Now that we have all the file sources, we have to recurse and get their source
        file_sources = file_sources
            .into_par_iter()
            .map(|inner_fs| match Compiler::recurse_deps(inner_fs.clone()) {
                Ok(new_fs) => new_fs,
                Err(e) => {
                    tracing::error!("Failed to resolve nested dependencies with error \"{:?}\"", e);
                    inner_fs
                }
            })
            .collect();

        // Finally set the parent deps
        new_fs.dependencies = Some(file_sources);

        Ok(new_fs)
    }

    /// Executes the main compilation process
    pub fn execute(&self) -> Result<Vec<Result<Artifact, CompilerError<'a>>>, CompilerError<'a>> {
        // Grab the input files
        let file_paths: Vec<PathBuf> = Compiler::transform_paths(&self.sources)?;

        // Parallel file fetching
        let mut files: Vec<FileSource> = Compiler::fetch_sources(&file_paths);

        // Parallel Dependency Resolution
        files = files
            .into_par_iter()
            .map(|f| match Compiler::recurse_deps(f) {
                Ok(fs) => Some(fs),
                Err(_) => None,
            })
            .filter(|fs| fs.is_some())
            .map(|fs| fs.unwrap())
            .collect();

        // Parallel Compilation
        let artifacts: Vec<Result<Artifact, CompilerError<'a>>> = files
            .into_par_iter()
            .map(|file| {
                // Fully Flatten a file into a source string containing source code of file and all
                // its dependencies
                let full_source = file.fully_flatten();

                // Perform Lexical Analysis
                // Create a new lexer from the FileSource, flattening dependencies
                let lexer: Lexer = Lexer::new(&full_source);

                // Grab the tokens from the lexer
                let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();

                // Parser incantation
                let mut parser = Parser::new(tokens);

                // Parse into an AST
                let parse_res = parser.parse().map_err(CompilerError::ParserError);
                let contract = parse_res.unwrap();

                // Run code generation
                let mut cg = Codegen::new();
                // cg.ast = Some(contract);

                // Gracefully derive the output from the cli
                let output: OutputLocation = self.get_outputs();

                // Codegen. See huffc: https://github.com/huff-language/huffc/blob/2e5287afbfdf9cc977b204a4fd1e89c27375b040/src/compiler/processor.ts
                let main_bytecode = match Codegen::roll(Some(contract.clone())) {
                    Ok(mb) => mb,
                    Err(e) => return Err(CompilerError::CodegenError(e)),
                };
                let constructor_bytecode = match Codegen::construct(Some(contract.clone())) {
                    Ok(mb) => mb,
                    Err(e) => return Err(CompilerError::CodegenError(e)),
                };
                let inputs = self.get_inputs();
                let encoded_inputs = Codegen::encode_constructor_args(inputs);
                let churn_res = cg.churn(encoded_inputs, &main_bytecode, &constructor_bytecode);
                match churn_res {
                    Ok(mut artifact) => {
                        println!("Successfully compiled {:?}!", file);
                        // Then we can have the code gen output the artifact
                        let abiout = cg.abigen(contract, None);
                        match abiout {
                            Ok(abi) => artifact.abi = Some(abi),
                            Err(e) => {
                                tracing::error!("Failed to generate artifact!\nError: {:?}", e)
                            }
                        }
                        let json_out = format!("./{}/{}.json", output.0, file.path.to_uppercase());
                        if let Err(e) = artifact.export(json_out) {
                            tracing::error!("Exporting artifact failed with error: {:?}", e);
                        }
                        Ok(artifact)
                    }
                    Err(e) => {
                        tracing::error!("Failed to compile!\nError: {:?}", e);
                        Err(CompilerError::CodegenError(e))
                    }
                }
            })
            .collect();

        Ok(artifacts)
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
                match unpack_files(f) {
                    Ok(files) => {
                        files.iter().for_each(|fil| paths.push(Path::new(&fil).to_path_buf()))
                    }
                    Err(e) => return Err(CompilerError::FileUnpackError(e)),
                }
            }
        }
        Ok(paths)
    }

    /// Derives Constructor Input Arguments
    pub fn get_inputs(&self) -> Vec<String> {
        match &self.inputs {
            Some(inputs) => inputs.clone(),
            None => vec![],
        }
    }

    /// Derives an output location
    pub fn get_outputs(&self) -> OutputLocation {
        match &self.output {
            Some(o) => OutputLocation(o.clone()),
            None => OutputLocation::default(),
        }
    }
}
