//! Tests for Either type support (Haskell's Either a b)
//!
//! These tests verify that the Either type from the `either` crate
//! is correctly serialized and deserialized, compatible with Haskell Store.

#![cfg(feature = "either")]

use either::Either;
use serde::{Deserialize, Serialize};
use serde_store::{from_bytes, to_bytes};

/// Helper function to test Either roundtrip
fn assert_either_roundtrip<L, R>(value: &Either<L, R>)
where
    L: Serialize + for<'de> Deserialize<'de> + PartialEq + std::fmt::Debug,
    R: Serialize + for<'de> Deserialize<'de> + PartialEq + std::fmt::Debug,
{
    let serialized = to_bytes(value).expect("Serialization failed");
    let deserialized: Either<L, R> = from_bytes(&serialized).expect("Deserialization failed");
    assert_eq!(
        value, &deserialized,
        "Either roundtrip failed: value changed after serialization/deserialization"
    );
}

#[test]
fn test_either_left_primitives() {
    assert_either_roundtrip(&Either::<i32, String>::Left(42));
    assert_either_roundtrip(&Either::<u64, bool>::Left(12345));
    assert_either_roundtrip(&Either::<bool, i32>::Left(true));
    assert_either_roundtrip(&Either::<f64, String>::Left(3.14159));
}

#[test]
fn test_either_right_primitives() {
    assert_either_roundtrip(&Either::<i32, String>::Right("hello".to_string()));
    assert_either_roundtrip(&Either::<bool, u64>::Right(99999));
    assert_either_roundtrip(&Either::<String, bool>::Right(false));
    assert_either_roundtrip(&Either::<i32, f32>::Right(2.71828));
}

#[test]
fn test_either_with_strings() {
    assert_either_roundtrip(&Either::<String, i32>::Left("left side".to_string()));
    assert_either_roundtrip(&Either::<String, i32>::Right(42));
    assert_either_roundtrip(&Either::<String, String>::Left("left".to_string()));
    assert_either_roundtrip(&Either::<String, String>::Right("right".to_string()));
    
    // Unicode strings
    assert_either_roundtrip(&Either::<String, i32>::Left("こんにちは".to_string()));
    assert_either_roundtrip(&Either::<String, String>::Right("🦀 Rust".to_string()));
}

#[test]
fn test_either_with_vectors() {
    assert_either_roundtrip(&Either::<Vec<i32>, String>::Left(vec![1, 2, 3, 4, 5]));
    assert_either_roundtrip(&Either::<Vec<i32>, String>::Right("error".to_string()));
    assert_either_roundtrip(&Either::<String, Vec<String>>::Right(vec![
        "a".to_string(),
        "b".to_string(),
        "c".to_string(),
    ]));
    
    // Empty vectors
    assert_either_roundtrip(&Either::<Vec<i32>, bool>::Left(vec![]));
}

#[test]
fn test_either_with_options() {
    assert_either_roundtrip(&Either::<Option<i32>, String>::Left(Some(42)));
    assert_either_roundtrip(&Either::<Option<i32>, String>::Left(None));
    assert_either_roundtrip(&Either::<Option<i32>, String>::Right("value".to_string()));
    
    assert_either_roundtrip(&Either::<i32, Option<String>>::Right(Some("test".to_string())));
    assert_either_roundtrip(&Either::<i32, Option<String>>::Right(None));
}

#[test]
fn test_either_nested() {
    // Either containing Either
    assert_either_roundtrip(&Either::<Either<i32, String>, bool>::Left(Either::Left(42)));
    assert_either_roundtrip(&Either::<Either<i32, String>, bool>::Left(Either::Right(
        "nested".to_string(),
    )));
    assert_either_roundtrip(&Either::<Either<i32, String>, bool>::Right(true));
    
    // Deeply nested
    assert_either_roundtrip(&Either::<
        Either<Either<i32, String>, bool>,
        f64,
    >::Left(Either::Left(Either::Left(42))));
}

#[test]
fn test_either_with_tuples() {
    assert_either_roundtrip(&Either::<(i32, i32), String>::Left((10, 20)));
    assert_either_roundtrip(&Either::<(i32, i32), String>::Right("error".to_string()));
    
    assert_either_roundtrip(&Either::<String, (i32, String, bool)>::Right((
        42,
        "test".to_string(),
        true,
    )));
}

