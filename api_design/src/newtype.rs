use std::ops::Add;

#[derive(Debug)]
pub struct Meters(pub f64);

#[derive(Debug)]
pub struct Feet(pub f64);

pub fn measure_distance(distance: Meters) {
    println!("Distance: {:?}", distance);
}

pub fn demo() {
    let distance = Meters(10.0);

    measure_distance(distance);

    let distance_feet = Feet(10.0);

    // Uncomment this to confirm that Rust rejects it:
    //
    // measure_distance(distance_feet);
}

impl Add for Meters {
    type Output = Meters;

    fn add(self, rhs: Meters) -> Meters {
        Meters(self.0 + rhs.0)
    }
}

pub fn add_demo() {
    let a = Meters(3.0);
    let b = Meters(2.0);

    let total = a + b;

    println!("{:?}", total);
}