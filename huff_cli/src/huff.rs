//! Huff
//!
//! The Huff Compiler CLI.

#![allow(dead_code)]
#![allow(clippy::enum_variant_names)]
#![warn(missing_docs)]
#![warn(unused_extern_crates)]
#![forbid(unsafe_code)]
#![forbid(where_clauses_object_safety)]

use std::path::Path;

use clap::Parser as ClapParser;
use huff_core::Compiler;
use huff_utils::prelude::unpack_files;

fn main() {
    // Parse the command line arguments
    let cli = Huff::parse();

    // Create compiler from the Huff Args
    let compiler: Compiler = Compiler {
        sources: cli.get_inputs().unwrap_or_default(),
        output: match &cli.output {
            Some(o) => Some(o.clone()),
            None => Some(cli.outputdir.clone()),
        },
        optimize: cli.optimize,
        bytecode: cli.bytecode,
    };
    let compile_res = compiler.execute();
    if compile_res.is_err() {
        tracing::error!("Compiling Errored!");
    }
}

/// The Huff CLI Args
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

impl Huff {
    /// Preprocesses input files for compiling
    pub fn get_inputs(&self) -> Option<Vec<String>> {
        match &self.path {
            Some(path) => {
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
                // If there's no path, unpack source files
                let source: String = self.source.clone();
                unpack_files(&source).ok()
            }
        }
    }
}
