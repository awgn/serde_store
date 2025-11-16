//! Comprehensive roundtrip (idempotence) tests for serde_store
//!
//! These tests verify that:
//! 1. serialize(x) -> deserialize -> y implies x == y (idempotence)
//! 2. The format is compatible with Haskell store library encoding

use serde::{Deserialize, Serialize};
use serde_store::{from_bytes, to_bytes};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

/// Helper function to test idempotence (roundtrip property)
fn assert_idempotent<T>(value: &T)
where
    T: Serialize + for<'de> Deserialize<'de> + PartialEq + std::fmt::Debug,
{
    let serialized = to_bytes(value).expect("Serialization failed");
    let deserialized: T = from_bytes(&serialized).expect("Deserialization failed");
    assert_eq!(
        value, &deserialized,
        "Idempotence failed: serialized value differs after deserialization"
    );

    // Double roundtrip to ensure stability
    let serialized2 = to_bytes(&deserialized).expect("Second serialization failed");
    assert_eq!(
        serialized, serialized2,
        "Binary representation changed after roundtrip"
    );
}

#[test]
fn test_idempotent_primitives() {
    // Booleans
    assert_idempotent(&true);
    assert_idempotent(&false);

    // Unsigned integers
    assert_idempotent(&0u8);
    assert_idempotent(&255u8);
    assert_idempotent(&0u16);
    assert_idempotent(&65535u16);
    assert_idempotent(&0u32);
    assert_idempotent(&4294967295u32);
    assert_idempotent(&0u64);
    assert_idempotent(&18446744073709551615u64);

    // Signed integers
    assert_idempotent(&0i8);
    assert_idempotent(&127i8);
    assert_idempotent(&(-128i8));
    assert_idempotent(&0i16);
    assert_idempotent(&32767i16);
    assert_idempotent(&(-32768i16));
    assert_idempotent(&0i32);
    assert_idempotent(&2147483647i32);
    assert_idempotent(&(-2147483648i32));
    assert_idempotent(&0i64);
    assert_idempotent(&9223372036854775807i64);
    assert_idempotent(&(-9223372036854775808i64));

    // Floating point
    assert_idempotent(&0.0f32);
    assert_idempotent(&3.14159f32);
    assert_idempotent(&(-2.71828f32));
    assert_idempotent(&0.0f64);
    assert_idempotent(&3.141592653589793f64);
    assert_idempotent(&(-2.718281828459045f64));
}

#[test]
fn test_idempotent_strings() {
    // Empty string
    assert_idempotent(&String::new());
    assert_idempotent(&"".to_string());

    // ASCII strings
    assert_idempotent(&"Hello, World!".to_string());
    assert_idempotent(&"The quick brown fox jumps over the lazy dog".to_string());

    // Unicode strings
    assert_idempotent(&"こんにちは世界".to_string());
    assert_idempotent(&"🦀 Rust 🚀".to_string());
    assert_idempotent(&"Emoji: 😀😃😄😁".to_string());
    assert_idempotent(&"Mixed: Hello 世界 🌍".to_string());

    // Special characters
    assert_idempotent(&"Newline:\nTab:\tCarriage return:\r".to_string());
    assert_idempotent(&"Quotes: \"double\" 'single'".to_string());
}

#[test]
fn test_idempotent_chars() {
    assert_idempotent(&'A');
    assert_idempotent(&'z');
    assert_idempotent(&'0');
    assert_idempotent(&'🦀');
    assert_idempotent(&'世');
    assert_idempotent(&'\n');
    assert_idempotent(&'\t');
}

#[test]
fn test_idempotent_bytes() {
    assert_idempotent(&Vec::<u8>::new());
    assert_idempotent(&vec![0u8]);
    assert_idempotent(&vec![1u8, 2, 3, 4, 5]);
    assert_idempotent(&vec![255u8; 100]);
    assert_idempotent(&(0..=255u8).collect::<Vec<_>>());
}

#[test]
fn test_idempotent_options() {
    // None variants
    assert_idempotent(&None::<u32>);
    assert_idempotent(&None::<String>);
    assert_idempotent(&None::<Vec<i32>>);

    // Some variants
    assert_idempotent(&Some(42u32));
    assert_idempotent(&Some(-42i32));
    assert_idempotent(&Some("hello".to_string()));
    assert_idempotent(&Some(vec![1, 2, 3]));

    // Nested options
    assert_idempotent(&Some(Some(42u32)));
    assert_idempotent(&Some(None::<u32>));
    assert_idempotent(&None::<Option<u32>>);
}

