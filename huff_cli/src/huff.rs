//! Huff
//!
//! The Huff Compiler CLI.

#![allow(dead_code)]
#![allow(clippy::enum_variant_names)]
#![warn(missing_docs)]
#![warn(unused_extern_crates)]
#![forbid(unsafe_code)]
#![forbid(where_clauses_object_safety)]

use clap::Parser as ClapParser;
use huff_core::Compiler;

fn main() {
    // Parse the command line arguments
    let cli = Huff::parse();

    // Create compiler from the Huff Args
    let compiler: Compiler = cli.into();
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
