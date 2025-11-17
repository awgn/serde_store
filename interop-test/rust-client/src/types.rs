//! Type definitions matching the Haskell server types
//! These must have the same binary representation when serialized with Store

use serde::{Deserialize, Serialize};


/// Test basic primitive types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestPrimitives {
    #[serde(rename = "tpBool")]
    pub tp_bool: bool,
    #[serde(rename = "tpU8")]
    pub tp_u8: u8,
    #[serde(rename = "tpU16")]
    pub tp_u16: u16,
    #[serde(rename = "tpU32")]
    pub tp_u32: u32,
    #[serde(rename = "tpU64")]
    pub tp_u64: u64,
    #[serde(rename = "tpI8")]
    pub tp_i8: i8,
    #[serde(rename = "tpI16")]
    pub tp_i16: i16,
    #[serde(rename = "tpI32")]
    pub tp_i32: i32,
    #[serde(rename = "tpI64")]
    pub tp_i64: i64,
    #[serde(rename = "tpF32")]
    pub tp_f32: f32,
    #[serde(rename = "tpF64")]
    pub tp_f64: f64,
}

/// Test string types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestStrings {
    #[serde(rename = "tsString")]
    pub ts_string: String,
    #[serde(rename = "tsEmpty")]
    pub ts_empty: String,
    #[serde(rename = "tsUnicode")]
    pub ts_unicode: String,
}

/// Test collections (using OrderedMap for Map compatibility)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestCollections {
    #[serde(rename = "tcList")]
    pub tc_list: Vec<i32>,
    #[serde(rename = "tcMap")]
    pub tc_map: serde_store::OrderedMap<String, i32>,
    #[serde(rename = "tcEmpty")]
    pub tc_empty: Vec<String>,
}

/// Person nested structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Person {
    #[serde(rename = "personName")]
    pub person_name: String,
    #[serde(rename = "personAge")]
    pub person_age: u32,
    #[serde(rename = "personEmail")]
    pub person_email: Option<String>,
}

/// Company nested structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Company {
    #[serde(rename = "companyName")]
    pub company_name: String,
    #[serde(rename = "companyEmployees")]
    pub company_employees: Vec<Person>,
    #[serde(rename = "companyRevenue")]
    pub company_revenue: f64,
}

/// Test nested structures
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestNested {
    #[serde(rename = "tnPerson")]
    pub tn_person: Person,
    #[serde(rename = "tnCompany")]
    pub tn_company: Company,
}

/// Test enums (must match Haskell ADT variant order)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TestEnum {
    VariantA,                                    // Index 0
    VariantB(i32),                               // Index 1
    VariantC { 
        #[serde(rename = "vcX")]
        vc_x: i32, 
        #[serde(rename = "vcY")]
        vc_y: i32 
    },          // Index 2
}

/// Test tuples of various sizes (2-7 elements, matching Haskell Store support)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestTuples {
    #[serde(rename = "ttTuple2")]
    pub tt_tuple2: (i32, String),
    #[serde(rename = "ttTuple3")]
    pub tt_tuple3: (u32, String, f64),
    #[serde(rename = "ttTuple4")]
    pub tt_tuple4: (bool, bool, u32, String),
    #[serde(rename = "ttTuple5")]
    pub tt_tuple5: (u32, i32, f32, String, bool),
    #[serde(rename = "ttTuple6")]
    pub tt_tuple6: (u32, i32, f32, String, bool, bool),
    #[serde(rename = "ttTuple7")]
    pub tt_tuple7: (u32, i32, f32, String, bool, bool, u64),
    #[serde(rename = "ttNested")]
    pub tt_nested: ((u32, u32), (u32, u32)),
    #[serde(rename = "ttWithList")]
    pub tt_with_list: (Vec<i32>, Option<String>, Vec<u32>),
}

