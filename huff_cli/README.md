# Huff CLI

The `huffc` CLI is written using [clap's](https://docs.rs/clap) [derive feature](https://docs.rs/clap/latest/clap/_derive/index.html).

## huffc

```
huffc 0.3.2
Huff Language Compiler built in Pure Rust.

USAGE:
    huffc [OPTIONS] [PATH] [SUBCOMMAND]

ARGS:
    <PATH>    The contract(s) to compile

OPTIONS:
    -a, --artifacts
            Whether to generate artifacts or not

    -b, --bytecode
            Generate and log bytecode

    -c, --constants <CONSTANTS>...
            Override / set constants for the compilation environment

    -d, --output-directory <OUTPUTDIR>
            The output directory [default: ./artifacts]

    -e, --evm-version <EVM_VERSION>
            Set the EVM version

    -g, --interface [<INTERFACE>...]
            Generate solidity interface for a Huff artifact

    -h, --help
            Print help information

    -i, --inputs <INPUTS>...
            The input constructor arguments

    -l, --label-indices
            Prints out the jump label PC indices for the specified contract

    -m, --alt-main <ALTERNATIVE_MAIN>
            Compile a specific macro

    -n, --interactive
            Interactively input the constructor args

    -o, --output <OUTPUT>
            The output file path

    -p, --print
            Prints out to the terminal

    -r, --bin-runtime
            Generate and log runtime bytecode

    -s, --source-path <SOURCE>
            The contracts source path [default: ./contracts]

    -t, --alt-constructor <ALTERNATIVE_CONSTRUCTOR>
            Compile a specific constructor macro

    -v, --verbose
            Verbose output

    -V, --version
            Print version information

    -z, --optimize
            Optimize compilation [WIP]

```

_NOTE: To generate the above output, run: `huffc --help`_

## Usage

To run `huffc` from the command line, you can simply run:

```bash
huffc --help
```

By default, huffc will attempt to compile all contracts in the `contracts` directory. If there is no `contracts` directory present, the following will spit out an error like so:

```bash,color=red
~ huffc

Error: Invalid File Directory ./contracts

```

#### Examples using [`huff-examples`](https://github.com/huff-language/huff-examples)

The [huff-examples](https://github.com/huff-language/huff-examples) github repository is added as a submodule to this repo for testing.

To run `huffc` against one of the examples, the path may simply be passed to `huffc`.

For example, to compile huff-example's [ERC20.huff](../huff-examples/erc20/contracts/ERC20.huff) contract, run:

```bash
huffc --bytecode ./huff-examples/erc20/contracts/ERC20.huff
```

_NOTE: The `--bytecode` flag will output the full deploy bytecode._

`huffc` also supports tracing using the [`tracing`](https://docs.rs/tracing/0.1.29/tracing/) crate. To produce a verbose output using tracing, append the `--verbose` or `-v` flag like so:

```bash
huffc --verbose --bytecode ./huff-examples/erc20/contracts/ERC20.huff
```

#### Specifying Artifact Outputs

**By default**, `huffc` will export json build artifacts to a `./artifacts` directory. This can be overidden using the `--output-directory` flag or shorthand `-d` flag and specifying a string following. For example:

```bash
huffc -d ./output ./huff-examples/erc20/contracts/ERC20.huff
```

_NOTE: The huff cli will gracefully remove double and single quotes, so the following will also compile:_

```bash
huffc -d "./output" './huff-examples/erc20/contracts/ERC20.huff'
```

If a specific contract is specified for compiling (ie not a directory), a single `json` file may be specified as an output location for the contract artifact like so:

```bash
huffc -o ./artifact.json ./huff-examples/erc20/contracts/ERC20.huff
```

**NOTE**: The following will _not_ compile since multiple artifacts cannot be output to the same artifact json file.

```bash
huffc -o ./artifact.json ./contracts/
```

#### Entering Constructor Arguments

`huffc` supports passing in constructor arguments to the contract. This is done by passing in the `--interactive` (shorthand: `-n`) flag or passing the `--inputs` (shorthand: `-i`) flag.

and passing in the arguments as a comma separated list.

For example, to compile a contract (let's call it `example.huff`) with the following constructor definition:

```huff
#define macro CONSTRUCTOR(uint256, address) = takes(0) returns (0) {
    0x04 calldataload
    0x00 sstore
    0x24 calldataload
    0x01 sstore
}
```

You can enter the arguments `(100, 0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef)` interactively by passing in the `-n` or `--interactive` flag like so:

```bash
$ huffc -b -n ./contracts/example.huff
[INTERACTIVE] Constructor Arguments for Contract: "./contracts/example.huff"
[INTERACTIVE] Enter a uint256 for constructor param: 100
[INTERACTIVE] Enter a address for constructor param: 0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef

335f.....f30000000000000000000000000000000000000000000000000000000000000064000000000000000000000000deadbeefdeadbeefdeadbeefdeadbeefdeadbeef
```

Alternatively, you can enter the arguments as a comma separated list by using the `-i` or `--inputs` flag like so:

```bash
$ huffc -b -i 100, 0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef ./contracts/example.huff
335f0.....f30000000000000000000000000000000000000000000000000000000000000064000000000000000000000000deadbeefdeadbeefdeadbeefdeadbeefdeadbeef
```

#### Other Options

- `-v` or `--verbose`: Outputs detailed logs to the terminal using the [tracing](https://crates.io/crates/tracing) crate.
- `-V` or `--version`: Prints the version of `huffc`.
- `-z` or `--optimize`: Optimizes the contract compilation - a work in progress.
- `-g` or `--interface`: Generates a solidity interface for the contract.

## Building huffc from source

To run `huffc` from the command line, you can use the following command:

```bash
cargo run --bin huffc
```

To pass arguments into the `huffc` binary, simply pass them in after a `--` flag. For example, to get the `huffc` version (a `-V` flag), you can run:

```bash
cargo run --bin huffc -- -V
```

All commands specified in [Usage](#usage) are also available from source by passing them in after the `--` flag.