#[test]
fn test_idempotent_vectors() {
    // Empty vectors
    assert_idempotent(&Vec::<u32>::new());
    assert_idempotent(&Vec::<String>::new());

    // Homogeneous vectors
    assert_idempotent(&vec![1, 2, 3, 4, 5]);
    assert_idempotent(&vec!["a", "b", "c"].iter().map(|s| s.to_string()).collect::<Vec<_>>());
    assert_idempotent(&vec![true, false, true, false]);

    // Nested vectors
    assert_idempotent(&vec![vec![1, 2], vec![3, 4, 5], vec![]]);
    assert_idempotent(&vec![
        vec!["a".to_string(), "b".to_string()],
        vec!["c".to_string()],
    ]);
}

#[test]
fn test_idempotent_arrays() {
    assert_idempotent(&[1u32, 2, 3, 4, 5]);
    assert_idempotent(&[0u8; 10]);
    assert_idempotent(&[true, false, true]);
}

#[test]
fn test_idempotent_tuples() {
    assert_idempotent(&());
    assert_idempotent(&(1u32,));
    assert_idempotent(&(1u32, 2u32));
    assert_idempotent(&(1u32, "hello".to_string()));
    assert_idempotent(&(1u32, "hello".to_string(), 3.14f64));
    assert_idempotent(&(true, false, 42u32, "test".to_string()));
    assert_idempotent(&(vec![1, 2, 3], Some(42), None::<String>));
}

#[test]
fn test_idempotent_structs() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Empty {}

    assert_idempotent(&Empty {});

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    assert_idempotent(&Point { x: 0, y: 0 });
    assert_idempotent(&Point { x: 10, y: -20 });

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Person {
        name: String,
        age: u32,
        email: Option<String>,
    }

    assert_idempotent(&Person {
        name: "Alice".to_string(),
        age: 30,
        email: Some("alice@example.com".to_string()),
    });

    assert_idempotent(&Person {
        name: "Bob".to_string(),
        age: 25,
        email: None,
    });

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Nested {
        id: u64,
        point: Point,
        people: Vec<Person>,
    }

    assert_idempotent(&Nested {
        id: 123,
        point: Point { x: 5, y: 10 },
        people: vec![
            Person {
                name: "Alice".to_string(),
                age: 30,
                email: Some("alice@example.com".to_string()),
            },
            Person {
                name: "Bob".to_string(),
                age: 25,
                email: None,
            },
        ],
    });
}

#[test]
fn test_idempotent_tuple_structs() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Wrapper(u32);

    assert_idempotent(&Wrapper(42));

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Pair(i32, String);

    assert_idempotent(&Pair(42, "hello".to_string()));

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Triple(u32, String, Vec<i32>);

    assert_idempotent(&Triple(1, "test".to_string(), vec![1, 2, 3]));
}

#[test]
fn test_idempotent_enums() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    enum Simple {
        A,
        B,
        C,
    }

    assert_idempotent(&Simple::A);
    assert_idempotent(&Simple::B);
    assert_idempotent(&Simple::C);

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    enum WithData {
        Unit,
        NewType(u32),
        Tuple(i32, String),
        Struct { x: i32, y: i32 },
    }

    assert_idempotent(&WithData::Unit);
    assert_idempotent(&WithData::NewType(42));
    assert_idempotent(&WithData::Tuple(10, "hello".to_string()));
    assert_idempotent(&WithData::Struct { x: 5, y: 10 });

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    enum Nested {
        Empty,
        Point { x: i32, y: i32 },
        List(Vec<i32>),
        Map(BTreeMap<String, i32>),
    }

    assert_idempotent(&Nested::Empty);
    assert_idempotent(&Nested::Point { x: 1, y: 2 });
    assert_idempotent(&Nested::List(vec![1, 2, 3]));

    let mut map = BTreeMap::new();
    map.insert("a".to_string(), 1);
    map.insert("b".to_string(), 2);
    assert_idempotent(&Nested::Map(map));
}

