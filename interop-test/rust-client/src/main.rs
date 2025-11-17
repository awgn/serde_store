//! Rust client for testing Store format interoperability with Haskell
//!
//! This client sends various data types to the Haskell server, which deserializes,
//! re-serializes, and echoes them back. We verify that the roundtrip is successful.

mod types;

use anyhow::{Context, Result};
use colored::Colorize;
use serde_store::{from_bytes, to_bytes};
use types::*;

const SERVER_URL: &str = "http://localhost:3000";

fn main() -> Result<()> {
    println!("{}", "=".repeat(80).bright_blue());
    println!("{}", "Rust-Haskell Store Interoperability Test".bright_cyan().bold());
    println!("{}", "=".repeat(80).bright_blue());
    println!();

    // Check server health
    print!("Checking server health... ");
    match check_health() {
        Ok(_) => println!("{}", "✓ Server is running".green()),
        Err(e) => {
            println!("{}", "✗ Server not reachable".red());
            println!("Error: {}", e);
            println!("\nPlease start the Haskell server first:");
            println!("  cd interop-test/haskell-server");
            println!("  stack run");
            return Err(e);
        }
    }
    println!();

    let mut total_tests = 0;
    let mut passed_tests = 0;

    // Test 1: Primitives
    println!("{}", "Test 1: Primitives".yellow().bold());
    total_tests += 1;
    if test_primitives().is_ok() {
        passed_tests += 1;
    }
    println!();

    // Test 2: Strings
    println!("{}", "Test 2: Strings".yellow().bold());
    total_tests += 1;
    if test_strings().is_ok() {
        passed_tests += 1;
    }
    println!();

    // Test 3: Collections
    println!("{}", "Test 3: Collections with OrderedMap".yellow().bold());
    total_tests += 1;
    if test_collections().is_ok() {
        passed_tests += 1;
    }
    println!();

    // Test 4: Nested Structures
    println!("{}", "Test 4: Nested Structures".yellow().bold());
    total_tests += 1;
    if test_nested().is_ok() {
        passed_tests += 1;
    }
    println!();

    // Test 5: Enums
    println!("{}", "Test 5: Enums (ADTs)".yellow().bold());
    total_tests += 1;
    if test_enums().is_ok() {
        passed_tests += 1;
    }
    println!();

    // Test 6: Tuples
    println!("{}", "Test 6: Tuples (2-7 elements)".yellow().bold());
    total_tests += 1;
    if test_tuples().is_ok() {
        passed_tests += 1;
    }
    println!();

    // Test 7: Either
    #[cfg(feature = "either")]
    {
        println!("{}", "Test 7: Either (Haskell's Either a b)".yellow().bold());
        total_tests += 1;
        if test_either().is_ok() {
            passed_tests += 1;
        }
        println!();
    }

    // Summary
    println!("{}", "=".repeat(80).bright_blue());
    println!("{}", "Test Summary".bright_cyan().bold());
    println!("{}", "=".repeat(80).bright_blue());
    println!(
        "Total: {} | Passed: {} | Failed: {}",
        total_tests,
        passed_tests.to_string().green(),
        (total_tests - passed_tests).to_string().red()
    );

    if passed_tests == total_tests {
        println!();
        println!("{}", "🎉 All tests passed! Interoperability verified! 🎉".green().bold());
        Ok(())
    } else {
        println!();
        println!("{}", "❌ Some tests failed".red().bold());
        Err(anyhow::anyhow!("Some tests failed"))
    }
}

fn check_health() -> Result<()> {
    let url = format!("{}/health", SERVER_URL);
    reqwest::blocking::get(&url)
        .context("Failed to connect to server")?
        .error_for_status()
        .context("Server returned error")?;
    Ok(())
}

fn test_primitives() -> Result<()> {
    let original = TestPrimitives::sample();
    println!("  Original: {:?}", original);
    
    let result = roundtrip("/echo/primitives", &original)?;
    
    if result == original {
        println!("  {}", "✓ Roundtrip successful".green());
        println!("  {}", "✓ Values match perfectly".green());
        Ok(())
    } else {
        println!("  {}", "✗ Values don't match!".red());
        println!("  Expected: {:?}", original);
        println!("  Got:      {:?}", result);
        Err(anyhow::anyhow!("Primitives roundtrip failed"))
    }
}

fn test_strings() -> Result<()> {
    let original = TestStrings::sample();
    println!("  Original: {:?}", original);
    
    let result = roundtrip("/echo/strings", &original)?;
    
    if result == original {
        println!("  {}", "✓ Roundtrip successful".green());
        println!("  {}", "✓ UTF-8 strings match (including Unicode)".green());
        Ok(())
    } else {
        println!("  {}", "✗ Values don't match!".red());
        println!("  Expected: {:?}", original);
        println!("  Got:      {:?}", result);
        Err(anyhow::anyhow!("Strings roundtrip failed"))
    }
}

fn test_collections() -> Result<()> {
    let original = TestCollections::sample();
    println!("  Original list: {:?}", original.tc_list);
    println!("  Original map: {:?}", original.tc_map);
    
    let result = roundtrip("/echo/collections", &original)?;
    
    if result == original {
        println!("  {}", "✓ Roundtrip successful".green());
        println!("  {}", "✓ Collections match (Vec and OrderedMap)".green());
        println!("  {}", "✓ OrderedMap magic marker handled correctly".green());
        Ok(())
    } else {
        println!("  {}", "✗ Values don't match!".red());
        println!("  Expected: {:?}", original);
        println!("  Got:      {:?}", result);
        Err(anyhow::anyhow!("Collections roundtrip failed"))
    }
}

