# rhuff CLIs

rhuff CLIs are written using [clap's](https://docs.rs/clap) [derive feature](https://github.com/clap-rs/clap/blob/master/examples/derive_ref/README.md).

Debug logs are printed with
[`tracing`](https://docs.rs/tracing/0.1.29/tracing/). You can configure the
verbosity level via the
[`RUST_LOG`](https://docs.rs/tracing-subscriber/0.3.2/tracing_subscriber/fmt/index.html#filtering-events-with-environment-variables)
environment variable, on a per package level,
e.g.:`RUST_LOG=forge=trace,evm_adapters=trace forge test`

## rhuff

```
rhuff 0.1.0
Efficiently compile Huff code.

USAGE:
    rhuff [OPTIONS] [PATH]

ARGS:
    <PATH>

OPTIONS:
    -b, --bytecode                        Generate and log bytecode (default: false).
    -d, --output-directory <OUTPUTDIR>    The output directory (default: "./").
    -h, --help                            Print help information
    -o, --output <OUTPUT>                 The output file path.
    -p, --print                           Print the output to the terminal.
    -s, --source-path <SOURCE>            The source path to the contracts (default: "./").
    -V, --version                         Print version information
    -z, --optimize                        Optimize compilation.
```



### Developing

To run `rhuff` from the command line, you can use the following command:

```bash
cargo run --bin rhuff
```

To pass arguments into the `rhuff` binary, simply pass them in after a `--` flag. For example, to get the `rhuff` version (a `-V` flag), you can run:

```bash
cargo run --bin rhuff -- -V
```

