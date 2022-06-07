use clap::Parser;

mod huff;
use huff::*;

fn main() {
    // Parse the command line arguments
    let cli = Huff::parse();

    // Run the Compiler
    let _compile_res = cli.execute();
}
