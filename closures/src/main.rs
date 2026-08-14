fn main() {
    // 1. Filter words longer than 3 characters and uppercase them
    let words = vec!["hello", "world", "rust", "is", "great"];

    let result: Vec<String> = words
        .iter()
        .filter(|word| word.len() > 3)
        .map(|word| word.to_uppercase())
        .collect();

    println!("Filtered words: {:?}", result);

    // 2. FnMut closure because it modifies the captured counter
    let mut counter = 0;

    let mut increment = || {
        counter += 1;
    };

    for _ in 0..3 {
        increment();
        println!("Counter: {}", counter);
    }

    // 3. move transfers ownership of text into the closure
    let text = String::from("hello");

    let reverse = move || {
        // chars() only borrows text, so the closure is still Fn
        // even though move made the closure own text.
        text.chars().rev().collect::<String>()
    };

    println!("Reversed: {}", reverse());
    println!("Reversed again: {}", reverse());

    // text cannot be used here because ownership was moved
    // into the closure. This follows the ownership rules from Lesson 2.

    // 4. Calculate factorial using fold
    let n = 5;

    let factorial = (1..=n).fold(1, |acc, x| acc * x);

    println!("Factorial: {}", factorial);
}