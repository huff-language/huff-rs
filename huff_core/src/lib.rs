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
    ffi::OsString,
    fs,
    path::{Path, PathBuf},
    sync::Arc,
    time::SystemTime,
};
use tracing_subscriber::{filter::Directive, EnvFilter};
use uuid::Uuid;

/// ## The Core Huff Compiler
///
/// #### Usage
///
/// The canonical way to instantiate a Compiler instance is using the public associated
/// [new](Compiler::new) function.
///
/// Let's say we want to create a Compiler for the `ERC20.huff` contract located in [huff-examples](https://github.com/huff-language/huff-examples/blob/main/erc20/contracts/ERC20.huff).
///
/// We want our Compiler to output to an `artifacts` directory, with no constructor arguments, and
/// no verbose output:
///
/// ```rust
/// use huff_core::Compiler;
/// use std::sync::Arc;
///
/// let compiler = Compiler::new(
///     Arc::new(vec!["../huff-examples/erc20/contracts/ERC20.huff".to_string()]),
///     Some("./artifacts".to_string()),
///     None,
///     false
/// );
/// ```
#[derive(Default, Debug, Clone)]
pub struct Compiler {
    /// The location of the files to compile
    pub sources: Arc<Vec<String>>,
    /// The output location
    pub output: Option<String>,
    /// Constructor Input Arguments
    pub construct_args: Option<Vec<String>>,
    /// Whether to optimize compilation or not.
    pub optimize: bool,
    /// Generate and log bytecode
    pub bytecode: bool,
}

