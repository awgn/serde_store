use serde::{Serialize, Deserialize};
use serde_store::to_bytes;

#[derive(Debug, Serialize, Deserialize)]
enum TestEnum {
    VariantA,
    VariantB(i32),
    VariantC { x: i32, y: i32 },
}

fn main() {
    println!("Testing enum serialization format:\n");
    
    let a = TestEnum::VariantA;
    let bytes_a = to_bytes(&a).unwrap();
    println!("VariantA: {} bytes", bytes_a.len());
    println!("  Hex: {}", hex(&bytes_a));
    println!("  Expected (Haskell): 00");
    
    let b = TestEnum::VariantB(42);
    let bytes_b = to_bytes(&b).unwrap();
    println!("\nVariantB(42): {} bytes", bytes_b.len());
    println!("  Hex: {}", hex(&bytes_b));
    println!("  Expected (Haskell): 01 2a 00 00 00");
    
    let c = TestEnum::VariantC { x: 10, y: 20 };
    let bytes_c = to_bytes(&c).unwrap();
    println!("\nVariantC{{x:10, y:20}}: {} bytes", bytes_c.len());
    println!("  Hex: {}", hex(&bytes_c));
    println!("  Expected (Haskell): 02 0a 00 00 00 14 00 00 00");
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join(" ")
}
