use std::mem::{align_of, size_of};
#[repr(C)]
struct BadOrder {
    flag: bool,   // 1 byte
    value: u64,  // 8 bytes
    small: u8,   // 1 byte
}
#[repr(C)]
struct GoodOrder {
    value: u64,  // 8 bytes
    flag: bool,   // 1 byte
    small: u8,   // 1 byte
}
fn main() {
    println!("--- 1. Reordering fields ---");
    println!("BadOrder:");
    println!("  size  = {}", size_of::<BadOrder>());
    println!("  align = {}", align_of::<BadOrder>());
    println!("GoodOrder:");
    println!("  size  = {}", size_of::<GoodOrder>());
    println!("  align = {}", align_of::<GoodOrder>());
    // --------------------------------------------------
    // 2. Add a fourth field and calculate manually
    // --------------------------------------------------
    #[repr(C)]
    struct FourFields {
        flag: bool,
        value: u64,
        small: u8,
        medium: u16,
    }
    println!("\n--- 2. Four fields ---");
    println!("FourFields:");
    println!("  size  = {}", size_of::<FourFields>());
    println!("  align = {}", align_of::<FourFields>());
    // --------------------------------------------------
    // 3. repr(C) vs repr(Rust)
    // --------------------------------------------------
    #[repr(C)]
    struct FfiStruct {
        value: u64,
        flag: bool,
        small: u8,
    }
    struct RustStruct {
        value: u64,
        flag: bool,
        small: u8,
    }
    println!("\n--- 3. repr(C) vs repr(Rust) ---");
    println!("FfiStruct:");
    println!("  size  = {}", size_of::<FfiStruct>());
    println!("  align = {}", align_of::<FfiStruct>());
    println!("RustStruct:");
    println!("  size  = {}", size_of::<RustStruct>());
    println!("  align = {}", align_of::<RustStruct>());
}