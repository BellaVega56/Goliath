# Goliath Move Specification Generator

A tool for automatically generating formal specifications for Move smart contracts.

## Features

- Generates module-level specifications
- Analyzes function signatures and generates appropriate specifications
- Handles various Move types and function patterns
- Supports common Move patterns like authorization, initialization, and resource management
- Colored output for better readability

## Installation

1. Make sure you have Rust installed on your system
2. Clone this repository:
```bash
git clone https://github.com/BellaVega56/goliath.git
cd goliath
```
3. Build the project:
```bash
cargo build --release
```

## Usage

Run the tool on a Move source file:

```bash
cargo run -- path/to/your/move/file.move
```

The tool will analyze the file and generate formal specifications for:
- Module invariants
- Function pre-conditions (requires)
- Function post-conditions (ensures)
- State transitions
- Authorization requirements

## Example Output

```move
// Module Specifications
spec module {
    invariant total_supply >= 0;
    invariant forall addr: address: exists<Capabilities>(addr) ==> addr == resource_account;
}

// Function: mint
spec mint(amount: u64) {
    requires initialized();
    requires exists<Capabilities>(resource_account_address());
    requires amount > 0;
    ensures result.value == amount;
    ensures total_supply == old(total_supply) + amount;
}
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT License - see LICENSE file for details 