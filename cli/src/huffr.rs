use std::path::Path;

use clap::Parser;
use codegen::*;
use utils::io::*;
// use lexer::Lexer;

/// Efficient Huff compiler.
#[derive(Parser, Debug, Clone)]
#[clap(version, about, long_about = None)]
struct Huffr {
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

// Parse files from an huffr instance
// TODO: We can probably turn this into a <BUILD> instance where we generate a list of all build
// files TODO:    with dependencies including their raw sources and perform compilation on that
// <BUILD> instance
impl From<Huffr> for Vec<String> {
    fn from(huffr: Huffr) -> Self {
        match huffr.path {
            Some(path) => {
                // If the file is huff, we can use it
                let ext = Path::new(&path).extension().unwrap_or_default();
                if ext.eq("huff") {
                    vec![path]
                } else {
                    // Otherwise, override the source files and use all files in the provided dir
                    unpack_files(&path).unwrap_or_default()
                }
            }
            None => {
                // If there's no path, unpack source files
                let source: String = huffr.source;
                unpack_files(&source).unwrap_or_default()
            }
        }
    }
}

/// An aliased output location to derive from the cli arguments.
#[derive(Debug, Clone)]
pub struct OutputLocation(pub(crate) String);

impl From<Huffr> for OutputLocation {
    fn from(huffr: Huffr) -> Self {
        match huffr.output {
            Some(o) => OutputLocation(o),
            None => OutputLocation(huffr.outputdir),
        }
    }
}

fn main() {
    // Parse the command line arguments
    let cli = Huffr::parse();

    // Gracefully derive file compilation
    let files: Vec<String> = cli.clone().into();
    println!("Compiling files: {:?}", files);

    // Perform Lexical Analysis
    // let lexer: Lexer = Lexer::new();
    // TODO: print compiled bytecode if flagged
    // TODO: print output to terminal if flagged

    // TODO: Unpack output (if only one file or contract specified)
    // TODO: Unpack output directory

    // Mock AST generated here
    let ast: Ast = Ast::new();
    println!("Created mock AST: {:?}", ast);

    // Run code generation
    let cg = Codegen::new(true);
    println!("Created a new codegen instance");

    let write_res = cg.write(&ast);
    println!("Codegen writing result: {:?}", write_res);

    // Gracefully derive the output from the cli
    let output: OutputLocation = cli.into();
    println!("Cli got output location: {:?}", output);

    let export_res = cg.export(&ast, &output.0, "INPUT");
    println!("Codegen export result: {:?}", export_res);
}
