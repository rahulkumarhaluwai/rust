fn main() {
    // 1. Move
    let original = String::from("Hello, Rust!");
    let moved = original;

    // Uncomment this to see the compiler error:
    // println!("Original: {}", original);
    // Error: borrow of moved value: `original`

    println!("Moved: {}", moved);

    // 2. Clone
    let first = String::from("Hello, Rust!");
    let second = first.clone();

    println!("First: {}", first);
    println!("Second: {}", second);

    // 3. Function takes ownership and returns it
    let message = String::from("Ownership in Rust");

    let message = print_length_and_return(message);

    println!("After function: {}", message);
}

fn print_length_and_return(s: String) -> String {
    println!("Length: {}", s.len());
    s
}
