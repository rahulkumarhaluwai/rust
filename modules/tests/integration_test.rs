use modules::{shapes, Shape};

#[test]
fn test_public_api() {
    let circle = shapes::create_circle(5.0);

    assert_eq!(circle.area(), std::f64::consts::PI * 25.0);
}

// This would NOT compile because random_shape is private:
//
// #[test]
// fn test_private_function() {
//     let shape = shapes::random_shape();
// }