//! # serde_store
//!
//! A Rust implementation of the [Haskell `store`](https://github.com/mgsloan/store) binary 
//! serialization format using Serde.
//!
//! ## Overview
//!
//! This library provides Serde serializers and deserializers that are compatible with Haskell's 
//! `store` library. It enables inter-operability between Rust and Haskell programs using a 
//! compact, efficient binary format.
//!
//! ## Features
//!
//! - ✅ **Full Haskell `store` compatibility**: Binary format matches Haskell store encoding
//! - ✅ **Little-endian encoding**: Optimized for modern architectures
//! - ✅ **Serde integration**: Works with any type implementing Serde traits
//! - ✅ **Idempotent**: Serialize-deserialize roundtrips preserve data exactly
//! - ✅ **Type-safe**: Leverages Rust's type system for correctness
//! - ✅ **Comprehensive tests**: Extensive test coverage including roundtrip tests
//! - ✅ **Optional features**: Support for `Either` and `SmolStr` types via cargo features
//!
//! ## Quick Start
//!
//! Add to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! serde_store = "0.1"
//! serde = { version = "1.0", features = ["derive"] }
//! ```
//!
//! ### Basic Example
//!
//! ```rust
//! use serde::{Serialize, Deserialize};
//! use serde_store::{to_bytes, from_bytes};
//!
//! #[derive(Serialize, Deserialize, Debug, PartialEq)]
//! struct Person {
//!     name: String,
//!     age: u32,
//!     email: Option<String>,
//! }
//!
//! let person = Person {
//!     name: "Alice".to_string(),
//!     age: 30,
//!     email: Some("alice@example.com".to_string()),
//! };
//!
//! // Serialize
//! let bytes = to_bytes(&person).unwrap();
//!
//! // Deserialize
//! let decoded: Person = from_bytes(&bytes).unwrap();
//!
//! assert_eq!(person, decoded);
//! ```
//!
//! ## Format Specification
//!
//! The format follows Haskell `store` conventions:
//!
//! ### Primitives
//! - **Booleans**: `u8` (0 = false, 1 = true)
//! - **Integers**: Little-endian encoding (i8, i16, i32, i64, u8, u16, u32, u64)
//! - **Floats**: Little-endian encoding (f32, f64)
//!
//! ### Strings and Bytes
//! - **String/Text**: `u64` length (LE) + UTF-8 bytes
//! - **Bytes**: `u64` length (LE) + raw bytes
//! - **Char**: Encoded as a single-character UTF-8 string
//!
//! ### Options
//! - **None**: `u8` tag = 0
//! - **Some(x)**: `u8` tag = 1, followed by serialized value
//!
//! ### Collections
//! - **Vec/Array/Seq**: `u64` length (LE) + elements
//! - **Map**: `u64` count (LE) + key-value pairs
//! - **Set**: `u64` count (LE) + elements
//!
//! ### Tuples
//! - Elements serialized sequentially (no length prefix)
//! - Supported up to 7 elements (matching Haskell Store)
//!
//! ### Structs (Products)
//! - Fields are serialized sequentially
//! - **No length prefix** (length is implicit from schema)
//!
//! ### Enums (Sum Types)
//! - **Discriminant**: `u64` variant index (LE)
//! - Followed by variant data (if any)
//!
//! ## Optional Features
//!
//! Enable features in your `Cargo.toml`:
//!
//! ```toml
//! # Enable Either support (Haskell's Either a b)
//! serde_store = { version = "0.1", features = ["either"] }
//!
//! # Enable SmolStr support (small string optimization)
//! serde_store = { version = "0.1", features = ["smol_str"] }
//!
//! # Enable all features
//! serde_store = { version = "0.1", features = ["either", "smol_str"] }
//! ```
//!
//! ### Using Either
//!
//! ```rust,ignore
//! use either::Either;
//! use serde_store::{to_bytes, from_bytes};
//!
//! // Either works like Haskell's Either a b
//! let left: Either<i32, String> = Either::Left(42);
//! let right: Either<i32, String> = Either::Right("error".to_string());
//!
//! let bytes = to_bytes(&left).unwrap();
//! let decoded: Either<i32, String> = from_bytes(&bytes).unwrap();
//! ```
//!
//! ## Haskell Interoperability
//!
//! This implementation is designed to be binary-compatible with Haskell's `store` library.
//!
//! ### Haskell Side
//!
//! ```haskell
//! {-# LANGUAGE DeriveGeneric #-}
//!
//! import Data.Store
//! import GHC.Generics
//!
//! data Person = Person
//!   { name :: Text
//!   , age :: Word32
//!   , email :: Maybe Text
//!   } deriving (Generic, Show)
//!
//! instance Store Person
//!
//! -- Encode in Haskell
//! bytes = encode (Person "Alice" 30 (Just "alice@example.com"))
//! ```
//!
//! ### Rust Side
//!
//! ```rust
//! # use serde::{Serialize, Deserialize};
//! # use serde_store::{to_bytes, from_bytes};
//! #[derive(Serialize, Deserialize)]
//! struct Person {
//!     name: String,
//!     age: u32,
//!     email: Option<String>,
//! }
//!
//! // Create a person to encode for Haskell
//! let person = Person { 
//!     name: "Alice".to_string(), 
//!     age: 30, 
//!     email: Some("alice@example.com".to_string()),
//! };
//! let bytes = to_bytes(&person).unwrap();
//!
//! // These bytes can be decoded in Haskell using the store library
//! // In Rust, decode them back:
//! let decoded: Person = from_bytes(&bytes).unwrap();
//! ```
//!
//! ## Type Mappings
//!
//! | Rust Type | Haskell Type | Notes |
//! |-----------|--------------|-------|
//! | `bool` | `Bool` | |
//! | `u8`, `u16`, `u32`, `u64` | `Word8`, `Word16`, `Word32`, `Word64` | |
//! | `i8`, `i16`, `i32`, `i64` | `Int8`, `Int16`, `Int32`, `Int64` | |
//! | `f32`, `f64` | `Float`, `Double` | |
//! | `String` | `Text` | |
//! | `Vec<u8>` | `ByteString` | |
//! | `Option<T>` | `Maybe T` | |
//! | `Vec<T>` | `[T]` or `Vector T` | |
//! | `(T1, T2, ...)` | `(T1, T2, ...)` | Up to 7 elements |
//! | `HashMap<K, V>` | `HashMap K V` | |
//! | `BTreeMap<K, V>` | `Map K V` | |
//! | `HashSet<T>` | `HashSet T` | |
//! | `BTreeSet<T>` | `Set T` | |
//! | `Either<L, R>` | `Either L R` | Requires `either` feature |
//! | `SmolStr` | `Text` | Requires `smol_str` feature |
//!
//! ## Advanced Usage
//!
//! ### Complex Enums
//!
//! ```rust
//! use serde::{Serialize, Deserialize};
//! use serde_store::{to_bytes, from_bytes};
//!
//! #[derive(Serialize, Deserialize, Debug, PartialEq)]
//! enum Status {
//!     Active,
//!     Inactive { reason: String },
//!     Pending(u32),
//! }
//!
//! let status = Status::Inactive { reason: "maintenance".to_string() };
//! let bytes = to_bytes(&status).unwrap();
//! let decoded: Status = from_bytes(&bytes).unwrap();
//! assert_eq!(status, decoded);
//! ```
//!
//! ### Nested Collections
//!
//! ```rust
//! use std::collections::BTreeMap;
//! use serde::{Serialize, Deserialize};
//! use serde_store::{to_bytes, from_bytes};
//!
//! #[derive(Serialize, Deserialize, Debug, PartialEq)]
//! struct Config {
//!     settings: BTreeMap<String, Vec<i32>>,
//! }
//!
//! let mut settings = BTreeMap::new();
//! settings.insert("timeouts".to_string(), vec![30, 60, 90]);
//!
//! let config = Config { settings };
//! let bytes = to_bytes(&config).unwrap();
//! let decoded: Config = from_bytes(&bytes).unwrap();
//! assert_eq!(config, decoded);
//! ```
//!
//! ## Error Handling
//!
//! All serialization and deserialization functions return a `Result<T, StoreError>`:
//!
//! ```rust
//! use serde::{Serialize, Deserialize};
//! use serde_store::{to_bytes, from_bytes, error::StoreError};
//!
//! #[derive(Serialize, Deserialize)]
//! struct Data {
//!     value: i32,
//! }
//!
//! let data = Data { value: 42 };
//!
//! match to_bytes(&data) {
//!     Ok(bytes) => {
//!         match from_bytes::<Data>(&bytes) {
//!             Ok(decoded) => println!("Success: {:?}", decoded.value),
//!             Err(e) => eprintln!("Deserialization error: {}", e),
//!         }
//!     }
//!     Err(e) => eprintln!("Serialization error: {}", e),
//! }
//! ```

pub mod deserializer;
pub mod error;
pub mod ordered_map;
pub mod serializer;

// Re-export commonly used functions
pub use deserializer::from_bytes;
pub use serializer::to_bytes;

// Re-export ordered map wrapper for BTreeMap compatibility with Haskell
pub use ordered_map::OrderedMap;