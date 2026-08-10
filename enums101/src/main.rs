// An enum allows us to represent different kinds of shapes.
//
// Each variant can store different data:
// - Circle stores its radius.
// - Rectangle stores its width and height.
// - Triangle stores its base and height.
#[derive(Debug)]
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64),
}

// Calculates the area of a shape.
//
// We take a reference (&Shape) because we only need to read the shape,
// not take ownership of it.
//
// The match must handle EVERY Shape variant.
// There is intentionally no catch-all (_) arm.
fn area(shape: &Shape) -> f64 {
    match shape {
        // For a circle: area = π × radius²
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,

        // For a rectangle: area = width × height
        Shape::Rectangle(width, height) => width * height,

        // For a triangle: area = ½ × base × height
        Shape::Triangle(base, height) => 0.5 * base * height,
    }
}

// Finds and returns the first even number in a slice.
//
// &[i32] means the function borrows a slice of i32 values.
// Option<i32> means the function can return:
// - Some(number) if an even number is found.
// - None if there is no even number.
fn find_first_even(nums: &[i32]) -> Option<i32> {
 
   // Loop through each number in the slice.
    //
    // `&num` would give us a reference, so `&` in the pattern
    // dereferences it and gives us the actual i32 value.
    for &num in nums {
        // An even number has no remainder when divided by 2.
        if num % 2 == 0 {
            // Return the first even number we find.
            return Some(num);
        }
    }

    // If the loop finishes without finding an even number,
    // return None.
    None
}

fn main() {
    // Create different Shape enum values.
    let circle = Shape::Circle(5.0);
    let rectangle = Shape::Rectangle(4.0, 6.0);
    let triangle = Shape::Triangle(10.0, 3.0);

    // Pass references to the shapes because area() only needs
    // to read them and does not need to take ownership.
    println!("Circle area: {}", area(&circle));
    println!("Rectangle area: {}", area(&rectangle));
    println!("Triangle area: {}", area(&triangle));

    // Create an array of numbers.
    let numbers = [1, 3, 7, 8, 11];

    // find_first_even() returns an Option<i32>.
    //
    // `Some(number)` means an even number was found.
    // `None` means there was no even number.
    match find_first_even(&numbers) {
        Some(number) => println!("First even number: {}", number),
        None => println!("There are no even numbers."),
    }
}
