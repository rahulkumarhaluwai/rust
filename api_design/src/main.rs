mod newtype;
mod builder;
mod shape;

fn main() {
    println!("=== 1. Newtype ===");
    newtype::demo();

    println!("\n=== 2. Newtype + Add ===");
    newtype::add_demo();

    println!("\n=== 3. Builder ===");
    builder::demo();

    println!("\n=== 4. Sealed Trait ===");
    shape::demo();
}