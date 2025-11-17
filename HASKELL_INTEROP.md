# Haskell Interoperability Guide

This guide explains how to use `serde_store` for data exchange between Rust and Haskell programs using the `store` library.

## Quick Start

### Haskell Side

```haskell
{-# LANGUAGE DeriveGeneric #-}
{-# LANGUAGE OverloadedStrings #-}

import Data.Store
import Data.Text (Text)
import qualified Data.ByteString as BS
import GHC.Generics

-- Define your data type
data Person = Person
  { personName :: Text
  , personAge :: Word32
  , personEmail :: Maybe Text
  } deriving (Generic, Show, Eq)

-- Derive Store instance automatically
instance Store Person

-- Encode data to binary
main :: IO ()
main = do
  let person = Person "Alice" 30 (Just "alice@example.com")
  let encoded = encode person
  BS.writeFile "person.bin" encoded
  putStrLn "Encoded person to person.bin"
```

### Rust Side

```rust
use serde::{Serialize, Deserialize};
use serde_store::from_bytes;
use std::fs;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Person {
    person_name: String,
    person_age: u32,
    person_email: Option<String>,
}

fn main() {
    // Read the binary file created by Haskell
    let bytes = fs::read("person.bin").expect("Failed to read file");
    
    // Deserialize
    let person: Person = from_bytes(&bytes).expect("Failed to deserialize");
    
    println!("Decoded person: {:?}", person);
    // Output: Person { person_name: "Alice", person_age: 30, person_email: Some("alice@example.com") }
}
```

## Type Correspondence

### Basic Types

| Haskell | Rust | Notes |
|---------|------|-------|
| `Bool` | `bool` | |
| `Word8` | `u8` | |
| `Word16` | `u16` | |
| `Word32` | `u32` | |
| `Word64` | `u64` | |
| `Int8` | `i8` | |
| `Int16` | `i16` | |
| `Int32` | `i32` | |
| `Int64` | `i64` | |
| `Float` | `f32` | |
| `Double` | `f64` | |
| `Char` | `char` | UTF-8 encoded |
| `Text` | `String` | Requires text >= 2.0 (UTF-8) |
| `ByteString` | `Vec<u8>` | |

### Container Types

| Haskell | Rust | Notes |
|---------|------|-------|
| `Maybe a` | `Option<T>` | |
| `[a]` | `Vec<T>` | Lists → Vectors |
| `Vector a` | `Vec<T>` | |
| `Seq a` | `Vec<T>` | |
| `Map k v` | `BTreeMap<K, V>` | Ordered |
| `HashMap k v` | `HashMap<K, V>` | Unordered |
| `Set a` | `BTreeSet<T>` | Ordered |
| `HashSet a` | `HashSet<T>` | Unordered |
| `IntMap v` | `HashMap<i32, V>` | Or BTreeMap |
| `IntSet` | `HashSet<i32>` | Or BTreeSet |

### Composite Types

| Haskell | Rust | Example |
|---------|------|---------|
| `(a, b)` | `(A, B)` | Tuples |
| `(a, b, c)` | `(A, B, C)` | |
| Record | Struct | See below |
| ADT | Enum | See below |

## Field Naming Conventions

**Important:** Haskell and Rust have different naming conventions. You need to ensure field names match exactly in the binary format.

### Haskell (camelCase)
```haskell
data Person = Person
  { personName :: Text
  , personAge :: Word32
  } deriving (Generic)
```

### Rust (snake_case with field renaming)
```rust
#[derive(Serialize, Deserialize)]
struct Person {
    #[serde(rename = "personName")]
    person_name: String,
    #[serde(rename = "personAge")]
    person_age: u32,
}
```

**OR** use Rust field names that match Haskell:

```rust
#[derive(Serialize, Deserialize)]
struct Person {
    personName: String,  // Violates Rust conventions but matches Haskell
    personAge: u32,
}
```

**OR** configure Haskell to use snake_case (not recommended):

```haskell
{-# LANGUAGE DeriveAnyClass #-}
data Person = Person
  { person_name :: Text
  , person_age :: Word32
  } deriving (Generic, Store)
```

## Enums and ADTs

### Simple Enum

**Haskell:**
```haskell
data Status = Active | Inactive | Pending
  deriving (Generic, Show)

instance Store Status
```

