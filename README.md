# serde_store

A Rust implementation of the [Haskell `store`](https://github.com/mgsloan/store) binary serialization format using Serde.

## Overview

This library provides Serde serializers and deserializers that are compatible with Haskell's `store` library. It enables inter-operability between Rust and Haskell programs using a compact, efficient binary format.

## Features

- ✅ **Full Haskell `store` compatibility**: Binary format matches Haskell store encoding
- ✅ **Little-endian encoding**: Optimized for modern architectures
- ✅ **Serde integration**: Works with any type implementing Serde traits
- ✅ **Idempotent**: Serialize-deserialize roundtrips preserve data exactly
- ✅ **Type-safe**: Leverages Rust's type system for correctness
- ✅ **Comprehensive tests**: Extensive test coverage including roundtrip tests

## Format Specification

The format follows Haskell `store` conventions:

### Primitives
- **Booleans**: `u8` (0 = false, 1 = true)
- **Integers**: Little-endian encoding (i8, i16, i32, i64, u8, u16, u32, u64)
- **Floats**: Little-endian encoding (f32, f64)

### Strings and Bytes
- **String/Text**: `u64` length (LE) + UTF-8 bytes
- **Bytes**: `u64` length (LE) + raw bytes
- **Char**: Encoded as a single-character UTF-8 string

### Options
- **None**: `u8` tag = 0
- **Some(x)**: `u8` tag = 1, followed by serialized value

### Collections
- **Vec/Array/Seq**: `u64` length (LE) + elements
- **Map**: `u64` count (LE) + key-value pairs
- **Set**: `u64` count (LE) + elements

### Structs and Tuples (Products)
- Fields are serialized sequentially
- **No length prefix** (length is implicit from schema)

### Enums (Sum Types)
- **Discriminant**: `u64` variant index (LE)
- Followed by variant data (if any)

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
serde_store = "0.1"
serde = { version = "1.0", features = ["derive"] }
```

### Basic Example

```rust
use serde::{Serialize, Deserialize};
use serde_store::{to_bytes, from_bytes};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Person {
    name: String,
    age: u32,
    email: Option<String>,
}

fn main() {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
        email: Some("alice@example.com".to_string()),
    };

    // Serialize
    let bytes = to_bytes(&person).unwrap();
    
    // Deserialize
    let decoded: Person = from_bytes(&bytes).unwrap();
    
    assert_eq!(person, decoded);
}
```

### Complex Types

```rust
use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};
use serde_store::{to_bytes, from_bytes};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum Status {
    Active,
    Inactive { reason: String },
    Pending(u32),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Config {
    id: u64,
    name: String,
    settings: BTreeMap<String, i32>,
    status: Status,
}

let mut settings = BTreeMap::new();
settings.insert("timeout".to_string(), 30);
settings.insert("retries".to_string(), 3);

let config = Config {
    id: 123,
    name: "production".to_string(),
    settings,
    status: Status::Active,
};

let bytes = to_bytes(&config).unwrap();
let decoded: Config = from_bytes(&bytes).unwrap();
assert_eq!(config, decoded);
```

## Haskell Interoperability

This implementation is designed to be binary-compatible with Haskell's `store` library.

### Haskell Side

```haskell
{-# LANGUAGE DeriveGeneric #-}

import Data.Store
import GHC.Generics

data Person = Person
  { name :: Text
  , age :: Word32
  , email :: Maybe Text
  } deriving (Generic, Show)

instance Store Person

-- Encode in Haskell
bytes = encode (Person "Alice" 30 (Just "alice@example.com"))

-- Decode in Rust
-- The same bytes can be decoded using serde_store
```

### Rust Side

```rust
#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u32,
    email: Option<String>,
}

// Decode bytes from Haskell
let person: Person = from_bytes(&haskell_bytes).unwrap();

// Encode for Haskell
let bytes = to_bytes(&person).unwrap();
```

## Type Mappings

| Rust Type | Haskell Type |
|-----------|--------------|
| `bool` | `Bool` |
| `u8`, `u16`, `u32`, `u64` | `Word8`, `Word16`, `Word32`, `Word64` |
| `i8`, `i16`, `i32`, `i64` | `Int8`, `Int16`, `Int32`, `Int64` |
| `f32`, `f64` | `Float`, `Double` |
| `String` | `Text` |
| `Vec<u8>` | `ByteString` |
| `Option<T>` | `Maybe T` |
| `Vec<T>` | `[T]` or `Vector T` |
| `(T1, T2, ...)` | `(T1, T2, ...)` |
| `HashMap<K, V>` | `HashMap K V` |
| `BTreeMap<K, V>` | `Map K V` |
| `HashSet<T>` | `HashSet T` |
| `BTreeSet<T>` | `Set T` |
| Struct | Product type |
| Enum | Sum type |

## Implementation Details

### Data Types Supported

The following Rust types are fully supported:

- ✅ All primitive numeric types
- ✅ Strings (UTF-8)
- ✅ Byte arrays and vectors
- ✅ Options
- ✅ Tuples (up to arbitrary arity via Serde)
- ✅ Structs (named and tuple structs)
- ✅ Enums (unit, newtype, tuple, and struct variants)
- ✅ Collections (Vec, HashMap, BTreeMap, HashSet, BTreeSet)
- ✅ Arrays

### Limitations

- **Endianness**: Only little-endian is supported (matching modern Haskell `store`)
- **Schema evolution**: Changes to data types require coordinated updates
- **Self-describing**: The format is NOT self-describing; both sides must know the schema
- **No versioning**: No built-in version negotiation

## Testing

The library includes comprehensive tests:

```bash
# Run all tests
cargo test

# Run with verbose output
cargo test -- --nocapture

# Run specific test suites
cargo test --test roundtrip_tests
```

### Test Coverage

- **Primitive types**: All numeric types, bools, chars
- **Strings**: ASCII, Unicode, empty strings, long strings
- **Collections**: Vectors, maps, sets (both hash-based and tree-based)
- **Complex types**: Nested structs, enums with data, options
- **Idempotence**: Serialize-deserialize cycles preserve data exactly
- **Binary stability**: Same value always produces same bytes

## Performance

The format is designed for efficiency:

- Zero-copy where possible (planned future optimization)
- Direct memory representations for primitives
- Compact encoding (no metadata overhead)
- Predictable size calculation for fixed-size types

## Contributing

Contributions are welcome! Please ensure:

1. All tests pass: `cargo test`
2. Code is formatted: `cargo fmt`
3. No clippy warnings: `cargo clippy`
4. Add tests for new features

## License

This project follows the same license as the Haskell `store` library (MIT).

## References

- [Haskell store library](https://github.com/mgsloan/store)
- [Serde documentation](https://serde.rs/)
- [Binary serialization formats comparison](https://github.com/alecthomas/go_serialization_benchmarks)

## Version History

### 0.1.0 (Initial Release)
- Full serializer implementation
- Full deserializer implementation
- Comprehensive test suite
- Haskell `store` format compatibility
- Support for all common Rust types