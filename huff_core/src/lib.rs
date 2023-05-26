#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![warn(unused_extern_crates)]
#![forbid(unsafe_code)]
#![forbid(where_clauses_object_safety)]

use ethers_core::utils::hex;
use huff_codegen::*;
use huff_lexer::*;
use huff_parser::*;
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use huff_utils::wasm::IntoParallelIterator;
use huff_utils::{
    file_provider::{FileProvider, FileSystemFileProvider, InMemoryFileProvider},
    prelude::*,
    time,
};
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
use rayon::prelude::*;
use std::{
    collections::{BTreeMap, HashMap},
    ffi::OsString,
    fs,
    iter::Iterator,
    path::PathBuf,
    sync::Arc,
};
use tracing_subscriber::{filter::Directive, EnvFilter};

pub(crate) mod cache;

/// ## The Core Huff Compiler
///
/// #### Usage
///
/// The canonical way to instantiate a Compiler instance is using the public associated
/// [new](Compiler::new) function.
///
/// Let's say we want to create a Compiler for the `ERC20.huff` contract located in [huff-examples](https://github.com/huff-language/huff-examples/blob/main/erc20/contracts/ERC20.huff).
///
/// We want our Compiler to output to an `artifact:Vec<ethers_core::abi::token::Token>s` directory,
/// with no constructor arguments, and no verbose output:
///
/// ```rust
/// use huff_core::Compiler;
/// use huff_utils::prelude::EVMVersion;
/// use std::sync::Arc;
///
/// let compiler = Compiler::new(
///     &EVMVersion::default(),
///     Arc::new(vec!["../huff-examples/erc20/contracts/ERC20.huff".to_string()]),
///     Some("./artifacts".to_string()),
///     None,
///     None,
///     None,
///     None,
///     false,
///     false
/// );
/// ```
#[derive(Debug, Clone)]
pub struct Compiler<'a, 'l> {
    /// The EVM version to compile for
    pub evm_version: &'l EVMVersion,
    /// The location of the files to compile
    pub sources: Arc<Vec<String>>,
    /// The output location
    pub output: Option<String>,
    /// Macro to use a main
    pub alternative_main: Option<String>,
    /// Constructor macro to use
    pub alternative_constructor: Option<String>,
    /// Constructor Input Arguments
    pub construct_args: Option<Vec<String>>,
    /// Constant Overrides
    pub constant_overrides: Option<BTreeMap<&'a str, Literal>>,
    /// Whether to optimize compilation or not.
    pub optimize: bool,
    /// Generate and log bytecode
    pub bytecode: bool,
    /// Whether to check cached artifacts
    pub cached: bool,
    /// The implementation of a FileReader
    pub file_provider: Arc<dyn FileProvider<'a>>,
}

