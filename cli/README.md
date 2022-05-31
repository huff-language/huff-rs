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
    rhuff <filename> [OPTIONS]

OPTIONS:
    -h, --help                Print help information
    -V, --version             Print version information
    -s, --source-path         The source path to the contracts (default: "./").
    -o, --output              The output file path.
    -d, --output-directory    The output directory (default: "./").
    -z, --optimize            Optimize compilation.
    -b, --bytecode            Generate and log bytecode (default: false).
    -p, --print               Print the output to the terminal.
```
