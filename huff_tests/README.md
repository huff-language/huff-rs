# Huff Tests

> **Warning**
> This library is currently in development.

A simple, stripped-down testing library for Huff contracts that utilizes [revm](https://github.com/bluealloy/revm) to execute test macros.

> **Note**
> Huff tests are not meant to fully replace the Foundry test suite. Ideally, Huff developers will use a combination of
> Huff tests as well as the Foundry test suite to test their contracts.

## Checklist
- [ ] Create a test runner that utilizes revm
  - [ ] Tests
- [ ] Implement test macros into the lexer / parser
  - [ ] Restrict regular macros / functions from invoking test macros

## Usage
TODO

## Helper functions
TODO

## Examples

Define a test macro within your Huff contract
```js
#define test MY_TEST() {
    // Initialize stack
    0x04
    0xa57b

    // Execute macro we want to test
    MY_MACRO()    
}
```