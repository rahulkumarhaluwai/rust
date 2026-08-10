use std::collections::HashMap;

// ------------------------------------------------------------
// 1. MEDIAN AND MODE
// ------------------------------------------------------------

fn median_and_mode(numbers: &Vec<i32>) -> (f64, i32) {
    // Clone the Vec so we can sort our own copy without
    // changing the original Vec owned by the caller.
    let mut sorted = numbers.clone();

    // Sort the numbers from smallest to largest.
    sorted.sort();

    // Calculate the median.
    let middle = sorted.len() / 2;

    let median = if sorted.len() % 2 == 0 {
        // If there is an even number of elements,
        // the median is the average of the two middle values.
        (sorted[middle - 1] + sorted[middle]) as f64 / 2.0
    } else {
        // If there is an odd number of elements,
        // the middle element is the median.
        sorted[middle] as f64
    };

    // HashMap stores each number as a key and
    // the number of times it appears as its value.
    let mut counts = HashMap::new();

    for number in &sorted {
        // Get the current count for this number.
        // If the number isn't in the map yet, use 0.
        let count = counts.entry(*number).or_insert(0);

        // Increase the count.
        *count += 1;
    }

    // Find the number with the highest count.
    let mut mode = sorted[0];
    let mut highest_count = 0;

    for (number, count) in &counts {
        if *count > highest_count {
            highest_count = *count;
            mode = *number;
        }
    }

    (median, mode)
}

// ------------------------------------------------------------
// 2. CHARACTER FREQUENCY
// ------------------------------------------------------------

fn character_frequency(sentence: &str) -> HashMap<char, i32> {
    let mut counts = HashMap::new();

    // .chars() safely iterates over the characters in the string.
    for character in sentence.chars() {
        let count = counts.entry(character).or_insert(0);

        *count += 1;
    }

    counts
}

// ------------------------------------------------------------
// 3. DELIBERATE BORROW-CHECKER ERROR
// ------------------------------------------------------------

fn demonstrate_borrowing_error() {
    let mut numbers = vec![10, 20, 30];

    // This creates an immutable borrow of `numbers`.
    let first = &numbers[0];

    // Uncommenting the next line produces:
    //
    // error[E0502]: cannot borrow `numbers` as mutable because
    // it is also borrowed as immutable
    //
    // Why?
    //
    // `first` still refers to `numbers`, so Rust won't allow us
    // to mutably borrow the same Vec at the same time.
    //
    // numbers.push(40);

    // `first` is still usable here because the immutable borrow
    // is still active.
    println!("First number: {}", first);

    // After the last use of `first`, the immutable borrow ends.
    // Now we can mutably borrow the Vec.
    numbers.push(40);

    println!("Numbers after push: {:?}", numbers);
}

// ------------------------------------------------------------
// MAIN
// ------------------------------------------------------------

fn main() {
    // --------------------------------------------------------
    // Median and Mode
    // --------------------------------------------------------

    let numbers = vec![1, 2, 2, 3, 4, 4, 4, 5];

    let (median, mode) = median_and_mode(&numbers);

    println!("Numbers: {:?}", numbers);
    println!("Median: {}", median);
    println!("Mode: {}", mode);

    // --------------------------------------------------------
    // Character Frequency
    // --------------------------------------------------------

    let sentence = "hello rust";

    let frequencies = character_frequency(sentence);

    println!("\nSentence: {}", sentence);
    println!("Character frequencies: {:?}", frequencies);

    // --------------------------------------------------------
    // Borrow Checker Example
    // --------------------------------------------------------

    println!("\nBorrowing example:");
    demonstrate_borrowing_error();
}