fn test_nested() -> Result<()> {
    let original = TestNested::sample();
    println!("  Testing nested Person and Company structures...");
    println!("  Person: {}", original.tn_person.person_name);
    println!("  Company: {} ({} employees)", 
             original.tn_company.company_name,
             original.tn_company.company_employees.len());
    
    let result = roundtrip("/echo/nested", &original)?;
    
    if result == original {
        println!("  {}", "✓ Roundtrip successful".green());
        println!("  {}", "✓ Nested structures preserved".green());
        Ok(())
    } else {
        println!("  {}", "✗ Values don't match!".red());
        Err(anyhow::anyhow!("Nested structures roundtrip failed"))
    }
}

fn test_enums() -> Result<()> {
    let samples = TestEnum::samples();
    
    for (i, original) in samples.iter().enumerate() {
        println!("  Testing variant {}: {:?}", i, original);
        
        let result = roundtrip("/echo/enum", original)?;
        
        if &result == original {
            println!("    {}", "✓ Roundtrip successful".green());
        } else {
            println!("    {}", "✗ Values don't match!".red());
            println!("    Expected: {:?}", original);
            println!("    Got:      {:?}", result);
            return Err(anyhow::anyhow!("Enum variant {} roundtrip failed", i));
        }
    }
    
    println!("  {}", "✓ All enum variants handled correctly".green());
    Ok(())
}

fn roundtrip<T>(endpoint: &str, value: &T) -> Result<T>
where
    T: serde::Serialize + serde::de::DeserializeOwned + std::fmt::Debug,
{
    // Serialize with serde_store
    let encoded = to_bytes(value).context("Failed to serialize value")?;
    println!("  → Serialized to {} bytes", encoded.len());
    
    // Send to Haskell server
    let url = format!("{}{}", SERVER_URL, endpoint);
    let client = reqwest::blocking::Client::new();
    let response = client
        .post(&url)
        .header("Content-Type", "application/octet-stream")
        .body(encoded)
        .send()
        .context("Failed to send request")?;
    
    let status = response.status();
    if !status.is_success() {
        let error_body = response.text().unwrap_or_else(|_| "Unknown error".to_string());
        return Err(anyhow::anyhow!(
            "Server returned error {}: {}",
            status,
            error_body
        ));
    }
    
    // Receive response
    let response_bytes = response.bytes().context("Failed to read response")?;
    println!("  ← Received {} bytes", response_bytes.len());
    
    // Deserialize
    let decoded: T = from_bytes(&response_bytes).context("Failed to deserialize response")?;
    
    Ok(decoded)
}

fn test_tuples() -> Result<()> {
    let original = TestTuples::sample();
    println!("  Testing tuples of various sizes (2-7 elements)...");
    println!("  Tuple2: {:?}", original.tt_tuple2);
    println!("  Tuple3: {:?}", original.tt_tuple3);
    println!("  Tuple7: {:?}", original.tt_tuple7);
    println!("  Nested: {:?}", original.tt_nested);
    
    let result = roundtrip("/echo/tuples", &original)?;
    
    if result == original {
        println!("  {}", "✓ Roundtrip successful".green());
        println!("  {}", "✓ All tuple sizes (2-7) handled correctly".green());
        println!("  {}", "✓ Nested tuples preserved".green());
        println!("  {}", "✓ Tuples with complex types work".green());
        Ok(())
    } else {
        println!("  {}", "✗ Values don't match!".red());
        println!("  Expected: {:?}", original);
        println!("  Got:      {:?}", result);
        Err(anyhow::anyhow!("Tuples roundtrip failed"))
    }
}

#[cfg(feature = "either")]
fn test_either() -> Result<()> {
    let original = TestEither::sample();
    println!("  Testing Either type (compatible with Haskell's Either)...");
    println!("  te_left_int: {:?}", original.te_left_int);
    println!("  te_right_string: {:?}", original.te_right_string);
    println!("  te_nested: {:?}", original.te_nested);
    
    let result = roundtrip("/echo/either", &original)?;
    
    if result == original {
        println!("  {}", "✓ Roundtrip successful".green());
        println!("  {}", "✓ Either::Left handled correctly".green());
        println!("  {}", "✓ Either::Right handled correctly".green());
        println!("  {}", "✓ Nested Either works".green());
        println!("  {}", "✓ Either with Option/Vec works".green());
        Ok(())
    } else {
        println!("  {}", "✗ Values don't match!".red());
        println!("  Expected: {:?}", original);
        println!("  Got:      {:?}", result);
        Err(anyhow::anyhow!("Either roundtrip failed"))
    }
}

#[allow(dead_code)]
fn print_enum_bytes() {
    use crate::types::TestEnum;
    
    let samples = TestEnum::samples();
    for sample in &samples {
        let bytes = serde_store::to_bytes(sample).unwrap();
        println!("{:?}: {} bytes", sample, bytes.len());
        println!("  Hex: {}", bytes.iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join(" "));
    }
}