/// Test Either type (similar to Haskell's Either)
#[cfg(feature = "either")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestEither {
    #[serde(rename = "teLeftInt")]
    pub te_left_int: either::Either<i32, String>,
    #[serde(rename = "teRightString")]
    pub te_right_string: either::Either<bool, String>,
    #[serde(rename = "teNested")]
    pub te_nested: either::Either<either::Either<i32, String>, bool>,
    #[serde(rename = "teWithOption")]
    pub te_with_option: either::Either<Option<i32>, String>,
    #[serde(rename = "teWithList")]
    pub te_with_list: either::Either<Vec<i32>, String>,
}

// Helper functions to create test data

impl TestPrimitives {
    pub fn sample() -> Self {
        TestPrimitives {
            tp_bool: true,
            tp_u8: 42,
            tp_u16: 1024,
            tp_u32: 65536,
            tp_u64: 123456789,
            tp_i8: -42,
            tp_i16: -1024,
            tp_i32: -65536,
            tp_i64: -123456789,
            tp_f32: 3.14159,
            tp_f64: 2.718281828,
        }
    }
}

impl TestStrings {
    pub fn sample() -> Self {
        TestStrings {
            ts_string: "Hello from Rust!".to_string(),
            ts_empty: String::new(),
            ts_unicode: "こんにちは 🦀 Γειά σου".to_string(),
        }
    }
}

impl TestCollections {
    pub fn sample() -> Self {
        let mut map = serde_store::OrderedMap::new();
        map.insert("alpha".to_string(), 1);
        map.insert("beta".to_string(), 2);
        map.insert("gamma".to_string(), 3);
        
        TestCollections {
            tc_list: vec![1, 2, 3, 4, 5],
            tc_map: map,
            tc_empty: vec![],
        }
    }
}

impl Person {
    pub fn sample(name: &str, age: u32, email: Option<&str>) -> Self {
        Person {
            person_name: name.to_string(),
            person_age: age,
            person_email: email.map(|s| s.to_string()),
        }
    }
}

impl Company {
    pub fn sample() -> Self {
        Company {
            company_name: "Rust Corp".to_string(),
            company_employees: vec![
                Person::sample("Alice", 30, Some("alice@rustcorp.com")),
                Person::sample("Bob", 25, None),
                Person::sample("Charlie", 35, Some("charlie@rustcorp.com")),
            ],
            company_revenue: 1_000_000.50,
        }
    }
}

impl TestNested {
    pub fn sample() -> Self {
        TestNested {
            tn_person: Person::sample("David", 40, Some("david@example.com")),
            tn_company: Company::sample(),
        }
    }
}

impl TestTuples {
    pub fn sample() -> Self {
        TestTuples {
            tt_tuple2: (42, "hello".to_string()),
            tt_tuple3: (100, "world".to_string(), 3.14159),
            tt_tuple4: (true, false, 42, "test".to_string()),
            tt_tuple5: (1, -2, 3.14, "five".to_string(), true),
            tt_tuple6: (10, -20, 2.71, "six".to_string(), true, false),
            tt_tuple7: (100, -200, 1.41, "seven".to_string(), false, true, 999),
            tt_nested: ((1, 2), (3, 4)),
            tt_with_list: (vec![1, 2, 3], Some("optional".to_string()), vec![10, 20, 30]),
        }
    }
}

#[cfg(feature = "either")]
impl TestEither {
    pub fn sample() -> Self {
        use either::Either;
        
        TestEither {
            te_left_int: Either::Left(42),
            te_right_string: Either::Right("hello".to_string()),
            te_nested: Either::Left(Either::Right("nested".to_string())),
            te_with_option: Either::Left(Some(99)),
            te_with_list: Either::Right("error".to_string()),
        }
    }
}

impl TestEnum {
    pub fn samples() -> Vec<Self> {
        vec![
            TestEnum::VariantA,
            TestEnum::VariantB(42),
            TestEnum::VariantC { vc_x: 10, vc_y: 20 },
        ]
    }
}