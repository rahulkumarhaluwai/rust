use std::fmt::Debug;

mod sealed {
    pub trait Sealed {}
}

pub trait Shape: sealed::Sealed {
    fn area(&self) -> f64;

    fn describe(&self) -> String {
        format!("This shape has an area of {}", self.area())
    }
}

#[derive(Debug)]
pub struct Circle {
    radius: f64,
}

impl sealed::Sealed for Circle {}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

#[derive(Debug)]
pub struct Rectangle {
    width: f64,
    height: f64,
}

impl sealed::Sealed for Rectangle {}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

fn print_all<T: Debug>(items: &[T]) {
    for item in items {
        println!("{:?}", item);
    }
}

pub fn demo() {
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Rectangle {
            width: 4.0,
            height: 6.0,
        }),
    ];

    for shape in &shapes {
        println!("{}", shape.describe());
    }

    let numbers = vec![1, 2, 3, 4];

    print_all(&numbers);
}