#[test]
fn test_either_with_structs() {
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct Point {
        x: i32,
        y: i32,
    }
    
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct Error {
        code: u32,
        message: String,
    }
    
    assert_either_roundtrip(&Either::<Point, Error>::Left(Point { x: 10, y: 20 }));
    assert_either_roundtrip(&Either::<Point, Error>::Right(Error {
        code: 404,
        message: "Not found".to_string(),
    }));
}

#[test]
fn test_either_with_enums() {
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    enum Color {
        Red,
        Green,
        Blue,
    }
    
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    enum Status {
        Ok,
        Error(String),
    }
    
    assert_either_roundtrip(&Either::<Color, Status>::Left(Color::Red));
    assert_either_roundtrip(&Either::<Color, Status>::Right(Status::Ok));
    assert_either_roundtrip(&Either::<Color, Status>::Right(Status::Error(
        "failed".to_string(),
    )));
}

#[test]
fn test_either_binary_stability() {
    // Verify that the same value always produces the same bytes
    let value1 = Either::<i32, String>::Left(42);
    let bytes1 = to_bytes(&value1).unwrap();
    let bytes2 = to_bytes(&value1).unwrap();
    assert_eq!(bytes1, bytes2, "Binary representation should be stable");
    
    let value2 = Either::<i32, String>::Right("hello".to_string());
    let bytes3 = to_bytes(&value2).unwrap();
    let bytes4 = to_bytes(&value2).unwrap();
    assert_eq!(bytes3, bytes4, "Binary representation should be stable");
}

#[test]
fn test_either_size_check() {
    // Left variant
    let left = Either::<u32, String>::Left(42);
    let left_bytes = to_bytes(&left).unwrap();
    
    // Right variant with same types
    let right = Either::<u32, String>::Right("test".to_string());
    let right_bytes = to_bytes(&right).unwrap();
    
    // They should have different sizes due to different content
    println!("Left size: {} bytes", left_bytes.len());
    println!("Right size: {} bytes", right_bytes.len());
    
    // Left should be smaller (1 byte tag + 4 bytes u32 = 5 bytes)
    // Right should be larger (1 byte tag + length prefix + "test" = more bytes)
    assert!(left_bytes.len() < right_bytes.len());
}

#[test]
fn test_either_collections() {
    use std::collections::{BTreeMap, BTreeSet};
    
    let mut map = BTreeMap::new();
    map.insert("key1".to_string(), 1);
    map.insert("key2".to_string(), 2);
    
    assert_either_roundtrip(&Either::<BTreeMap<String, i32>, String>::Left(map));
    assert_either_roundtrip(&Either::<BTreeMap<String, i32>, String>::Right(
        "error".to_string(),
    ));
    
    let mut set = BTreeSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    
    assert_either_roundtrip(&Either::<BTreeSet<i32>, bool>::Left(set));
}

#[test]
fn test_either_multiple_roundtrips() {
    // Test that multiple serialize-deserialize cycles are stable
    let original = Either::<i32, String>::Left(42);
    
    let mut current = original.clone();
    for _ in 0..10 {
        let bytes = to_bytes(&current).unwrap();
        current = from_bytes(&bytes).unwrap();
    }
    
    assert_eq!(original, current, "Value should remain stable after multiple roundtrips");
}

#[test]
fn test_either_edge_cases() {
    // Empty string
    assert_either_roundtrip(&Either::<String, i32>::Left(String::new()));
    assert_either_roundtrip(&Either::<i32, String>::Right(String::new()));
    
    // Zero values
    assert_either_roundtrip(&Either::<i32, i32>::Left(0));
    assert_either_roundtrip(&Either::<i32, i32>::Right(0));
    
    // Max values
    assert_either_roundtrip(&Either::<u64, i64>::Left(u64::MAX));
    assert_either_roundtrip(&Either::<i64, u64>::Right(u64::MAX));
}

#[test]
fn debug_either_format() {
    use either::Either;
    use serde_store::to_bytes;
    
    let left: Either<i32, String> = Either::Left(42);
    let right: Either<i32, String> = Either::Right("hello".to_string());
    
    let left_bytes = to_bytes(&left).unwrap();
    let right_bytes = to_bytes(&right).unwrap();
    
    println!("\nLeft(42) serialized:");
    println!("  Bytes: {:?}", left_bytes);
    println!("  Hex: {}", left_bytes.iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join(" "));
    
    println!("\nRight(\"hello\") serialized:");
    println!("  Bytes: {:?}", right_bytes);
    println!("  Hex: {}", right_bytes.iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join(" "));
}
