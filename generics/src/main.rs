use std::fmt::Debug;

// A trait defines behavior that different types can implement.
trait Shape {
    fn area(&self) -> f64;

    // Default method: every Shape gets this automatically.
    fn describe(&self) -> String {
        format!("This shape has an area of {}", self.area())
    }
}

#[derive(Debug)]
struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

// Generic function: works with any type that implements Debug.
fn print_all<T: Debug>(items: &[T]) {
    for item in items {
        println!("{:?}", item);
    }
}

fn main() {
    // dyn Shape allows different concrete types in the same collection.
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Rectangle {
            width: 4.0,
            height: 6.0,
        }),
    ];

    // Each object can call the Shape methods through dynamic dispatch.
    for shape in &shapes {
        println!("{}", shape.describe());
    }

    let numbers = vec![1, 2, 3, 4];
    print_all(&numbers);
}

// Generics are useful when one concrete type is used with flexible behavior.
// dyn Trait is useful when different concrete types need to be treated through one interface.