impl<'a> Compiler {
    /// Public associated function to instantiate a new compiler.
    pub fn new(
        sources: Arc<Vec<String>>,
        output: Option<String>,
        construct_args: Option<Vec<String>>,
        verbose: bool,
    ) -> Self {
        if cfg!(feature = "verbose") || verbose {
            Compiler::init_tracing_subscriber(Some(vec![tracing::Level::INFO.into()]));
        }
        Self { sources, output, construct_args, optimize: false, bytecode: false }
    }

    /// Tracing
    ///
    /// Creates a new tracing subscriber to span the compilation process.
    pub fn init_tracing_subscriber(directives: Option<Vec<Directive>>) {
        let subscriber_builder = tracing_subscriber::fmt();
        let mut env_filter = EnvFilter::from_default_env();
        if let Some(dv) = directives {
            for d in dv {
                env_filter = env_filter.add_directive(d);
            }
        }
        if let Err(e) = subscriber_builder.with_env_filter(env_filter).try_init() {
            println!("Failed to initialize tracing!\nError: {:?}", e)
        }
    }

    /// Executor
    ///
    /// The core compilation process.
    ///
    /// ### Steps
    ///
    /// 1. Transform inputs into File Paths with [transform_paths](Compiler::transform_paths).
    /// 2. Fetch file sources in parallel with [fetch_sources](Compiler::fetch_sources).
    /// 3. Recurse file dependencies in parallel with [recurse_deps](Compiler::recurse_deps).
    /// 4. For each top-level file [Parallelized], generate the artifact using
    /// [gen_artifact](Compiler::gen_artifact).
    /// 5. Return the compiling error(s) or successfully generated artifacts.
    pub fn execute(&self) -> Result<Vec<Arc<Artifact>>, Arc<CompilerError<'a>>> {
        // Grab the input files
        let file_paths: Vec<PathBuf> = Compiler::transform_paths(&self.sources)?;

        // Parallel file fetching
        let files: Vec<Arc<FileSource>> = Compiler::fetch_sources(&file_paths);

        // Parallel Dependency Resolution
        let recursed_file_sources: Vec<Result<Arc<FileSource>, Arc<CompilerError<'a>>>> =
            files.into_par_iter().map(Compiler::recurse_deps).collect();

        // Collect Recurse Deps errors and try to resolve to the first one
        let mut errors = recursed_file_sources
            .iter()
            .filter_map(|rfs| rfs.as_ref().err())
            .collect::<Vec<&Arc<CompilerError>>>();
        if !errors.is_empty() {
            let error = errors.remove(0);
            return Err(Arc::clone(error))
        }

        // Unpack recursed dependencies into FileSources
        let files = recursed_file_sources
            .iter()
            .filter_map(|fs| fs.as_ref().map(Arc::clone).ok())
            .collect::<Vec<Arc<FileSource>>>();
        tracing::info!(target: "core", "COMPILER RECURSED {} FILE DEPENDENCIES", files.len());

        // Parallel Compilation
        let potential_artifacts: Vec<Result<Artifact, CompilerError<'a>>> =
            files.into_par_iter().map(|f| self.gen_artifact(f)).collect();

        // Output errors + return OR print # of successfully compiled files
        let mut errors: Vec<CompilerError<'a>> = vec![];
        let mut artifacts: Vec<Arc<Artifact>> = vec![];
        for r in potential_artifacts {
            match r {
                Ok(a) => artifacts.push(Arc::new(a)),
                Err(ce) => errors.push(ce),
            }
        }
        if !errors.is_empty() {
            tracing::error!(target: "core", "{} FILES FAILED TO COMPILE", errors.len());
            return Err(Arc::new(CompilerError::FailedCompiles(errors)))
        }
        match artifacts.len() {
            0 => tracing::warn!(target: "core", "NO FILES COMPILED SUCCESSFULLY"),
            num => tracing::info!(target: "core", "{} FILES COMPILED SUCCESSFULLY", num),
        }

        // Grab the output
        let output = self.get_outputs();

        // Export
        Compiler::export_artifacts(&artifacts, &output);

        Ok(artifacts)
    }

    /// Artifact Generation
    ///
    /// Compiles a FileSource into an Artifact.
    pub fn gen_artifact(&self, file: Arc<FileSource>) -> Result<Artifact, CompilerError<'a>> {
        // Fully Flatten a file into a source string containing source code of file and all
        // its dependencies
        let flattened = FileSource::fully_flatten(Arc::clone(&file));
        tracing::info!(target: "core", "FLATTENED SOURCE FILE \"{}\"", file.path);
        let full_source = FullFileSource {
            source: &flattened.0,
            file: Some(Arc::clone(&file)),
            spans: flattened.1,
        };
        tracing::debug!(target: "core", "GOT FULL SOURCE: \"{:?}\"", full_source);

        // Perform Lexical Analysis
        // Create a new lexer from the FileSource, flattening dependencies
        let lexer: Lexer = Lexer::new(full_source);

        // Grab the tokens from the lexer
        let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
        tracing::info!(target: "core", "LEXICAL ANALYSIS COMPLETE FOR \"{}\"", file.path);
        tracing::info!(target: "core", "└─ TOKEN COUNT: {}", tokens.len());

        // Parser incantation
        let mut parser = Parser::new(tokens, Some(file.path.clone()));

        // Parse into an AST
        let parse_res = parser.parse().map_err(CompilerError::ParserError);
        let mut contract = parse_res?;
        contract.derive_storage_pointers();
        tracing::info!(target: "core", "PARSED CONTRACT [{}]", file.path);

        // Primary Bytecode Generation
        // See huffc: https://github.com/huff-language/huffc/blob/2e5287afbfdf9cc977b204a4fd1e89c27375b040/src/compiler/processor.ts
        let mut cg = Codegen::new();
        let main_bytecode = match Codegen::generate_main_bytecode(&contract) {
            Ok(mb) => mb,
            Err(mut e) => {
                // Add File Source to Span
                e.span = AstSpan(
                    e.span
                        .0
                        .iter()
                        .map(|s| {
                            let mut n_s = s.clone();
                            n_s.file = Some(Arc::clone(&file));
                            n_s
                        })
                        .collect::<Vec<Span>>(),
                );
                tracing::error!(target: "codegen", "Roll Failed with CodegenError: {:?}", e);
                return Err(CompilerError::CodegenError(e))
            }
        };
        tracing::info!(target: "core", "MAIN BYTECODE GENERATED [{}]", main_bytecode);
        let constructor_bytecode = match Codegen::generate_constructor_bytecode(&contract) {
            Ok(mb) => mb,
            Err(mut e) => {
                // Add File Source to Span
                e.span = AstSpan(
                    e.span
                        .0
                        .iter()
                        .map(|s| {
                            let mut n_s = s.clone();
                            n_s.file = Some(Arc::clone(&file));
                            n_s
                        })
                        .collect::<Vec<Span>>(),
                );
                tracing::error!(target: "codegen", "Construct Failed with CodegenError: {:?}", e);
                return Err(CompilerError::CodegenError(e))
            }
        };

        // Encode Constructor Arguments
        tracing::info!(target: "core", "CONSTRUCTOR BYTECODE GENERATED [{}]", constructor_bytecode);
        let inputs = self.get_constructor_args();
        tracing::info!(target: "core", "ENCODING {} INPUTS", inputs.len());
        let encoded_inputs = Codegen::encode_constructor_args(inputs);
        tracing::info!(target: "core", "ENCODED {} INPUTS", encoded_inputs.len());

        // Generate Artifact with ABI
        let churn_res = cg.churn(file, encoded_inputs, &main_bytecode, &constructor_bytecode);
        match churn_res {
            Ok(mut artifact) => {
                // Then we can have the code gen output the artifact
                let abiout = cg.abi_gen(contract, None);
                match abiout {
                    Ok(abi) => {
                        tracing::info!(target: "core", "GENERATED ABI");
                        artifact.abi = Some(abi)
                    }
                    Err(e) => {
                        tracing::error!(target: "core", "ARTIFACT GENERATION FAILED: {:?}", e)
                    }
                }
                Ok(artifact)
            }
            Err(e) => {
                tracing::error!(target: "core", "CODEGEN ERRORED!\nError: {:?}", e);
                Err(CompilerError::CodegenError(e))
            }
        }
    }

    /// Get the file sources for a vec of PathBufs
    pub fn fetch_sources(paths: &Vec<PathBuf>) -> Vec<Arc<FileSource>> {
        let files: Vec<Arc<FileSource>> = paths
            .into_par_iter()
            .map(|pb| {
                let file_loc = String::from(pb.to_string_lossy());
                match std::fs::read_to_string(&file_loc) {
                    Ok(source) => Some(Arc::new(FileSource {
                        id: Uuid::new_v4(),
                        path: file_loc,
                        source: Some(source),
                        access: Some(SystemTime::now()),
                        dependencies: None,
                    })),
                    Err(_) => {
                        tracing::error!(target: "core", "FILE READ FAILED: \"{}\"!", file_loc);
                        None
                    }
                }
            })
            .filter(|f| f.is_some())
            .map(|f| f.unwrap_or_default())
            .collect();
        tracing::info!(target: "core", "COMPILER FETCHED {} FILE SOURCES", files.len());
        files
    }

    /// Recurses file dependencies
    pub fn recurse_deps(fs: Arc<FileSource>) -> Result<Arc<FileSource>, Arc<CompilerError<'a>>> {
        let mut new_fs = FileSource { path: fs.path.clone(), ..Default::default() };
        let file_source = if let Some(s) = &fs.source {
            s.clone()
        } else {
            // Read from path
            let new_source = match std::fs::read_to_string(&fs.path) {
                Ok(source) => source,
                Err(_) => {
                    tracing::error!(target: "core", "FILE READ FAILED: \"{}\"!", fs.path);
                    return Err(Arc::new(CompilerError::PathBufRead(OsString::from(&fs.path))))
                }
            };
            new_fs.access = Some(SystemTime::now());
            new_source
        };
        let imports: Vec<String> = Lexer::lex_imports(&file_source);
        new_fs.source = Some(file_source);
        if !imports.is_empty() {
            tracing::info!(target: "core", "IMPORT LEXICAL ANALYSIS COMPLETE ON {:?}", imports);
        }
        let localized_imports: Vec<String> = imports
            .iter()
            .map(|import| {
                FileSource::localize_file(&fs.path, import).unwrap_or_default().replacen(
                    "contracts/contracts",
                    "contracts",
                    1,
                )
            })
            .collect();
        if !localized_imports.is_empty() {
            tracing::info!(target: "core", "LOCALIZED IMPORTS {:?}", localized_imports);
        }
        let import_bufs: Vec<PathBuf> = Compiler::transform_paths(&localized_imports)?;
        let mut file_sources: Vec<Arc<FileSource>> = Compiler::fetch_sources(&import_bufs);
        if !file_sources.is_empty() {
            tracing::info!(target: "core", "FETCHED {} FILE SOURCES", file_sources.len());
        }

        // Now that we have all the file sources, we have to recurse and get their source
        file_sources = file_sources
            .into_par_iter()
            .map(|inner_fs| match Compiler::recurse_deps(Arc::clone(&inner_fs)) {
                Ok(new_fs) => new_fs,
                Err(e) => {
                    tracing::error!(target: "core", "NESTED DEPENDENCY RESOLUTION FAILED: \"{:?}\"", e);
                    Arc::clone(&inner_fs)
                }
            })
            .collect();

        // Finally set the parent deps
        new_fs.dependencies = Some(file_sources);

        Ok(Arc::new(new_fs))
    }

    /// Export Artifacts
    ///
    /// 1. Cleans any previous artifacts in the output directory.
    /// 2. Exports artifacts in parallel as serialized json `Artifact` objects.
    pub fn export_artifacts(artifacts: &Vec<Arc<Artifact>>, output: &OutputLocation) {
        // Clean the Output Directory
        tracing::warn!(target: "core", "REMOVING DIRECTORY: \"{}\"", output.0);
        if !output.0.is_empty() && fs::remove_dir_all(&output.0).is_ok() {
            tracing::info!(target: "core", "OUTPUT DIRECTORY DELETED!");
        }

        // Export the artifacts with parallelized io
        artifacts.into_par_iter().for_each(|a| {
            let json_out =
                format!("{}/{}.json", output.0, a.file.path.to_uppercase().replacen("./", "", 1));
            tracing::debug!(target: "core", "JSON OUTPUT: {:?}", json_out);

            if let Err(e) = a.export(&json_out) {
                tracing::error!(target: "core", "ARTIFACT EXPORT FAILED!\nError: {:?}", e);
            }
            tracing::info!(target: "core", "EXPORTED ARTIFACT TO \"{}\"", json_out);
        });
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
                    Err(e) => {
                        tracing::error!(target: "core", "ERROR UNPACKING FILE: {:?}", e);
                        return Err(CompilerError::FileUnpackError(e))
                    }
                }
            }
        }
        Ok(paths)
    }

    /// Derives Constructor Input Arguments
    pub fn get_constructor_args(&self) -> Vec<String> {
        match &self.construct_args {
            Some(construct_args) => construct_args.clone(),
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
