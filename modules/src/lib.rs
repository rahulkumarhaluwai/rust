use rand::Rng;
use std::fmt::Debug;

pub trait Shape {
    fn area(&self) -> f64;

    fn describe(&self) -> String {
        format!("This shape has an area of {}", self.area())
    }
}

#[derive(Debug)]
pub struct Circle {
    pub radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

#[derive(Debug)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

pub fn print_all<T: Debug>(items: &[T]) {
    for item in items {
        println!("{:?}", item);
    }
}

pub mod shapes {
    use super::*;

    // Private helper
    fn random_shape() -> Box<dyn Shape> {
        let mut rng = rand::rng();
        let choice = rng.random_range(0..2);

        if choice == 0 {
            Box::new(Circle { radius: 5.0 })
        } else {
            Box::new(Rectangle {
                width: 4.0,
                height: 6.0,
            })
        }
    }

    // Public function
    pub fn create_random_shape() -> Box<dyn Shape> {
        random_shape()
    }

    // Public function with input validation
    pub fn create_circle(radius: f64) -> Circle {
        if radius < 0.0 {
            panic!("Radius cannot be negative");
        }

        Circle { radius }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_circle() {
        let circle = shapes::create_circle(5.0);

        assert_eq!(circle.area(), std::f64::consts::PI * 25.0);
    }

    #[test]
    fn test_random_shape() {
        let shape = shapes::create_random_shape();

        let area = shape.area();

        assert_eq!(
            area == std::f64::consts::PI * 25.0 || area == 24.0,
            true
        );

        println!("Random shape area: {}", area);
    }

    #[test]
    #[should_panic]
    fn test_negative_radius() {
        shapes::create_circle(-5.0);
    }
}