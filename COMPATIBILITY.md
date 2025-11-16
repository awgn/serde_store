# Haskell Store Compatibility Analysis

This document analyzes the compatibility between the Rust `serde_store` implementation and the Haskell `store` library.

## Current Implementation Status

### ✅ Fully Supported Types

The following Haskell `store` types are fully supported in the Rust implementation:

#### Primitives
- `Bool` → `bool`
- `Word8`, `Word16`, `Word32`, `Word64` → `u8`, `u16`, `u32`, `u64`
- `Int8`, `Int16`, `Int32`, `Int64` → `i8`, `i16`, `i32`, `i64`
- `Float`, `Double` → `f32`, `f64`
- `Char` → `char` (UTF-8 encoded)

#### Text and Binary
- `Text` → `String`
- `ByteString` → `Vec<u8>` or `&[u8]`
- `ShortByteString` → `Vec<u8>`
- `Lazy ByteString` → Not directly supported (can use `Vec<u8>`)

#### Containers
- `[]` (lists) → `Vec<T>`
- `Vector` → `Vec<T>`
- `Seq` → `Vec<T>`
- `Map` → `BTreeMap<K, V>`
- `HashMap` → `HashMap<K, V>`
- `Set` → `BTreeSet<T>`
- `HashSet` → `HashSet<T>`
- `IntMap` → Can use `HashMap<i32, V>` or `BTreeMap<i32, V>`
- `IntSet` → Can use `HashSet<i32>` or `BTreeSet<i32>`

#### Maybe and Either
- `Maybe a` → `Option<T>`
- `Either a b` → Custom enum or use Result/Either from external crate

#### Tuples
- `()` → `()`
- `(a, b)` → `(A, B)`
- `(a, b, c)` → `(A, B, C)`
- Up to arbitrary arity (Serde supports this)

#### Product Types
- Haskell records → Rust structs
- Haskell tuple types → Rust tuple structs

#### Sum Types
- Haskell ADTs → Rust enums

## ❌ Not Yet Supported / Missing Types

### 1. Integer (Arbitrary Precision)

**Haskell:**
```haskell
instance Store Integer
```

**Status:** Not implemented in Rust

**Rationale:** Haskell's `Integer` type supports arbitrary precision. The Haskell `store` library has special encoding for this using either `integer-gmp` or `integer-simple`.

**Rust Alternative:**
- Use `num_bigint::BigInt` crate
- Implement custom serialization matching Haskell's encoding

**Action Required:**
```rust
// TODO: Add support for BigInt
use num_bigint::BigInt;

// Need to implement Store-compatible serialization
// that matches Haskell's Integer encoding
```

### 2. Natural (Arbitrary Precision Natural Numbers)

**Haskell:**
```haskell
instance Store Natural
```

**Status:** Not implemented

**Rust Alternative:**
- Use `num_bigint::BigUint`

### 3. Ratio Types

**Haskell:**
```haskell
instance Store a => Store (Ratio a)
```

**Status:** Not implemented

**Rust Alternative:**
- Use `num_rational::Ratio<T>`
- Implement serialization as `(numerator, denominator)` tuple

**Action Required:**
```rust
use num_rational::Ratio;

// Haskell encodes Ratio as (numerator :% denominator)
// which serializes as a tuple
```

### 4. Complex Numbers

**Haskell:**
```haskell
instance Store a => Store (Complex a)
```

**Status:** Not implemented

**Rust Alternative:**
- Use `num_complex::Complex<T>`

**Action Required:**
```rust
use num_complex::Complex;

// Haskell encodes Complex as (real :+ imag)
// Implement as tuple (real, imag)
```

### 5. Fixed-Point Decimals

**Haskell:**
```haskell
instance Store (Fixed a)
instance Store Pico
```

**Status:** Not implemented

**Rust Alternative:**
- Use `rust_decimal::Decimal` or `fixed` crate

### 6. Time Types

**Haskell:** (from `time` package)
```haskell
instance Store Day
instance Store TimeOfDay
instance Store LocalTime
instance Store UTCTime
instance Store NominalDiffTime
instance Store DiffTime
instance Store TimeZone
instance Store ZonedTime
instance Store UniversalTime
instance Store AbsoluteTime
```

**Status:** Not implemented

**Rust Alternative:**
- Use `chrono` crate
- Map Haskell time types to chrono equivalents

**Action Required:**
```rust
use chrono::{DateTime, Utc, NaiveDate, NaiveTime, NaiveDateTime};

// Need to understand Haskell time encoding and map appropriately
// Day -> NaiveDate
// UTCTime -> DateTime<Utc>
// etc.
```

### 7. Array Types

**Haskell:**
```haskell
instance (Ix i, Store i, Store e) => Store (Array i e)
instance (Ix i, IArray UArray e, Store i, Store e) => Store (UArray i e)
```

