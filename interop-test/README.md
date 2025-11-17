# Rust-Haskell Store Interoperability Tests

This directory contains integration tests to verify that the Rust `serde_store` implementation is fully compatible with Haskell's `store` library.

## Overview

The test setup consists of:

1. **Haskell Server** (`haskell-server/`): A Scotty-based HTTP server that:
   - Receives binary data serialized with Store format
   - Deserializes it using Haskell's `store` library
   - Re-serializes it back to binary
   - Returns the result to the client

2. **Rust Client** (`rust-client/`): A Rust application that:
   - Serializes various data types using `serde_store`
   - Sends them to the Haskell server
   - Receives the echoed data back
   - Verifies that the roundtrip is successful

## Test Coverage

The tests verify interoperability for:

- ✅ **Primitives**: bool, u8, u16, u32, u64, i8, i16, i32, i64, f32, f64
- ✅ **Strings**: UTF-8 text including Unicode characters
- ✅ **Collections**: Vec (List), OrderedMap (Map with ascending order marker)
- ✅ **Nested Structures**: Complex nested data types
- ✅ **Enums**: Algebraic Data Types with multiple variants
- ✅ **Options**: Maybe/Option types

## Prerequisites

### Haskell
- Stack (Haskell build tool)
- GHC 9.2 or later

### Rust
- Rust 1.70 or later
- Cargo

## Running the Tests

### Step 1: Start the Haskell Server

```bash
cd haskell-server
stack build
stack run
```

The server will start on `http://localhost:3000` and display available endpoints:

```
================================================
Haskell Store Echo Server
================================================
Starting on port 3000...

Endpoints:
  POST /echo/primitives   - Echo TestPrimitives
  POST /echo/strings      - Echo TestStrings
  POST /echo/collections  - Echo TestCollections
  POST /echo/nested       - Echo TestNested
  POST /echo/enum         - Echo TestEnum
  POST /echo              - Echo with type tag
  GET  /health            - Health check

Ready to accept connections...
================================================
```

### Step 2: Run the Rust Client

In a new terminal:

```bash
cd rust-client
cargo run
```

The client will execute all tests and display results:

```
================================================================================
Rust-Haskell Store Interoperability Test
================================================================================

Checking server health... ✓ Server is running

Test 1: Primitives
  Original: TestPrimitives { tp_bool: true, tp_u8: 42, ... }
  → Serialized to 47 bytes
  ← Received 47 bytes
  ✓ Roundtrip successful
  ✓ Values match perfectly

Test 2: Strings
  ...

================================================================================
Test Summary
================================================================================
Total: 5 | Passed: 5 | Failed: 0

🎉 All tests passed! Interoperability verified! 🎉
```

## Data Type Mapping

### Haskell → Rust

| Haskell Type | Rust Type | Notes |
|--------------|-----------|-------|
| `Bool` | `bool` | |
| `Word8`, `Word16`, `Word32`, `Word64` | `u8`, `u16`, `u32`, `u64` | |
| `Int8`, `Int16`, `Int32`, `Int64` | `i8`, `i16`, `i32`, `i64` | |
| `Float`, `Double` | `f32`, `f64` | |
| `Text` | `String` | UTF-8 encoded (requires text >= 2.0) |
| `[a]` | `Vec<T>` | |
| `Map k v` | `OrderedMap<K, V>` | **Important**: Use `OrderedMap` wrapper for compatibility |
| `Maybe a` | `Option<T>` | |
| ADT (sum type) | `enum` | Variant order must match |
| Record (product type) | `struct` | Field order must match |

### Field Naming Convention

**Important**: Haskell uses camelCase, Rust uses snake_case. Use `#[serde(rename = "...")]` to match:

**Haskell:**
```haskell
data Person = Person
  { personName :: Text
  , personAge :: Word32
  }
```

**Rust:**
```rust
#[derive(Serialize, Deserialize)]
struct Person {
    #[serde(rename = "personName")]
    person_name: String,
    #[serde(rename = "personAge")]
    person_age: u32,
}
```

## Critical: OrderedMap for BTreeMap

Haskell's `store` library (>= 0.4) uses a magic marker `1217678090` for `Map` and `IntMap` to indicate ascending order. In Rust, you **must** use the `OrderedMap` wrapper instead of plain `BTreeMap`:

```rust
use serde_store::OrderedMap;

#[derive(Serialize, Deserialize)]
struct MyStruct {
    // ✓ Correct - compatible with Haskell Map
    data: OrderedMap<String, i32>,
    
    // ✗ Wrong - will fail interop with Haskell
    // data: BTreeMap<String, i32>,
}
```

## Troubleshooting

### Server Not Starting

If the Haskell server fails to build:
```bash
cd haskell-server
stack clean
stack build
```

### Port Already in Use

If port 3000 is already in use, modify the port in:
- `haskell-server/app/Main.hs`: Change `scotty 3000` to another port
- `rust-client/src/main.rs`: Change `const SERVER_URL` accordingly

### Test Failures

If tests fail:

1. **Check field order**: Fields must be in the same order in both Haskell and Rust
2. **Check field names**: Use `#[serde(rename = "...")]` to match Haskell camelCase
3. **Check enum variant order**: Variants must be in the same order
4. **Use OrderedMap**: Always use `OrderedMap` for maps, not `BTreeMap`
5. **Check Haskell text version**: Ensure `text >= 2.0` for UTF-8 compatibility

### Verbose Logging

The Haskell server logs all operations. Check the server terminal for detailed deserialization/serialization logs.

## Adding New Test Cases

### 1. Define the type in Haskell

Edit `haskell-server/src/Types.hs`:
```haskell
data MyNewType = MyNewType
  { myField :: Int32
  } deriving (Generic, Show, Eq)

instance Store MyNewType
```

### 2. Add endpoint in Haskell

Edit `haskell-server/app/Main.hs`:
```haskell
post "/echo/mynewtype" $ echoHandler (Proxy :: Proxy MyNewType) "mynewtype"
```

### 3. Define the type in Rust

Edit `rust-client/src/types.rs`:
```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MyNewType {
    #[serde(rename = "myField")]
    pub my_field: i32,
}
```

### 4. Add test in Rust

Edit `rust-client/src/main.rs`:
```rust
fn test_mynewtype() -> Result<()> {
    let original = MyNewType { my_field: 42 };
    let result = roundtrip("/echo/mynewtype", &original)?;
    assert_eq!(result, original);
    Ok(())
}
```

## Architecture

```
┌─────────────┐                          ┌──────────────────┐
│             │  HTTP POST (binary)      │                  │
│  Rust       │─────────────────────────>│  Haskell         │
│  Client     │                          │  Server (Scotty) │
│             │                          │                  │
│ serde_store │  HTTP Response (binary)  │  store library   │
│ serializer  │<─────────────────────────│  deserializer    │
│             │                          │  + serializer    │
└─────────────┘                          └──────────────────┘
      │                                           │
      │                                           │
      ▼                                           ▼
   Binary data                               Binary data
   (Store format)                            (Store format)
   Little Endian                             Little Endian
```

## Binary Format Verification

The test verifies that:
1. Rust serialization → Haskell deserialization works
2. Haskell re-serialization → Rust deserialization works
3. The roundtrip produces identical values

This ensures 100% binary format compatibility between the two implementations.

## Performance

The tests are designed for correctness, not performance. For production use:
- The Haskell server could be optimized with Warp directly
- Batch operations could be implemented
- Connection pooling could be added

## License

These tests are part of the serde_store project.