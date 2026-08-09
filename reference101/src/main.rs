// 1. Return the longest word using a string slice
fn longest_word(s: &str) -> &str {
    let mut longest = "";

    for word in s.split_whitespace() {
        if word.len() > longest.len() {
            longest = word;
        }
    }

    longest
}

// 2. Mutable + immutable reference at the same time
fn mutable_and_immutable() {
    let mut x = String::from("hello");

    let r1 = &mut x;
    let r2 = &x; // ERROR: cannot borrow `x` as immutable because it is also borrowed as mutable

    println!("{r1}");
    println!("{r2}");
}

// 3. Dangling reference
fn dangling_reference() -> &String {
    let s = String::from("hello");

    &s // ERROR: missing lifetime specifier / cannot return reference to local variable `s`
}

fn main() {
    let sentence = "Rust makes systems programming powerful";

    let word = longest_word(sentence);
    println!("Longest word: {word}");

    // Uncomment to see the borrow checker error:
    // mutable_and_immutable();

    // Uncomment to see the dangling-reference error:
    // let reference = dangling_reference();
}