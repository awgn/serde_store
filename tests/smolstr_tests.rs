//! Tests for SmolStr support
//!
//! These tests verify that SmolStr (string type with inline storage)
//! works correctly with serde_store serialization/deserialization.

#![cfg(feature = "smol_str")]

use smol_str::SmolStr;
use serde::{Deserialize, Serialize};
use serde_store::{from_bytes, to_bytes};

/// Helper function to test SmolStr roundtrip
fn assert_smolstr_roundtrip<T>(value: &T)
where
    T: Serialize + for<'de> Deserialize<'de> + PartialEq + std::fmt::Debug,
{
    let serialized = to_bytes(value).expect("Serialization failed");
    let deserialized: T = from_bytes(&serialized).expect("Deserialization failed");
    assert_eq!(
        value, &deserialized,
        "SmolStr roundtrip failed: value changed after serialization/deserialization"
    );
}

#[test]
fn test_smolstr_basic() {
    let s = SmolStr::new("hello");
    assert_smolstr_roundtrip(&s);
}

#[test]
fn test_smolstr_empty() {
    let s = SmolStr::new("");
    assert_smolstr_roundtrip(&s);
}

#[test]
fn test_smolstr_short() {
    // SmolStr inlines strings up to 23 bytes
    let s = SmolStr::new("short");
    assert_smolstr_roundtrip(&s);
}

#[test]
fn test_smolstr_long() {
    // Longer than inline threshold
    let s = SmolStr::new("This is a very long string that exceeds the inline storage capacity");
    assert_smolstr_roundtrip(&s);
}

#[test]
fn test_smolstr_unicode() {
    let s = SmolStr::new("こんにちは世界");
    assert_smolstr_roundtrip(&s);
    
    let s2 = SmolStr::new("🦀 Rust 🚀");
    assert_smolstr_roundtrip(&s2);
}

#[test]
fn test_smolstr_in_struct() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Person {
        name: SmolStr,
        city: SmolStr,
    }
    
    let person = Person {
        name: SmolStr::new("Alice"),
        city: SmolStr::new("Wonderland"),
    };
    
    assert_smolstr_roundtrip(&person);
}

#[test]
fn test_smolstr_in_vec() {
    let names = vec![
        SmolStr::new("Alice"),
        SmolStr::new("Bob"),
        SmolStr::new("Charlie"),
    ];
    
    assert_smolstr_roundtrip(&names);
}

#[test]
fn test_smolstr_with_option() {
    assert_smolstr_roundtrip(&Some(SmolStr::new("present")));
    assert_smolstr_roundtrip(&None::<SmolStr>);
}

#[test]
fn test_smolstr_in_tuple() {
    let tuple = (SmolStr::new("first"), 42u32, SmolStr::new("second"));
    assert_smolstr_roundtrip(&tuple);
}

#[test]
fn test_smolstr_mixed_with_string() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Mixed {
        smol: SmolStr,
        regular: String,
    }
    
    let mixed = Mixed {
        smol: SmolStr::new("smol"),
        regular: "regular".to_string(),
    };
    
    assert_smolstr_roundtrip(&mixed);
}

#[test]
fn test_smolstr_binary_format() {
    // Verify SmolStr serializes the same as String
    let smol = SmolStr::new("test");
    let string = "test".to_string();
    
    let smol_bytes = to_bytes(&smol).unwrap();
    let string_bytes = to_bytes(&string).unwrap();
    
    assert_eq!(
        smol_bytes, string_bytes,
        "SmolStr should serialize identically to String"
    );
}

#[test]
fn test_smolstr_interchangeable_with_string() {
    // Serialize as String, deserialize as SmolStr
    let original_string = "interchangeable".to_string();
    let bytes = to_bytes(&original_string).unwrap();
    let as_smolstr: SmolStr = from_bytes(&bytes).unwrap();
    
    assert_eq!(original_string, as_smolstr.as_str());
    
    // Serialize as SmolStr, deserialize as String
    let original_smol = SmolStr::new("interchangeable");
    let bytes2 = to_bytes(&original_smol).unwrap();
    let as_string: String = from_bytes(&bytes2).unwrap();
    
    assert_eq!(original_smol.as_str(), as_string);
}

#[test]
fn test_smolstr_edge_cases() {
    // Exactly at inline threshold (23 bytes for SmolStr)
    let s = SmolStr::new("1234567890123456789012");
    assert_smolstr_roundtrip(&s);
    
    // Just over inline threshold
    let s2 = SmolStr::new("12345678901234567890123");
    assert_smolstr_roundtrip(&s2);
    
    // Very long string
    let long = "x".repeat(1000);
    let s3 = SmolStr::new(&long);
    assert_smolstr_roundtrip(&s3);
}

#[test]
fn test_smolstr_special_chars() {
    let s = SmolStr::new("Line1\nLine2\tTabbed");
    assert_smolstr_roundtrip(&s);
    
    let s2 = SmolStr::new("Quote: \"hello\"");
    assert_smolstr_roundtrip(&s2);
}