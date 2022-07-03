#![doc = include_str!("../README.md")]
#![allow(dead_code)]
#![allow(clippy::enum_variant_names)]
#![warn(missing_docs)]
#![warn(unused_extern_crates)]
#![forbid(unsafe_code)]
#![forbid(where_clauses_object_safety)]
#![allow(deprecated)]

use clap::Parser as ClapParser;
use ethers_core::utils::hex;
use huff_codegen::Codegen;
use huff_core::Compiler;
use huff_utils::prelude::{
    unpack_files, AstSpan, CodegenError, CodegenErrorKind, CompilerError, FileSource,
    OutputLocation, Span,
};
use isatty::stdout_isatty;
use spinners::{Spinner, Spinners};
use std::{io::Write, path::Path, sync::Arc};
use yansi::Paint;

/// The Huff CLI Args
#[derive(ClapParser, Debug, Clone)]
#[clap(name = "huffc", version, about, long_about = None)]
struct Huff {
    /// The main path
    pub path: Option<String>,

    /// The contracts source path.
    #[clap(short = 's', long = "source-path", default_value = "./contracts")]
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

    /// Interactively input the constructor args
    #[clap(short = 'n', long = "interactive")]
    interactive: bool,

    /// Whether to generate artifacts or not
    #[clap(short = 'a', long = "artifacts")]
    artifacts: bool,

    /// Optimize compilation [WIP]
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

/// Helper function to read an stdin input
pub(crate) fn get_input(prompt: &str) -> String {
    // let mut sp = Spinner::new(Spinners::Line, format!("{}{}",
    // Paint::blue("[INTERACTIVE]".to_string()), prompt));
    print!("{} {} ", Paint::blue("[INTERACTIVE]".to_string()), prompt);
    let mut input = String::new();
    let _ = std::io::stdout().flush();
    let _ = std::io::stdin().read_line(&mut input);
    // sp.stop();
    input.trim().to_string()
}

fn main() {
    // Parse the command line arguments
    let mut cli = Huff::parse();

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

    if cli.interactive {
        // Don't accept configured inputs
        cli.inputs = None;
        // Don't export artifacts are compile
        // Have to first generate artifacts, prompt user for args,
        // and finally save artifacts with the new constructor args.
        cli.artifacts = false;
    }

    let output = match (&cli.output, cli.artifacts) {
        (Some(o), true) => Some(o.clone()),
        (None, true) => Some(cli.outputdir.clone()),
        _ => None,
    };

    let compiler: Compiler = Compiler {
        sources: Arc::clone(&sources),
        output,
        construct_args: cli.inputs,
        optimize: cli.optimize,
        bytecode: cli.bytecode,
    };

    // Create compiling spinner
    tracing::debug!(target: "cli", "[⠔] COMPILING");
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
        Ok(mut artifacts) => {
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
                tracing::error!(target: "cli", "COMPILER ERRORED: {:?}", e);
                eprintln!("{}", Paint::red(format!("{}", e)));
                std::process::exit(1);
            }
            if cli.bytecode {
                if cli.interactive {
                    tracing::info!(target: "cli", "ENTERING INTERACTIVE MODE");
                    // let mut new_artifacts = vec![];
                    for artifact in &mut artifacts {
                        let mut appended_args = String::default();
                        match artifact.abi {
                            Some(ref abi) => match abi.constructor {
                                Some(ref args) => {
                                    println!(
                                        "{} Constructor Arguments for Contract: \"{}\"",
                                        Paint::blue("[INTERACTIVE]".to_string()),
                                        artifact.file.path
                                    );
                                    for input in &args.inputs {
                                        let arg_input = get_input(&format!(
                                            "Enter a {:?} for constructor param{}:",
                                            input.kind,
                                            (!input.name.is_empty())
                                                .then(|| format!(" \"{}\"", input.name))
                                                .unwrap_or_default()
                                        ));
                                        let encoded =
                                            Codegen::encode_constructor_args(vec![arg_input])
                                                .iter()
                                                .fold(String::default(), |acc, str| {
                                                    let inner: Vec<u8> =
                                                        ethers_core::abi::encode(&[str.clone()]);
                                                    let hex_args: String =
                                                        hex::encode(inner.as_slice());
                                                    format!("{}{}", acc, hex_args)
                                                });
                                        appended_args.push_str(&encoded);
                                    }
                                }
                                None => {
                                    tracing::warn!(target: "cli", "NO CONSTRUCTOR FOR ABI: {:?}", abi)
                                }
                            },
                            None => {
                                tracing::warn!(target: "cli", "NO ABI FOR ARTIFACT: {:?}", artifact)
                            }
                        }
                        match Arc::get_mut(artifact) {
                            Some(art) => {
                                art.bytecode = format!("{}{}", art.bytecode, appended_args);
                            }
                            None => {
                                tracing::warn!(target: "cli", "FAILED TO ACQUIRE MUTABLE REF TO ARTIFACT")
                            }
                        }
                    }
                    tracing::debug!(target: "cli", "Re-exporting artifacts...");
                    Compiler::export_artifacts(
                        &artifacts,
                        &OutputLocation(cli.output.unwrap_or_else(|| cli.outputdir.clone())),
                    );
                    tracing::info!(target: "cli", "RE-EXPORTED INTERACTIVE ARTIFACTS");
                }
                match sources.len() {
                    1 => print!("{}", artifacts[0].bytecode),
                    _ => artifacts
                        .iter()
                        .for_each(|a| println!("\"{}\" bytecode: {}", a.file.path, a.bytecode)),
                }
            }
        }
        Err(e) => {
            tracing::error!(target: "cli", "COMPILER ERRORED: {:?}", e);
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
