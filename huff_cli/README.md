# Huff CLI

The `huff` CLI is written using [clap's](https://docs.rs/clap) [derive feature](https://github.com/clap-rs/clap/blob/master/examples/derive_ref/README.md).


## huff

```
huff 0.1.0
Efficient Huff compiler written in rust

USAGE:
    huff [OPTIONS] [PATH]

ARGS:
    <PATH>

OPTIONS:
    -b, --bytecode                        Generate and log bytecode
    -d, --output-directory <OUTPUTDIR>    The output directory [default: ./artifacts]
    -h, --help                            Print help information
    -i, --inputs <INPUTS>...              The input constructor arguments
    -o, --output <OUTPUT>                 The output file path
    -p, --print                           Prints out to the terminal
    -s, --source-path <SOURCE>            The contracts source path [default: ./src]
    -v, --verbose                         Verbose output
    -V, --version                         Print version information
    -z, --optimize                        Optimize compilation
```

_NOTE: To generate the above output, run: `huff --help`_


## Usage

To run `huff` from the command line, you can use the following command:

```bash
cargo run --bin huff
```

To pass arguments into the `huff` binary, simply pass them in after a `--` flag. For example, to get the `huff` version (a `-V` flag), you can run:

```bash
cargo run --bin huff -- -V
```

**Example: Using `huff-examples`**

The [huff-examples](https://github.com/huff-language/huff-examples) github repository is added as a submodule to this repo for testing.

To run `huff` against one of the examples, the path may simply be passed to `huff`.

For example, to compile huff-example's [ERC20.huff](../huff-examples/erc20/contracts/ERC20.huff) contract, run:

```bash
cargo run --bin huff -- --bytecode ./huff-examples/erc20/contracts/ERC20.huff
```

_NOTE: The `--bytecode` flag will output the full deploy bytecode._

`huff` also supports tracing using the [`tracing`](https://docs.rs/tracing/0.1.29/tracing/) crate. To produce a verbose output using tracing, append the `--verbose` or `-v` flag like so:

```bash
cargo run --bin huff -- --verbose --bytecode ./huff-examples/erc20/contracts/ERC20.huff
```

**By default**, `huff` will export json build artifacts to a `./artifacts` directory. This can be overidden using the `--output-directory` flag or shorthand `-d` flag and specifying a string following. For example:

```bash
cargo run --bin huff -- -d ./output ./huff-examples/erc20/contracts/ERC20.huff
```

_NOTE: The huff cli will gracefully remove double and single quotes, so the following will also compile:_

```bash
cargo run --bin huff -- -d "./output" './huff-examples/erc20/contracts/ERC20.huff'
```

