<img align="right" width="150" height="150" top="100" src="./assets/huff.png">

# huff-rs • [![tests](https://github.com/huff-language/huff-rs/actions/workflows/tests.yaml/badge.svg)](https://github.com/huff-language/huff-rs/actions/workflows/tests.yaml) [![lints](https://github.com/huff-language/huff-rs/actions/workflows/lints.yaml/badge.svg)](https://github.com/huff-language/huff-rs/actions/workflows/lints.yaml) ![GitHub](https://img.shields.io/github/license/huff-language/huff-rs)  ![Crates.io](https://img.shields.io/crates/v/huff-rs)

> `huff-rs` is a [huff](https://github.com/huff-language) compiler written in pure rust.


## What is a Huff?

Huff is a low-level programming language designed for developing highly optimized smart contracts that run on the Ethereum Virtual Machine (EVM). Huff does not hide the inner workings of the EVM. Instead, Huff exposes its programming stack to the developer for manual manipulation.

Rather than having functions, Huff has macros - individual blocks of bytecode that can be rigorously tested and evaluated using the Huff runtime testing suite.

[Huff](https://github.com/AztecProtocol/huff) was originally developed by the Aztec Protocol team to write [Weierstrudel](https://github.com/aztecprotocol/weierstrudel). Weierstrudel is an on-chain elliptical curve arithmetic library that requires incredibly optimized code that neither [Solidity](https://docs.soliditylang.org/en/v0.8.14/) nor [Yul](https://docs.soliditylang.org/en/v0.8.9/yul.html) could provide.

While EVM experts can use Huff to write highly-efficient smart contracts for use in production, it can also serve as a way for beginners to learn more about the EVM.


## Usage

**Build**
```bash
cargo build
```

**Run Tests**
```bash
cargo test
```


## Blueprint

```ml
utils
├─ refactored utilities
lexer
├─ 
```


## Contributing

All contributions are welcome! We want to make contributing to this project as easy and transparent as possible, whether it's:
  - Reporting a bug
  - Discussing the current state of the code
  - Submitting a fix
  - Proposing new features
  - Becoming a maintainer

We use GitHub issues to track public bugs. Report a bug by [opening a new issue](https://github.com/huff-language/huff-rs/issues/new); it's that easy!

To pass github actions, please run:
```bash
cargo check --all
cargo test --all --all-features
cargo +nightly fmt -- --check
cargo +nightly clippy --all --all-features -- -D warnings
```

In order to fix any formatting issues, run:
```bash
cargo +nightly fmt --all
```
```


## References

- [huffc](https://github.com/huff-language/huffc)
- [ripc](https://github.com/ibraheemdev/ripc)
