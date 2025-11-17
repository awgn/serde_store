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
- ✅ **Optional features**: Support for `Either` and `SmolStr` types via cargo features

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

### Tuples
- Elements serialized sequentially (no length prefix)
- Supported up to 7 elements (matching Haskell Store)

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

### Optional Features

```toml
# Enable Either support (Haskell's Either a b)
serde_store = { version = "0.1", features = ["either"] }

# Enable SmolStr support (small string optimization)
serde_store = { version = "0.1", features = ["smol_str"] }

# Enable all features
serde_store = { version = "0.1", features = ["either", "smol_str"] }
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

### Using Either (Optional Feature)

```rust
use either::Either;
use serde_store::{to_bytes, from_bytes};

// Either works like Haskell's Either a b
let left: Either<i32, String> = Either::Left(42);
let right: Either<i32, String> = Either::Right("error".to_string());

let bytes = to_bytes(&left).unwrap();
let decoded: Either<i32, String> = from_bytes(&bytes).unwrap();
```

### Using SmolStr (Optional Feature)

```rust
use smol_str::SmolStr;
use serde_store::{to_bytes, from_bytes};

// SmolStr is binary-compatible with String
let s = SmolStr::new("hello");
let bytes = to_bytes(&s).unwrap();

// Can deserialize as String
let as_string: String = from_bytes(&bytes).unwrap();
// Or as SmolStr
let as_smolstr: SmolStr = from_bytes(&bytes).unwrap();
```

## Type Mappings

| Rust Type | Haskell Type | Notes |
|-----------|--------------|-------|
| `bool` | `Bool` | |
| `u8`, `u16`, `u32`, `u64` | `Word8`, `Word16`, `Word32`, `Word64` | |
| `i8`, `i16`, `i32`, `i64` | `Int8`, `Int16`, `Int32`, `Int64` | |
| `f32`, `f64` | `Float`, `Double` | |
| `String` | `Text` | |
| `Vec<u8>` | `ByteString` | |
| `Option<T>` | `Maybe T` | |
| `Vec<T>` | `[T]` or `Vector T` | |
| `(T1, T2, ...)` | `(T1, T2, ...)` | Up to 7 elements |
| `HashMap<K, V>` | `HashMap K V` | |
| `BTreeMap<K, V>` | `Map K V` | |
| `HashSet<T>` | `HashSet T` | |
| `BTreeSet<T>` | `Set T` | |
| `Either<L, R>` | `Either L R` | Requires `either` feature |
| `SmolStr` | `Text` | Requires `smol_str` feature, binary-compatible with `String` |
| Struct | Product type | |
| Enum | Sum type | |

## Implementation Details

### Data Types Supported

The following Rust types are fully supported:

- ✅ All primitive numeric types
- ✅ Strings (UTF-8)
- ✅ Byte arrays and vectors
- ✅ Options (`Option<T>`)
- ✅ Tuples (1-7 elements, matching Haskell Store support)
- ✅ Structs (named and tuple structs)
- ✅ Enums (unit, newtype, tuple, and struct variants)
- ✅ Collections (Vec, HashMap, BTreeMap, HashSet, BTreeSet)
- ✅ Arrays
- ✅ Either type (with `either` feature)
- ✅ SmolStr (with `smol_str` feature, binary-compatible with String)
- ✅ Any type implementing `Serialize`/`Deserialize`

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

# Run with all features
cargo test --all-features

# Run with verbose output
cargo test -- --nocapture

# Run specific test suites
cargo test --test roundtrip_tests
cargo test --test tuple_tests
cargo test --features either --test either_tests
cargo test --features smol_str --test smolstr_tests
```

### Test Coverage

- **Primitive types**: All numeric types, bools, chars
- **Strings**: ASCII, Unicode, empty strings, long strings
- **Collections**: Vectors, maps, sets (both hash-based and tree-based)
- **Complex types**: Nested structs, enums with data, options
- **Tuples**: All sizes (1-7 elements), nested tuples, tuples with complex types
- **Either**: Left/Right variants, nested Either, with Options/Vecs (requires `either` feature)
- **SmolStr**: Short/long strings, Unicode, interchangeability with String (requires `smol_str` feature)
- **Idempotence**: Serialize-deserialize cycles preserve data exactly
- **Binary stability**: Same value always produces same bytes
- **Haskell interop**: Full test suite with Haskell Store echo server

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
- Tuple support (1-7 elements)
- Optional `Either` support (via cargo feature)
- Optional `SmolStr` support (via cargo feature)
- Full Haskell interoperability test suite