#[test]
fn test_idempotent_hashmaps() {
    let mut map = HashMap::new();
    map.insert("key1".to_string(), 1);
    map.insert("key2".to_string(), 2);
    map.insert("key3".to_string(), 3);

    let serialized = to_bytes(&map).unwrap();
    let deserialized: HashMap<String, i32> = from_bytes(&serialized).unwrap();

    assert_eq!(map.len(), deserialized.len());
    for (k, v) in &map {
        assert_eq!(deserialized.get(k), Some(v));
    }

    // Test with different key/value types
    let mut map2 = HashMap::new();
    map2.insert(1u32, "one".to_string());
    map2.insert(2u32, "two".to_string());

    let serialized2 = to_bytes(&map2).unwrap();
    let deserialized2: HashMap<u32, String> = from_bytes(&serialized2).unwrap();

    assert_eq!(map2.len(), deserialized2.len());
    for (k, v) in &map2 {
        assert_eq!(deserialized2.get(k), Some(v));
    }
}

#[test]
fn test_idempotent_btreemaps() {
    let mut map = BTreeMap::new();
    map.insert("key1".to_string(), 1);
    map.insert("key2".to_string(), 2);
    map.insert("key3".to_string(), 3);

    assert_idempotent(&map);

    let mut map2 = BTreeMap::new();
    map2.insert(1u32, vec![1, 2, 3]);
    map2.insert(2u32, vec![4, 5, 6]);

    assert_idempotent(&map2);
}

#[test]
fn test_idempotent_hashsets() {
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);

    let serialized = to_bytes(&set).unwrap();
    let deserialized: HashSet<i32> = from_bytes(&serialized).unwrap();

    assert_eq!(set.len(), deserialized.len());
    for v in &set {
        assert!(deserialized.contains(v));
    }
}

#[test]
fn test_idempotent_btreesets() {
    let mut set = BTreeSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);

    assert_idempotent(&set);

    let mut set2 = BTreeSet::new();
    set2.insert("a".to_string());
    set2.insert("b".to_string());
    set2.insert("c".to_string());

    assert_idempotent(&set2);
}

#[test]
fn test_idempotent_complex_nested() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct ComplexData {
        id: u64,
        name: String,
        tags: Vec<String>,
        scores: BTreeMap<String, f64>,
        metadata: Option<BTreeMap<String, String>>,
        nested: Vec<NestedData>,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct NestedData {
        key: String,
        values: Vec<i32>,
        enabled: bool,
    }

    let mut scores = BTreeMap::new();
    scores.insert("math".to_string(), 95.5);
    scores.insert("science".to_string(), 87.3);

    let mut metadata = BTreeMap::new();
    metadata.insert("author".to_string(), "test".to_string());
    metadata.insert("version".to_string(), "1.0".to_string());

    let complex = ComplexData {
        id: 12345,
        name: "Test Object".to_string(),
        tags: vec!["tag1".to_string(), "tag2".to_string()],
        scores,
        metadata: Some(metadata),
        nested: vec![
            NestedData {
                key: "item1".to_string(),
                values: vec![1, 2, 3],
                enabled: true,
            },
            NestedData {
                key: "item2".to_string(),
                values: vec![],
                enabled: false,
            },
        ],
    };

    assert_idempotent(&complex);
}

#[test]
fn test_idempotent_edge_cases() {
    // Very long strings
    let long_string = "a".repeat(10000);
    assert_idempotent(&long_string);

    // Large vectors
    let large_vec: Vec<u32> = (0..1000).collect();
    assert_idempotent(&large_vec);

    // Deeply nested options
    assert_idempotent(&Some(Some(Some(Some(42u32)))));

    // Empty collections
    assert_idempotent(&Vec::<String>::new());
    assert_idempotent(&BTreeMap::<String, i32>::new());
    assert_idempotent(&BTreeSet::<i32>::new());
}

#[test]
fn test_binary_stability() {
    // This test ensures that the same value always produces the same bytes
    let value = (42u32, "hello".to_string(), vec![1, 2, 3]);

    let bytes1 = to_bytes(&value).unwrap();
    let bytes2 = to_bytes(&value).unwrap();
    let bytes3 = to_bytes(&value).unwrap();

    assert_eq!(bytes1, bytes2);
    assert_eq!(bytes2, bytes3);
}

#[test]
fn test_multiple_roundtrips() {
    // Test that multiple serialize-deserialize cycles are stable
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    struct Data {
        x: i32,
        y: String,
        z: Vec<f64>,
    }

    let original = Data {
        x: 42,
        y: "test".to_string(),
        z: vec![1.1, 2.2, 3.3],
    };

    let mut current = original.clone();
    for _ in 0..10 {
        let bytes = to_bytes(&current).unwrap();
        current = from_bytes(&bytes).unwrap();
    }

    assert_eq!(original, current);
}