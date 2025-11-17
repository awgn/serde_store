use serde::{Serialize, Deserialize};
use serde_store::to_bytes;

#[derive(Debug, Serialize, Deserialize)]
enum TestEnum {
    VariantA,
    VariantB(i32),
    VariantC { 
        #[serde(rename = "vcX")]
        vc_x: i32, 
        #[serde(rename = "vcY")]
        vc_y: i32 
    },
}

fn main() {
    let a = TestEnum::VariantA;
    let b = TestEnum::VariantB(42);
    let c = TestEnum::VariantC { vc_x: 10, vc_y: 20 };
    
    let bytes_a = to_bytes(&a).unwrap();
    let bytes_b = to_bytes(&b).unwrap();
    let bytes_c = to_bytes(&c).unwrap();
    
    println!("VariantA: {} bytes", bytes_a.len());
    println!("  Hex: {}", hex(&bytes_a));
    
    println!("\nVariantB(42): {} bytes", bytes_b.len());
    println!("  Hex: {}", hex(&bytes_b));
    
    println!("\nVariantC{{10, 20}}: {} bytes", bytes_c.len());
    println!("  Hex: {}", hex(&bytes_c));
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join(" ")
}