impl<'a, 'l> Compiler<'a, 'l> {
    /// Public associated function to instantiate a new compiler.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        evm_version: &'l EVMVersion,
        sources: Arc<Vec<String>>,
        output: Option<String>,
        alternative_main: Option<String>,
        alternative_constructor: Option<String>,
        construct_args: Option<Vec<String>>,
        constant_overrides: Option<BTreeMap<&'a str, Literal>>,
        verbose: bool,
        cached: bool,
    ) -> Self {
        if cfg!(feature = "verbose") || verbose {
            Compiler::init_tracing_subscriber(Some(vec![tracing::Level::INFO.into()]));
        }
        Self {
            evm_version,
            sources,
            output,
            alternative_main,
            alternative_constructor,
            construct_args,
            constant_overrides,
            optimize: false,
            bytecode: false,
            cached,
            file_provider: Arc::new(FileSystemFileProvider {}),
        }
    }

    /// Creates a new instance of a compiler with an in-memory FileReader from the supplied sources
    /// map.
    #[allow(clippy::too_many_arguments)]
    pub fn new_in_memory(
        evm_version: &'l EVMVersion,
        sources: Arc<Vec<String>>,
        file_sources: HashMap<String, String>,
        alternative_main: Option<String>,
        alternative_constructor: Option<String>,
        construct_args: Option<Vec<String>>,
        constant_overrides: Option<BTreeMap<&'a str, Literal>>,
        verbose: bool,
    ) -> Self {
        if cfg!(feature = "verbose") || verbose {
            Compiler::init_tracing_subscriber(Some(vec![tracing::Level::INFO.into()]));
        }
        Self {
            evm_version,
            sources,
            output: None,
            alternative_main,
            alternative_constructor,
            construct_args,
            constant_overrides,
            optimize: false,
            bytecode: false,
            cached: false,
            file_provider: Arc::new(InMemoryFileProvider::new(file_sources)),
        }
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
            println!("Failed to initialize tracing!\nError: {e:?}")
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
    pub fn execute(&self) -> Result<Vec<Arc<Artifact>>, Arc<CompilerError>> {
        // Grab the input files
        let file_paths: Vec<PathBuf> = self.file_provider.transform_paths(&self.sources)?;

        // Parallel file fetching
        let files: Vec<Result<Arc<FileSource>, CompilerError>> =
            Self::fetch_sources(file_paths, self.file_provider.clone());

        // Unwrap errors
        let mut errors =
            files.iter().filter_map(|rfs| rfs.as_ref().err()).collect::<Vec<&CompilerError>>();
        if !errors.is_empty() {
            let error = errors.remove(0);
            return Err(Arc::new(error.clone()))
        }

        // Unpack files into their file sources
        let files = files
            .iter()
            .filter_map(|fs| fs.as_ref().map(Arc::clone).ok())
            .collect::<Vec<Arc<FileSource>>>();

        // Grab the output
        let output = self.get_outputs();

        // TODO: Parallelize Artifact Caching
        // rayon::spawn({
        //     let cloned_files: Vec<Arc<FileSource>> = files.iter().map(Arc::clone).collect();
        //     let ol: OutputLocation = output.clone();
        //     || cache::get_cached_artifacts(cloned_files, ol)
        // });

        let mut artifacts: Vec<Arc<Artifact>> = vec![];

        // Get our constructor arguments as a hex encoded string to compare to the cache
        let inputs = self.get_constructor_args();
        let encoded_inputs = Codegen::encode_constructor_args(inputs);
        let encoded: Vec<Vec<u8>> =
            encoded_inputs.iter().map(|tok| ethers_core::abi::encode(&[tok.clone()])).collect();
        let constructor_args = encoded.iter().map(|tok| hex::encode(tok.as_slice())).collect();

        // Get Cached or Generate Artifacts
        tracing::debug!(target: "core", "Output directory: {}", output.0);
        match cache::get_cached_artifacts(&files, &output, constructor_args) {
            Some(arts) => artifacts = arts,
            None => {
                tracing::debug!(target: "core", "FINISHED RECURSING DEPENDENCIES!");
                // Parallel Dependency Resolution
                let recursed_file_sources: Vec<Result<Arc<FileSource>, Arc<CompilerError>>> = files
                    .into_par_iter()
                    .map(|v| {
                        Self::recurse_deps(v, &Remapper::new("./"), self.file_provider.clone())
                    })
                    .collect();

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
                    .into_iter()
                    .filter_map(|fs| fs.ok())
                    .collect::<Vec<Arc<FileSource>>>();
                tracing::info!(target: "core", "COMPILER RECURSED {} FILE DEPENDENCIES", files.len());

                // Parallel Compilation
                let potential_artifacts: Vec<Result<Artifact, CompilerError>> =
                    files.into_par_iter().map(|f| self.gen_artifact(f)).collect();

                let mut gen_errors: Vec<CompilerError> = vec![];

                // Output errors + return OR print # of successfully compiled files
                for r in potential_artifacts {
                    match r {
                        Ok(a) => artifacts.push(Arc::new(a)),
                        Err(ce) => gen_errors.push(ce),
                    }
                }

                if !gen_errors.is_empty() {
                    tracing::error!(target: "core", "{} FILES FAILED TO COMPILE", gen_errors.len());
                    return Err(Arc::new(CompilerError::FailedCompiles(gen_errors)))
                }

                // Export
                Compiler::export_artifacts(&artifacts, &output);
            }
        }

        Ok(artifacts)
    }

    /// Grab the ASTs for all file sources.
    ///
    /// ### Steps
    ///
    /// 1. Transform inputs into File Paths with [transform_paths](Compiler::transform_paths).
    /// 2. Fetch file sources in parallel with [fetch_sources](Compiler::fetch_sources).
    /// 3. Recurse file dependencies in parallel with [recurse_deps](Compiler::recurse_deps).
    /// 4. For each top-level file, parse its contents and return a vec of [Contract](Contract)
    ///    ASTs.
    pub fn grab_contracts(&self) -> Result<Vec<Contract>, Arc<CompilerError>> {
        // Grab the input files
        let file_paths: Vec<PathBuf> = self.file_provider.transform_paths(&self.sources)?;

        // Parallel file fetching
        let files: Vec<Result<Arc<FileSource>, CompilerError>> =
            Self::fetch_sources(file_paths, self.file_provider.clone());

        // Unwrap errors
        let mut errors =
            files.iter().filter_map(|rfs| rfs.as_ref().err()).collect::<Vec<&CompilerError>>();
        if !errors.is_empty() {
            let error = errors.remove(0);
            return Err(Arc::new(error.clone()))
        }

        // Unpack files into their file sources
        let files = files
            .iter()
            .filter_map(|fs| fs.as_ref().map(Arc::clone).ok())
            .collect::<Vec<Arc<FileSource>>>();

        let recursed_file_sources: Vec<Result<Arc<FileSource>, Arc<CompilerError>>> = files
            .into_par_iter()
            .map(|f| {
                Self::recurse_deps(
                    f,
                    &huff_utils::files::Remapper::new("./"),
                    self.file_provider.clone(),
                )
            })
            .collect();

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
            .into_iter()
            .filter_map(|fs| fs.ok())
            .collect::<Vec<Arc<FileSource>>>();
        tracing::info!(target: "core", "COMPILER RECURSED {} FILE DEPENDENCIES", files.len());

        // Parse file sources and collect ASTs in parallel
        files
            .into_par_iter()
            .map(|file| {
                // Fully Flatten a file into a source string containing source code of file and all
                // its dependencies
                let flattened = FileSource::fully_flatten(Arc::clone(&file));
                tracing::info!(target: "core", "FLATTENED SOURCE FILE \"{}\"", file.path);
                let full_source = FullFileSource {
                    source: &flattened.0,
                    file: Some(Arc::clone(&file)),
                    spans: flattened.1,
                };
                tracing::debug!(target: "core", "GOT FULL SOURCE FOR PATH: {:?}", file.path);

                // Perform Lexical Analysis
                // Create a new lexer from the FileSource, flattening dependencies
                let lexer = Lexer::new(full_source.source);

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
                contract.add_override_constants(&self.constant_overrides);
                tracing::info!(target: "core", "PARSED CONTRACT [{}]", file.path);
                Ok(contract)
            })
            .collect::<Result<Vec<Contract>, Arc<CompilerError>>>()
    }

    /// Artifact Generation
    ///
    /// Compiles a FileSource into an Artifact.
    pub fn gen_artifact(&self, file: Arc<FileSource>) -> Result<Artifact, CompilerError> {
        // Fully Flatten a file into a source string containing source code of file and all
        // its dependencies
        let flattened = FileSource::fully_flatten(Arc::clone(&file));
        tracing::info!(target: "core", "FLATTENED SOURCE FILE \"{}\"", file.path);
        let full_source = FullFileSource {
            source: &flattened.0,
            file: Some(Arc::clone(&file)),
            spans: flattened.1,
        };
        tracing::debug!(target: "core", "GOT FULL SOURCE FOR PATH: {:?}", file.path);

        // Perform Lexical Analysis
        // Create a new lexer from the FileSource, flattening dependencies
        let lexer = Lexer::new(full_source.source);

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
        contract.add_override_constants(&self.constant_overrides);
        tracing::info!(target: "core", "PARSED CONTRACT [{}]", file.path);

        // Primary Bytecode Generation
        let mut cg = Codegen::new();
        let main_bytecode = match Codegen::generate_main_bytecode(
            self.evm_version,
            &contract,
            self.alternative_main.clone(),
        ) {
            Ok(mb) => mb,
            Err(mut e) => {
                tracing::error!(target: "core", "FAILED TO GENERATE MAIN BYTECODE FOR CONTRACT");
                // Add File Source to Span
                e.span = AstSpan(
                    e.span
                        .0
                        .into_iter()
                        .map(|mut s| {
                            s.file = Some(Arc::clone(&file));
                            s
                        })
                        .collect::<Vec<Span>>(),
                );
                tracing::error!(target: "core", "Roll Failed with CodegenError: {:?}", e.kind);
                return Err(CompilerError::CodegenError(e))
            }
        };
        tracing::info!(target: "core", "MAIN BYTECODE GENERATED [{}]", main_bytecode);

        // Generate Constructor Bytecode
        let inputs = self.get_constructor_args();
        let (constructor_bytecode, has_custom_bootstrap) =
            match Codegen::generate_constructor_bytecode(
                self.evm_version,
                &contract,
                self.alternative_constructor.clone(),
            ) {
                Ok(mb) => mb,
                Err(mut e) => {
                    // Return any errors except if the inputs is empty and the constructor
                    // definition is missing
                    if e.kind != CodegenErrorKind::MissingMacroDefinition("CONSTRUCTOR".to_string()) ||
                        !inputs.is_empty()
                    {
                        // Add File Source to Span
                        let mut errs = e
                            .span
                            .0
                            .into_iter()
                            .map(|mut s| {
                                s.file = Some(Arc::clone(&file));
                                s
                            })
                            .collect::<Vec<Span>>();
                        errs.dedup();
                        e.span = AstSpan(errs);
                        tracing::error!(target: "codegen", "Constructor inputs provided, but contract missing \"CONSTRUCTOR\" macro!");
                        return Err(CompilerError::CodegenError(e))
                    }

                    // If the kind is a missing constructor we can ignore it
                    tracing::warn!(target: "codegen", "Contract has no \"CONSTRUCTOR\" macro definition!");
                    (String::default(), false)
                }
            };
        tracing::info!(target: "core", "CONSTRUCTOR BYTECODE GENERATED [{}]", constructor_bytecode);

        // Encode Constructor Arguments
        let encoded_inputs = Codegen::encode_constructor_args(inputs);
        tracing::info!(target: "core", "ENCODED {} INPUTS", encoded_inputs.len());

        // Generate Artifact with ABI
        let churn_res = cg.churn(
            file,
            encoded_inputs,
            &main_bytecode,
            &constructor_bytecode,
            has_custom_bootstrap,
        );
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
    pub fn fetch_sources(
        paths: Vec<PathBuf>,
        reader: Arc<dyn FileProvider<'a>>,
    ) -> Vec<Result<Arc<FileSource>, CompilerError>> {
        paths.into_par_iter().map(|pb| reader.read_file(pb)).collect()
    }

    /// Recurses file dependencies
    pub fn recurse_deps(
        fs: Arc<FileSource>,
        remapper: &Remapper,
        reader: Arc<dyn FileProvider<'a>>,
    ) -> Result<Arc<FileSource>, Arc<CompilerError>> {
        tracing::debug!(target: "core", "RECURSING DEPENDENCIES FOR {}", fs.path);
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
            new_fs.access = Some(time::get_current_time());
            new_source
        };
        let imports: Vec<String> = Lexer::lex_imports(&file_source);
        new_fs.source = Some(file_source);
        if !imports.is_empty() {
            tracing::info!(target: "core", "IMPORT LEXICAL ANALYSIS COMPLETE ON {:?}", imports);
        }

        let localized_imports: Vec<String> = imports
            .into_iter()
            .map(|mut import| {
                // Check for foundry toml remappings
                match remapper.remap(&import) {
                    Some(remapped) => {
                        tracing::debug!(target: "core", "REMAPPED IMPORT PATH \"{}\"", import);
                        import = remapped;
                    }
                    None => {
                        import = FileSource::localize_file(&fs.path, &import)
                            .unwrap_or_default()
                            .replacen("contracts/contracts", "contracts", 1);
                    }
                }
                import
            })
            .collect();
        if !localized_imports.is_empty() {
            tracing::info!(target: "core", "LOCALIZED IMPORTS {:?}", localized_imports);
        }
        let import_bufs: Vec<PathBuf> = reader.transform_paths(&localized_imports)?;
        let potentials: Result<Vec<Arc<FileSource>>, CompilerError> =
            Self::fetch_sources(import_bufs, reader.clone()).into_iter().collect();
        let mut file_sources = match potentials {
            Ok(p) => p,
            Err(e) => return Err(Arc::new(e)),
        };
        if !file_sources.is_empty() {
            tracing::info!(target: "core", "FETCHED {} FILE SOURCES", file_sources.len());
        }

        // Now that we have all the file sources, we have to recurse and get their source
        file_sources = file_sources
            .into_par_iter()
            .map(|inner_fs| match Self::recurse_deps(Arc::clone(&inner_fs), remapper, reader.clone()) {
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
        // Exit if empty output location
        if output.0.is_empty() {
            tracing::warn!(target: "core", "Exiting artifact export with empty output location!");
            return
        }

        // Clean the Output Directory
        tracing::warn!(target: "core", "REMOVING DIRECTORY: \"{}\"", output.0);
        if !output.0.is_empty() && fs::remove_dir_all(&output.0).is_ok() {
            tracing::info!(target: "core", "OUTPUT DIRECTORY DELETED!");
        }

        // Is the output a directory or a file?
        let is_file = std::path::PathBuf::from(&output.0).extension().is_some();

        // Export the artifacts with parallelized io
        artifacts.into_par_iter().for_each(|a| {
            // If it's a file type, we just export to `output.0`
            let json_out = match is_file {
                true => output.0.clone(),
                false => format!(
                    "{}/{}.json",
                    output.0,
                    a.file.path.to_uppercase().replacen("./", "", 1)
                ),
            };

            if let Err(e) = a.export(&json_out) {
                tracing::error!(target: "core", "ARTIFACT EXPORT FAILED!\nError: {:?}", e);
            }
            tracing::info!(target: "core", "EXPORTED ARTIFACT TO \"{}\"", json_out);
        });
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
