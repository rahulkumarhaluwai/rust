use std::num::ParseIntError;

fn parse_and_double(s: &str) -> Result<i32, ParseIntError> {
    // `parse()` returns Result<i32, ParseIntError>.
    //
    // `?` means:
    // - If parsing succeeds, unwrap the i32 and continue.
    // - If parsing fails, immediately return the error.
    let number: i32 = s.parse()?;

    // If we reach this point, parsing was successful.
    Ok(number * 2)
}

// Our own error type for mathematical operations.
//
// We use `Debug` so that we can print errors with `{:?}`.
#[derive(Debug)]
enum MathError {
    DivideByZero,
    NegativeSquareRoot,
}

// ------------------------------------------------------------
// Safe division
// ------------------------------------------------------------

fn safe_divide(a: f64, b: f64) -> Result<f64, MathError> {
    // Division by zero is not allowed in our function.
    if b == 0.0 {
        return Err(MathError::DivideByZero);
    }

    Ok(a / b)
}

// ------------------------------------------------------------
// Safe square root
// ------------------------------------------------------------

fn safe_sqrt(x: f64) -> Result<f64, MathError> {
    // A real-number square root cannot be taken from a
    // negative number.
    if x < 0.0 {
        return Err(MathError::NegativeSquareRoot);
    }

    Ok(x.sqrt())
}

fn divide_then_sqrt(a: f64, b: f64) -> Result<f64, MathError> {
    // First, divide a by b.
    //
    // If safe_divide() returns:
    //     Ok(result) -> `?` gives us the result.
    //     Err(error) -> `?` immediately returns that error.
    let divided = safe_divide(a, b)?;

    // Then take the square root of the division result.
    //
    // Again, `?` propagates the error immediately if one occurs.
    let result = safe_sqrt(divided)?;

    // Both operations succeeded.
    Ok(result)
}

fn main() {
    println!("=== 1. parse_and_double() ===");

    // "21" can successfully be parsed into an i32.
    let result1 = parse_and_double("21");

    match result1 {
        Ok(value) => println!("\"21\" doubled = {}", value),
        Err(error) => println!("Error parsing \"21\": {}", error),
    }

    // "abc" cannot be parsed into an i32.
    // The `?` inside parse_and_double() propagates the
    // ParseIntError back to main().
    let result2 = parse_and_double("abc");

    match result2 {
        Ok(value) => println!("\"abc\" doubled = {}", value),
        Err(error) => println!("Error parsing \"abc\": {}", error),
    }

    println!("\n=== 2. Custom MathError ===");

    // Successful division.
    match safe_divide(10.0, 2.0) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(error) => println!("Division error: {:?}", error),
    }

    // Division by zero.
    match safe_divide(10.0, 0.0) {
        Ok(result) => println!("10 / 0 = {}", result),
        Err(error) => println!("Division error: {:?}", error),
    }

    // Successful square root.
    match safe_sqrt(25.0) {
        Ok(result) => println!("sqrt(25) = {}", result),
        Err(error) => println!("Square root error: {:?}", error),
    }

    // Negative square root.
    match safe_sqrt(-25.0) {
        Ok(result) => println!("sqrt(-25) = {}", result),
        Err(error) => println!("Square root error: {:?}", error),
    }

    println!("\n=== 3. divide_then_sqrt() ===");

    // 100 / 4 = 25
    // sqrt(25) = 5
    //
    // Both operations succeed.
    match divide_then_sqrt(100.0, 4.0) {
        Ok(result) => println!("sqrt(100 / 4) = {}", result),
        Err(error) => println!("Chain error: {:?}", error),
    }

    // --------------------------------------------------------
    // Error from the FIRST operation
    // --------------------------------------------------------

    // 100 / 0 causes DivideByZero.
    //
    // `safe_divide()` returns Err(DivideByZero).
    // The first `?` in divide_then_sqrt() immediately
    // returns that error.
    //
    // safe_sqrt() is NEVER called.
    match divide_then_sqrt(100.0, 0.0) {
        Ok(result) => println!("Result = {}", result),
        Err(error) => println!("Chain error (division): {:?}", error),
    }

    // --------------------------------------------------------
    // Error from the SECOND operation
    // --------------------------------------------------------

    // -100 / 4 = -25
    // sqrt(-25) is invalid.
    //
    // Division succeeds, so the first `?` continues.
    //
    // safe_sqrt(-25) returns Err(NegativeSquareRoot).
    // The second `?` immediately returns that error.
    match divide_then_sqrt(-100.0, 4.0) {
        Ok(result) => println!("Result = {}", result),
        Err(error) => println!("Chain error (square root): {:?}", error),
    }
}