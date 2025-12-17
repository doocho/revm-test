# evm-test

A simple test program for the [revm](https://github.com/bluealloy/revm) (Rust EVM) library.

## Features

- **ETH Transfer Test**: Transfer ETH between accounts using in-memory database
- **Contract Execution Test**: Execute simple EVM bytecode (2 + 3 = 5)

## Requirements

- Rust 1.70+

## Usage

```bash
cargo run
```

## Output

```
=== REVM Simple Test ===

--- Test 1: Simple ETH Transfer ---
  Transfer successful!
  Success: true
  Receiver balance: 100000000000000000 wei

--- Test 2: Simple Contract Execution ---
  Contract call successful!
  Success: true
  Return value: 5 (expected: 5)
```

## Dependencies

- `revm` - Rust Ethereum Virtual Machine implementation
- `alloy-primitives` - Ethereum primitive types

## License

MIT
