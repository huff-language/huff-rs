use clap::Parser;
// use lexer::Lexer;

/// Efficiently compile Huff code.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    path: Option<String>,

    /// The source path to the contracts (default: "./src").
    #[clap(short = 's', long = "source-path", default_value = "./src")]
    source: String,

    /// The output file path.
    #[clap(short = 'o', long = "output")]
    output: Option<String>,

    /// The output directory (default: "./artifacts").
    #[clap(short = 'd', long = "output-directory")]
    outputdir: Option<String>,

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

fn main() {
    // Parse the command line arguments
    let cli = Cli::parse();

    // Unpack source
    let source: String = cli.source;

    println!("Source directory: {:?}", source);

    // TODO: this can be a file ?

    // Parse source files
    let source_files: Vec<String> = std::fs::read_dir(source)
        .unwrap()
        .map(|x| x.unwrap().path().to_str().unwrap().to_string())
        .collect();

    println!("Source files: {:?}", source_files);


    // Perform Lexical Analysis
    // let lexer: Lexer = Lexer::new();
    // TODO: print compiled bytecode if flagged
    // TODO: print output to terminal if flagged


    // TODO: Unpack output (if only one file or contract specified)
    // TODO: Unpack output directory


}