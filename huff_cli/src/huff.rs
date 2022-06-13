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
use huff_utils::prelude::unpack_files;
use std::path::Path;

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
        huff_core::init_tracing_subscriber(Some(vec![tracing::Level::DEBUG.into()]));
    }

    // Create compiler from the Huff Args
    let compiler: Compiler = Compiler {
        sources: cli.get_inputs().unwrap_or_default(),
        output: match &cli.output {
            Some(o) => Some(o.clone()),
            None => Some(cli.outputdir.clone()),
        },
        inputs: cli.inputs,
        optimize: cli.optimize,
        bytecode: cli.bytecode,
    };
    tracing::info!(target: "core", "COMPILER INCANTATION COMPLETE");
    tracing::info!(target: "core", "EXECUTING COMPILATION...");
    let compile_res = compiler.execute();
    if let Err(e) = compile_res {
        tracing::error!(target: "core", "COMPILER ERRORED: {:?}", e);
    }
}

impl Huff {
    /// Preprocesses input files for compiling
    pub fn get_inputs(&self) -> Option<Vec<String>> {
        match &self.path {
            Some(path) => {
                tracing::info!(target: "io", "FETCHING INPUT: {}", path);
                // If the file is huff, we can use it
                let ext = Path::new(&path).extension().unwrap_or_default();
                if ext.eq("huff") {
                    Some(vec![path.clone()])
                } else {
                    // Otherwise, override the source files and use all files in the provided dir
                    unpack_files(path).ok()
                }
            }
            None => {
                tracing::info!(target: "io", "FETCHING SOURCE FILES: {}", self.source);
                // If there's no path, unpack source files
                let source: String = self.source.clone();
                unpack_files(&source).ok()
            }
        }
    }
}
