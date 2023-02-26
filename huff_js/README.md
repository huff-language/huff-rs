# Huff JS

A wasm compatible interface to the [Huff](https://huff.sh) Core Compiler.

## Building

[wasm-pack](https://github.com/rustwasm/wasm-pack) is the easiest way to generate a usable JavaScript module from the source.

```bash
# Targeting a bundler (e.g. webpack)
wasm-pack build --out-name huffc --target bundler

# Targeting a browser
wasm-pack build --out-name huffc --target browser

# Targeting nodejs
wasm-pack build --out-name huffc --target nodejs
```

The generated module will be output to a `pkg` directory.

## Usage

The compiled package exposes a method `compile` which accepts a JSON definition of the following format:

```json
{
    "files": {
        "contract.huff": "..."
    },
    "sources": [ "contract.huff" ],
    "constructor_args": "...",
    "alternative_main": "...",
    "alternative_constructor": "..."
}
```

The `compile` method will return the compiler output in the following format:

```json
{
    "errors": undefined,                    // Will be an array of errors if compilation failed
    "contracts": {
      "entry.huff": {
        "bytecode": "...",                  // Deployment bytecode
        "runtime": "...",                   // Runtime bytecode
        "abi": [Object]                     // Generated ABI
      }
    }
}
```

`contracts` will be populated with compiler artifacts for each of the Huff files supplied to `inputs`.

### Example Usage

```js
const { compile } = require('./pkg/huffc.js')

let files = {
    "add.huff": "#define function add(uint256,uint256) nonpayable returns (uint256)\n" +
        "\n" +
        "#define macro MAIN() = {\n" +
        "   // Load our numbers from calldata and add them together.\n" +
        "   0x04 calldataload // [number1]\n" +
        "   0x24 calldataload // [number2]\n" +
        "   add               // [number1+number2]\n" +
        "   // Return our new number.\n" +
        "   0x00 mstore // Store our number in memory.\n" +
        "   0x20 0x00 return // Return it.\n" +
        "}\n"
}

const result = compile({
    files,
    sources: ['add.huff']
});

console.log(result)
```

Outputs:

```json
{
  errors: undefined,
  contracts: Map(1) {
    'add.huff' => {
      bytecode: '600f8060093d393df36004356024350160005260206000f3',
      runtime: '6004356024350160005260206000f3',
      abi: [Object]
    }
  }
}
```