**Rust:**
```rust
#[derive(Serialize, Deserialize, Debug)]
enum Status {
    Active,    // variant index 0
    Inactive,  // variant index 1
    Pending,   // variant index 2
}
```

### Enum with Data

**Haskell:**
```haskell
data Result = Success Text | Error Word32 Text
  deriving (Generic)

instance Store Result
```

**Rust:**
```rust
#[derive(Serialize, Deserialize)]
enum Result {
    Success(String),           // variant 0
    Error(u32, String),        // variant 1
}
```

### Enum with Named Fields

**Haskell:**
```haskell
data Event = 
    Click { x :: Int32, y :: Int32 }
  | KeyPress { key :: Text }
  deriving (Generic)

instance Store Event
```

**Rust:**
```rust
#[derive(Serialize, Deserialize)]
enum Event {
    Click { x: i32, y: i32 },
    KeyPress { key: String },
}
```

## Complete Example: Bidirectional Communication

### Shared Data Schema

**Haskell:** `Types.hs`
```haskell
{-# LANGUAGE DeriveGeneric #-}
{-# LANGUAGE OverloadedStrings #-}

module Types where

import Data.Store
import Data.Text (Text)
import Data.Map (Map)
import qualified Data.Map as Map
import Data.Word
import GHC.Generics

data User = User
  { userId :: Word64
  , userName :: Text
  , userEmail :: Maybe Text
  , userTags :: [Text]
  } deriving (Generic, Show, Eq)

instance Store User

data Message = Message
  { msgId :: Word64
  , msgFrom :: User
  , msgTo :: [User]
  , msgContent :: Text
  , msgMetadata :: Map Text Text
  } deriving (Generic, Show, Eq)

instance Store Message
```

**Rust:** `types.rs`
```rust
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    #[serde(rename = "userId")]
    pub user_id: u64,
    #[serde(rename = "userName")]
    pub user_name: String,
    #[serde(rename = "userEmail")]
    pub user_email: Option<String>,
    #[serde(rename = "userTags")]
    pub user_tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Message {
    #[serde(rename = "msgId")]
    pub msg_id: u64,
    #[serde(rename = "msgFrom")]
    pub msg_from: User,
    #[serde(rename = "msgTo")]
    pub msg_to: Vec<User>,
    #[serde(rename = "msgContent")]
    pub msg_content: String,
    #[serde(rename = "msgMetadata")]
    pub msg_metadata: BTreeMap<String, String>,
}
```

### Haskell: Encode and Send

```haskell
import qualified Data.ByteString as BS
import Types

sendMessage :: IO ()
sendMessage = do
  let alice = User 1 "Alice" (Just "alice@example.com") ["admin", "user"]
  let bob = User 2 "Bob" Nothing ["user"]
  
  let msg = Message
        { msgId = 1001
        , msgFrom = alice
        , msgTo = [bob]
        , msgContent = "Hello from Haskell!"
        , msgMetadata = Map.fromList [("priority", "high")]
        }
  
  let encoded = encode msg
  BS.writeFile "message.bin" encoded
  putStrLn "Message sent to message.bin"
```

### Rust: Receive and Decode

```rust
use serde_store::from_bytes;
use std::fs;

mod types;
use types::Message;

fn main() {
    let bytes = fs::read("message.bin").expect("Failed to read message");
    let msg: Message = from_bytes(&bytes).expect("Failed to decode");
    
    println!("Received message #{}", msg.msg_id);
    println!("From: {} (ID: {})", msg.msg_from.user_name, msg.msg_from.user_id);
    println!("To: {} recipient(s)", msg.msg_to.len());
    println!("Content: {}", msg.msg_content);
    println!("Metadata: {:?}", msg.msg_metadata);
}
```

## Common Pitfalls

### 1. Text Encoding Version Mismatch

**Problem:** Haskell `text < 2.0` uses UTF-16, `text >= 2.0` uses UTF-8

**Solution:** Ensure you're using `text >= 2.0` in Haskell, or handle the conversion manually.

```haskell
-- Check your text version
-- In cabal file or stack.yaml, ensure:
-- text >= 2.0
```

### 2. Field Order Matters

**Problem:** Fields must be in the same order

