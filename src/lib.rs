pub mod deserializer;
pub mod error;
pub mod ordered_map;
pub mod serializer;

// Re-export commonly used functions
pub use deserializer::from_bytes;
pub use serializer::to_bytes;

// Re-export ordered map wrapper for BTreeMap compatibility with Haskell
pub use ordered_map::OrderedMap;
