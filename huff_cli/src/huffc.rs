#![doc = include_str!("../README.md")]
#![allow(dead_code)]
#![allow(clippy::enum_variant_names)]
#![warn(missing_docs)]
#![warn(unused_extern_crates)]
#![forbid(unsafe_code)]
#![forbid(where_clauses_object_safety)]
#![allow(deprecated)]

use clap::Parser as ClapParser;
use huff_core::Compiler;
use huff_utils::prelude::{
    unpack_files, AstSpan, CodegenError, CodegenErrorKind, CompilerError, FileSource, Span,
};
use isatty::stdout_isatty;
use spinners::{Spinner, Spinners};
use std::{path::Path, sync::Arc};
use yansi::Paint;

/// The Huff CLI Args
#[derive(ClapParser, Debug, Clone)]
#[clap(version, about, long_about = None)]
struct Huff {
    /// The main path
    pub path: Option<String>,

    /// The contracts source path.
    #[clap(short = 's', long = "source-path", default_value = "./src")]
    source: String,

    /// The output file path.
    #[clap(short = 'o', long = "output")]
    output: Option<String>,

    /// The output directory.
    #[clap(short = 'd', long = "output-directory", default_value = "./artifacts")]
    outputdir: String,

    /// The input constructor arguments
    #[clap(short = 'i', long = "inputs", multiple_values = true)]
    inputs: Option<Vec<String>>,

    /// Whether to generate artifacts or not
    #[clap(short = 'a', long = "artifacts")]
    artifacts: bool,

    /// Optimize compilation.
    #[clap(short = 'z', long = "optimize")]
    optimize: bool,

    /// Generate and log bytecode.
    #[clap(short = 'b', long = "bytecode")]
    bytecode: bool,

    /// Prints out to the terminal.
    #[clap(short = 'p', long = "print")]
    print: bool,

    /// Verbose output.
    #[clap(short = 'v', long = "verbose")]
    verbose: bool,
}

fn main() {
    // Parse the command line arguments
    let cli = Huff::parse();

    // Initiate Tracing if Verbose
    if cli.verbose {
        Compiler::init_tracing_subscriber(Some(vec![tracing::Level::DEBUG.into()]));
    }

    // Create compiler from the Huff Args
    let sources: Arc<Vec<String>> = match cli.get_inputs() {
        Ok(s) => Arc::new(s),
        Err(e) => {
            eprintln!("{}", Paint::red(format!("{}", e)));
            std::process::exit(1);
        }
    };
    let compiler: Compiler = Compiler {
        sources: Arc::clone(&sources),
        output: match (&cli.output, cli.artifacts) {
            (Some(o), true) => Some(o.clone()),
            (None, true) => Some(cli.outputdir.clone()),
            _ => None,
        },
        construct_args: cli.inputs,
        optimize: cli.optimize,
        bytecode: cli.bytecode,
    };

    // Create compiling spinner
    tracing::debug!(target: "core", "[⠔] COMPILING");
    let mut sp: Option<Spinner> = None;
    // If stdout is a TTY, create a spinner
    if stdout_isatty() {
        sp = Some(Spinner::new(Spinners::Dots, "Compiling...".into()));
    }

    let compile_res = compiler.execute();
    // Stop spinner animation if it exists
    if let Some(mut sp) = sp {
        sp.stop();
        println!(" ");
    }
    match compile_res {
        Ok(artifacts) => {
            if artifacts.is_empty() {
                let e = CompilerError::CodegenError(CodegenError {
                    kind: CodegenErrorKind::AbiGenerationFailure,
                    span: AstSpan(
                        sources
                            .iter()
                            .map(|s| Span {
                                start: 0,
                                end: 0,
                                file: Some(Arc::new(FileSource {
                                    id: uuid::Uuid::new_v4(),
                                    path: s.clone(),
                                    source: None,
                                    access: None,
                                    dependencies: None,
                                })),
                            })
                            .collect::<Vec<Span>>(),
                    ),
                    token: None,
                });
                tracing::error!(target: "core", "COMPILER ERRORED: {:?}", e);
                eprintln!("{}", Paint::red(format!("{}", e)));
                std::process::exit(1);
            }
            if cli.bytecode {
                match sources.len() {
                    1 => println!("{}", artifacts[0].bytecode),
                    _ => artifacts
                        .iter()
                        .for_each(|a| println!("\"{}\" bytecode: {}", a.file.path, a.bytecode)),
                }
            }
        }
        Err(e) => {
            tracing::error!(target: "core", "COMPILER ERRORED: {:?}", e);
            eprintln!("{}", Paint::red(format!("{}", e)));
            std::process::exit(1);
        }
    }
}

impl Huff {
    /// Preprocesses input files for compiling
    pub fn get_inputs(&self) -> Result<Vec<String>, CompilerError> {
        match &self.path {
            Some(path) => {
                tracing::debug!(target: "io", "FETCHING INPUT: {}", path);
                // If the file is huff, we can use it
                let ext = Path::new(&path).extension().unwrap_or_default();
                if ext.eq("huff") {
                    Ok(vec![path.clone()])
                } else {
                    // Otherwise, override the source files and use all files in the provided dir
                    unpack_files(path).map_err(CompilerError::FileUnpackError)
                }
            }
            None => {
                tracing::debug!(target: "io", "FETCHING SOURCE FILES: {}", self.source);
                // If there's no path, unpack source files
                unpack_files(&self.source).map_err(CompilerError::FileUnpackError)
            }
        }
    }
}
