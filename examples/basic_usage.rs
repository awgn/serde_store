//! Basic usage example for serde_store
//!
//! This example demonstrates serialization and deserialization of various
//! Rust data types using the Haskell store-compatible format.
//!
//! Run with: cargo run --example basic_usage

use serde::{Deserialize, Serialize};
use serde_store::{from_bytes, to_bytes};
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Person {
    name: String,
    age: u32,
    email: Option<String>,
    tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Company {
    name: String,
    employees: Vec<Person>,
    departments: BTreeMap<String, u32>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum Status {
    Active,
    Inactive { reason: String },
    Pending { priority: u32, deadline: String },
}

fn main() {
    println!("=== Serde Store Basic Usage Examples ===\n");

    // Example 1: Simple primitives
    example_primitives();

    // Example 2: Structs
    example_structs();

    // Example 3: Enums
    example_enums();

    // Example 4: Collections
    example_collections();

    // Example 5: Complex nested structures
    example_complex();

    println!("\n=== All examples completed successfully! ===");
}

fn example_primitives() {
    println!("Example 1: Primitives");
    println!("---------------------");

    // Integers
    let int_val = 42i32;
    let bytes = to_bytes(&int_val).unwrap();
    let decoded: i32 = from_bytes(&bytes).unwrap();
    assert_eq!(int_val, decoded);
    println!("✓ Integer: {} -> {} bytes -> {}", int_val, bytes.len(), decoded);

    // Floating point
    let float_val = 3.14159f64;
    let bytes = to_bytes(&float_val).unwrap();
    let decoded: f64 = from_bytes(&bytes).unwrap();
    assert_eq!(float_val, decoded);
    println!("✓ Float: {} -> {} bytes -> {}", float_val, bytes.len(), decoded);

    // Boolean
    let bool_val = true;
    let bytes = to_bytes(&bool_val).unwrap();
    let decoded: bool = from_bytes(&bytes).unwrap();
    assert_eq!(bool_val, decoded);
    println!("✓ Boolean: {} -> {} bytes -> {}", bool_val, bytes.len(), decoded);

    // String
    let str_val = "Hello, Store!";
    let bytes = to_bytes(&str_val).unwrap();
    let decoded: String = from_bytes(&bytes).unwrap();
    assert_eq!(str_val, &decoded);
    println!(
        "✓ String: '{}' -> {} bytes -> '{}'",
        str_val,
        bytes.len(),
        decoded
    );

    println!();
}

fn example_structs() {
    println!("Example 2: Structs");
    println!("------------------");

    let person = Person {
        name: "Alice".to_string(),
        age: 30,
        email: Some("alice@example.com".to_string()),
        tags: vec!["developer".to_string(), "rust".to_string()],
    };

    let bytes = to_bytes(&person).unwrap();
    let decoded: Person = from_bytes(&bytes).unwrap();
    assert_eq!(person, decoded);

    println!("✓ Person struct:");
    println!("  Original: {:?}", person);
    println!("  Serialized to {} bytes", bytes.len());
    println!("  Decoded: {:?}", decoded);
    println!();
}

fn example_enums() {
    println!("Example 3: Enums");
    println!("----------------");

    // Unit variant
    let status1 = Status::Active;
    let bytes = to_bytes(&status1).unwrap();
    let decoded: Status = from_bytes(&bytes).unwrap();
    assert_eq!(status1, decoded);
    println!("✓ Enum (unit): {:?} -> {} bytes", status1, bytes.len());

    // Struct variant
    let status2 = Status::Inactive {
        reason: "Maintenance".to_string(),
    };
    let bytes = to_bytes(&status2).unwrap();
    let decoded: Status = from_bytes(&bytes).unwrap();
    assert_eq!(status2, decoded);
    println!("✓ Enum (struct): {:?} -> {} bytes", status2, bytes.len());

    // Another struct variant
    let status3 = Status::Pending {
        priority: 1,
        deadline: "2024-12-31".to_string(),
    };
    let bytes = to_bytes(&status3).unwrap();
    let decoded: Status = from_bytes(&bytes).unwrap();
    assert_eq!(status3, decoded);
    println!("✓ Enum (struct): {:?} -> {} bytes", status3, bytes.len());

    println!();
}

fn example_collections() {
    println!("Example 4: Collections");
    println!("----------------------");

    // Vector
    let vec = vec![1, 2, 3, 4, 5];
    let bytes = to_bytes(&vec).unwrap();
    let decoded: Vec<i32> = from_bytes(&bytes).unwrap();
    assert_eq!(vec, decoded);
    println!("✓ Vec<i32>: {:?} -> {} bytes", vec, bytes.len());

    // HashMap
    let mut map = HashMap::new();
    map.insert("key1".to_string(), 100);
    map.insert("key2".to_string(), 200);
    let bytes = to_bytes(&map).unwrap();
    let decoded: HashMap<String, i32> = from_bytes(&bytes).unwrap();
    assert_eq!(map.len(), decoded.len());
    for (k, v) in &map {
        assert_eq!(decoded.get(k), Some(v));
    }
    println!("✓ HashMap: {} entries -> {} bytes", map.len(), bytes.len());

    // BTreeMap (ordered)
    let mut btree = BTreeMap::new();
    btree.insert("a".to_string(), 1);
    btree.insert("b".to_string(), 2);
    btree.insert("c".to_string(), 3);
    let bytes = to_bytes(&btree).unwrap();
    let decoded: BTreeMap<String, i32> = from_bytes(&bytes).unwrap();
    assert_eq!(btree, decoded);
    println!("✓ BTreeMap: {:?} -> {} bytes", btree, bytes.len());

    // Option
    let some_val: Option<String> = Some("value".to_string());
    let bytes = to_bytes(&some_val).unwrap();
    let decoded: Option<String> = from_bytes(&bytes).unwrap();
    assert_eq!(some_val, decoded);
    println!("✓ Option::Some -> {} bytes", bytes.len());

    let none_val: Option<String> = None;
    let bytes = to_bytes(&none_val).unwrap();
    let decoded: Option<String> = from_bytes(&bytes).unwrap();
    assert_eq!(none_val, decoded);
    println!("✓ Option::None -> {} bytes", bytes.len());

    println!();
}

fn example_complex() {
    println!("Example 5: Complex Nested Structures");
    println!("-------------------------------------");

    let mut departments = BTreeMap::new();
    departments.insert("Engineering".to_string(), 50);
    departments.insert("Sales".to_string(), 30);
    departments.insert("HR".to_string(), 10);

    let company = Company {
        name: "Acme Corp".to_string(),
        employees: vec![
            Person {
                name: "Alice".to_string(),
                age: 30,
                email: Some("alice@acme.com".to_string()),
                tags: vec!["engineer".to_string(), "rust".to_string()],
            },
            Person {
                name: "Bob".to_string(),
                age: 25,
                email: None,
                tags: vec!["sales".to_string()],
            },
            Person {
                name: "Charlie".to_string(),
                age: 35,
                email: Some("charlie@acme.com".to_string()),
                tags: vec!["manager".to_string(), "hr".to_string()],
            },
        ],
        departments,
    };

    let bytes = to_bytes(&company).unwrap();
    let decoded: Company = from_bytes(&bytes).unwrap();
    assert_eq!(company, decoded);

    println!("✓ Company struct with {} employees", company.employees.len());
    println!("  Serialized to {} bytes", bytes.len());
    println!("  Company: {}", decoded.name);
    println!("  Departments: {:?}", decoded.departments);
    for emp in &decoded.employees {
        println!(
            "    - {} (age: {}, email: {:?})",
            emp.name, emp.age, emp.email
        );
    }

    // Show byte efficiency
    println!("\n✓ Binary Format Efficiency:");
    println!("  {} bytes for complete company data structure", bytes.len());
    println!(
        "  vs JSON: ~{} bytes (estimated)",
        serde_json::to_string(&company)
            .map(|s| s.len())
            .unwrap_or(0)
    );

    println!();
}