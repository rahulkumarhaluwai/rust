fn main() {
    // 1. Filter and uppercase words
    let words = vec!["hello", "world", "rust", "is", "great"];

    let result: Vec<String> = words
        .iter()
        .filter(|word| word.len() > 3)
        .map(|word| word.to_uppercase())
        .collect();

    println!("Filtered words: {:?}", result);

    // 2. FnMut closure because it changes counter
    let mut counter = 0;

    let mut increment = || {
        counter += 1;
    };

    for _ in 0..3 {
        increment();
        println!("Counter: {}", counter);
    }

    // 3. move gives ownership of text to the closure
    let text = String::from("hello");

    let reverse = move || {
        // text is borrowed here, not consumed
        // so the closure implements Fn
        text.chars().rev().collect::<String>()
    };

    println!("Reversed: {}", reverse());
    println!("Reversed again: {}", reverse());

    // text cannot be used here because ownership was moved
    // into the closure.

    // 4. Factorial using fold
    let n = 5;

    let factorial = (1..=n).fold(1, |acc, x| acc * x);

    println!("Factorial: {}", factorial);
}