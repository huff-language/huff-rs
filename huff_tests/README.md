# Huff Tests

A simple, stripped-down testing library for Huff contracts that utilizes [revm](https://github.com/bluealloy/revm) to execute test macros.

> **Note**
> Huff tests are not meant to fully replace the Foundry test suite. Ideally, Huff developers will use a combination of
> Huff tests alongside the Foundry and `foundry-huff` to test their contracts.

## Usage
To run tests within a contract from the `huffc` cli, use the `test` subcommand:
```
huffc ./path/to/my/contract/Contract.huff test
```

Format the test report using the `-f` flag:
```
huffc ./path/to/my/contract/Contract.huff test -f <list|table|json>
```

Match specific tests using the `-m` flag:
```
huffc ./path/to/my/contract/Contract.huff test -m MY_TEST
```

Set environment variables with decorator flags above test macros:

Available Flags:
* `calldata(<string>)`: The calldata flag accepts a string of abi encoded calldata to be included in the test transaction.
* `value(<literal>)`: The value flag accepts a hex literal and determines the callvalue of the test transaction.

```
#[calldata("0xf8a8fd6d00000000000000000000000000000000000000027627abd8d94cf3a4eb06de95"), value(0x01)]
#define test MY_TEST() = takes (0) returns (0) {
    // ...
}

#[value(0x0de0b6b3a7640000)]
#define test MY_TEST_2() = takes (0) returns (0) {
    // ...
}
```

Provide a fuzz range for calldata inputs:
```rust
todo!()
```

## Examples

Define a test macro within your Huff contract
```js
#[calldata("0xf8a8fd6d00000000000000000000000000000000000000027627abd8d94cf3a4eb06de95"), value(0x01)]
#define test MY_TEST() = takes (0) returns (0) {
    // Initialize stack
    0x04 calldataload  // [0x027627abd8d94cf3a4eb06de95]
    callvalue          // [0x01, 0x027627abd8d94cf3a4eb06de95]

    // Execute macro we want to test
    MY_MACRO()
}

#define macro MY_MACRO() = takes (0) returns (0) {
    // ...
}
```