**Status:** Partially supported (Rust arrays work, but not with arbitrary index types)

**Notes:** Rust arrays `[T; N]` work fine. Haskell's `Array i e` allows arbitrary index types, which doesn't map directly to Rust.

### 8. Storable Vector

**Haskell:**
```haskell
instance Storable a => Store (Storable.Vector a)
```

**Status:** Could be optimized

**Notes:** The current implementation works for `Vec<T>` but doesn't have special optimizations for `Storable` types like the Haskell version.

### 9. Template Haskell Types

**Haskell:**
```haskell
instance Store Name
instance Store NameFlavour
instance Store ModName
instance Store PkgName
instance Store NameSpace
instance Store Info
-- etc.
```

**Status:** Not applicable

**Notes:** These are Haskell-specific types used for metaprogramming. Not relevant for Rust.

### 10. Void Type

**Haskell:**
```haskell
instance Store Void
```

**Status:** Could be added

**Rust Alternative:**
```rust
use std::convert::Infallible;
// or define a custom Void enum
enum Void {}
```

## 🔄 Encoding Differences to Note

### 1. Ordered Maps Magic Marker

Haskell `store` versions >= 0.4 use a magic marker (`1217678090`) for `Map` and `IntMap` to indicate ascending order:

```haskell
markMapPokedInAscendingOrder :: Word32
markMapPokedInAscendingOrder = 1217678090
```

**Current Status:** Not implemented in Rust version

**Action Required:** Add magic marker for `BTreeMap` serialization to match Haskell format exactly.

### 2. Text Encoding

**Haskell store (text >= 2.0):** UTF-8 internal representation
**Haskell store (text < 2.0):** UTF-16 internal representation

**Rust:** Always UTF-8

**Compatibility:** Works with modern Haskell (text >= 2.0) only. For older Haskell versions, text would need UTF-16 conversion.

### 3. Integer Encoding Variants

Haskell has different `Integer` encodings depending on:
- GMP vs integer-simple backend
- Version of integer-gmp

The Rust implementation would need to handle these variants or standardize on one.

## Recommended Additions for Full Compatibility

### High Priority

1. **Add BigInt/BigUint support** for `Integer`/`Natural`
   ```rust
   use num_bigint::{BigInt, BigUint};
   ```

2. **Add ordered map magic marker** for `BTreeMap`
   ```rust
   const ASCENDING_MAP_MARKER: u32 = 1217678090;
   ```

3. **Add Complex number support**
   ```rust
   use num_complex::Complex;
   ```

### Medium Priority

4. **Add Ratio support** for rational numbers
   ```rust
   use num_rational::Ratio;
   ```

5. **Add chrono time types** mapping
   ```rust
   use chrono::*;
   ```

6. **Add Void type**

### Low Priority

7. **Optimize Storable vectors** with unsafe copy operations
8. **Add StaticSize support** for compile-time sized types

## Testing Strategy for Compatibility

To ensure full compatibility, we should:

1. **Cross-language tests:** Create identical data structures in both Haskell and Rust, serialize in one language, deserialize in the other

2. **Property tests:** Use QuickCheck (Haskell) and quickcheck (Rust) to generate random data and verify roundtrip

3. **Binary format tests:** Compare exact byte output for known values

4. **Version compatibility:** Test against different versions of Haskell `store` library

## Example Cross-Language Test Setup

### Haskell Side
```haskell
{-# LANGUAGE DeriveGeneric #-}
import Data.Store
import qualified Data.ByteString as BS

data TestData = TestData
  { tdInt :: Int64
  , tdString :: Text
  , tdList :: [Int32]
  } deriving (Generic, Show)

instance Store TestData

main = do
  let testValue = TestData 42 "hello" [1,2,3]
  let encoded = encode testValue
  BS.writeFile "test.bin" encoded
```

### Rust Side
```rust
use serde::{Serialize, Deserialize};
use serde_store::from_bytes;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct TestData {
    td_int: i64,
    td_string: String,
    td_list: Vec<i32>,
}

fn main() {
    let bytes = std::fs::read("test.bin").unwrap();
    let decoded: TestData = from_bytes(&bytes).unwrap();
    println!("{:?}", decoded);
}
```

## Conclusion

The current Rust implementation covers the most commonly used types from Haskell's `store` library. To achieve 100% compatibility, we need to add:

1. Arbitrary precision integer support (`Integer`, `Natural`)
2. Numeric types (`Complex`, `Ratio`, `Fixed`)
3. Time types (via `chrono`)
4. Magic marker for ordered maps
5. Proper handling of legacy text encodings (UTF-16)

The core binary format is fully compatible with Haskell `store` for all currently implemented types.