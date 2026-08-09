#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Borrows self immutably
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Borrows another Rectangle immutably
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    // Associated function — no self parameter
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    // Borrows self mutably
    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    println!("rect1 = {:?}", rect1);
    println!("Area of rect1 = {}", rect1.area());

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    let mut square = Rectangle::square(20);

    println!("Before scaling: {:?}", square);

    square.scale(2);

    println!("After scaling: {:?}", square);
}