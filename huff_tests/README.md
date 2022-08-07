# Huff Tests

> **Warning**
> This library is currently in development.

A simple, stripped-down testing library for Huff contracts that utilizes [revm](https://github.com/bluealloy/revm) to execute test macros.

> **Note**
> Huff tests are not meant to fully replace the Foundry test suite. Ideally, Huff developers will use a combination of
> Huff tests as well as the Foundry test suite to test their contracts.

## Usage
To run tests within a contract from the `huffc` cli, use the `test` subcommand:
```
huffc ./path/to/my/contract/Contract.huff test
```

Format the test output using the `-f` flag:
```
huffc ./path/to/my/contract/Contract.huff test -f <list|table|json>
```

Set environment variables:
```
~TODO~
```

Set explicit calldata or provide a fuzz range for inputs:
```
~TODO~
```

Match specific tests:
```
~TODO~
```

## Helper functions
TODO

## Examples

Define a test macro within your Huff contract
```js
#define test MY_TEST() {
    // Initialize stack
    0x04      // [0x04]
    0xa57b    // [0xa57b, 0x04]

    // Execute macro we want to test
    MY_MACRO()
}
```