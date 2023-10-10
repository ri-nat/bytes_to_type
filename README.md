# bytes_to_type

[![Crates.io](https://img.shields.io/crates/v/bytes_to_type.svg)](https://crates.io/crates/bytes_to_type)
[![Documentation](https://docs.rs/bytes_to_type/badge.svg)](https://docs.rs/bytes_to_type)

A Rust library for converting slices of bytes to vectors of a specific type.

## About

`bytes_to_type` allows you to conveniently convert slices of byte data into vectors of a specific type, handling necessary conversions and memory alignment seamlessly.

### Key Features

- **Type Conversion:** Easily convert slices of bytes into vectors of a specified type.
- **Memory Safety:** Ensures that conversions maintain Rust’s guarantees of memory safety.

## Getting Started

### Installation

Add `bytes_to_type` to your `Cargo.toml` file:

```toml
[dependencies]
bytes_to_type = "0.1.0"
```

## Example

```rust
use bytes_to_type::bytes_to_type;

// This macro will generate a function with the following signature:
// pub fn bytes_to_u32(bytes: &[u8]) -> Result<Vec<u32>, anyhow::Error>
bytes_to_type!(u32);

let bytes = vec![1, 2, 3, 4, 5, 6, 7, 8];
let result = bytes_to_u32(&bytes).expect("Failed to convert bytes to u32");

assert_eq!(result, vec![67305985, 134678021]);
```

## License

This project is licensed under the MIT License.