**Haskell:**
```haskell
data Point = Point { x :: Int32, y :: Int32 }
```

**Rust:** ✅ Correct
```rust
struct Point { x: i32, y: i32 }
```

**Rust:** ❌ Wrong (different order)
```rust
struct Point { y: i32, x: i32 }
```

### 3. Enum Variant Order

**Problem:** Variant indices must match

**Haskell:**
```haskell
data Color = Red | Green | Blue  -- Red=0, Green=1, Blue=2
```

**Rust:** ✅ Correct
```rust
enum Color {
    Red,    // 0
    Green,  // 1
    Blue,   // 2
}
```

**Rust:** ❌ Wrong
```rust
enum Color {
    Blue,   // 0 - doesn't match!
    Red,    // 1
    Green,  // 2
}
```

### 4. Collection Type Mismatch

**Problem:** Using unordered HashMap when Haskell uses ordered Map

**Haskell:**
```haskell
import Data.Map (Map)  -- Ordered

data Config = Config { settings :: Map Text Int }
```

**Rust:** ✅ Use BTreeMap (ordered)
```rust
use std::collections::BTreeMap;

struct Config {
    settings: BTreeMap<String, i32>,
}
```

**Rust:** ⚠️ HashMap works but order differs
```rust
use std::collections::HashMap;

struct Config {
    settings: HashMap<String, i32>,  // May serialize differently
}
```

## Testing Interoperability

### Create Test Files in Both Languages

**Haskell:** Generate test data
```haskell
-- generate_test_data.hs
import Data.Store
import qualified Data.ByteString as BS

main = do
  -- Write various test cases
  BS.writeFile "test_int.bin" (encode (42 :: Int32))
  BS.writeFile "test_string.bin" (encode ("Hello" :: Text))
  BS.writeFile "test_list.bin" (encode ([1,2,3,4,5] :: [Int32]))
```

**Rust:** Verify decoding
```rust
// verify_test_data.rs
use serde_store::from_bytes;
use std::fs;

fn main() {
    let int_val: i32 = from_bytes(&fs::read("test_int.bin").unwrap()).unwrap();
    assert_eq!(int_val, 42);
    
    let str_val: String = from_bytes(&fs::read("test_string.bin").unwrap()).unwrap();
    assert_eq!(str_val, "Hello");
    
    let list_val: Vec<i32> = from_bytes(&fs::read("test_list.bin").unwrap()).unwrap();
    assert_eq!(list_val, vec![1, 2, 3, 4, 5]);
    
    println!("All tests passed!");
}
```

## Performance Considerations

1. **Binary Size:** Store format is compact but not as small as specialized formats
2. **Speed:** Very fast - direct memory operations for primitives
3. **Memory:** Single buffer allocation for serialization
4. **Zero-copy:** Not yet optimized (future improvement)

## Debugging Tips

### Inspect Binary Format

**Rust:**
```rust
let bytes = to_bytes(&value).unwrap();
println!("Encoded to {} bytes: {:02x?}", bytes.len(), bytes);
```

**Haskell:**
```haskell
import qualified Data.ByteString as BS
import Text.Printf (printf)

let encoded = encode value
BS.putStr $ BS.concatMap (BS.pack . printf "%02x ") encoded
```

### Validate Roundtrip

**Test in same language first:**

```rust
let original = MyStruct { ... };
let bytes = to_bytes(&original).unwrap();
let decoded: MyStruct = from_bytes(&bytes).unwrap();
assert_eq!(original, decoded);
```

```haskell
let original = MyStruct { ... }
let decoded = decode (encode original) :: Either PeekException MyStruct
decoded `shouldBe` Right original
```

## Version Compatibility

- **Haskell store:** >= 0.7.0 recommended
- **Rust serde_store:** 0.2.1
- **Haskell text:** >= 2.0 required for UTF-8
- **Endianness:** Little-endian only (x86, ARM modern)

## Further Resources

- [Haskell store library](https://github.com/mgsloan/store)
- [Serde documentation](https://serde.rs/)
- [Store format specification](https://hackage.haskell.org/package/store)

## Getting Help

If data doesn't deserialize correctly:

1. Check field names and order
2. Verify type mappings
3. Confirm text package version (>= 2.0)
4. Test with simple types first
5. Use binary inspection to debug
