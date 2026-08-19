/*With measured per-call overhead, I’d choose dyn Trait when I genuinely need 
heterogeneous values behind one interface—especially when the flexibility of 
storing different implementations outweighs the dispatch cost. If the concrete 
types are known and performance is sensitive to every call, an enum or generics 
is clearly better because it can avoid dynamic dispatch and often enables more optimization.*/
use std::fmt::Debug;

trait Animal {
    fn speak(&self) -> String;
    // This makes Animal NOT object-safe because the method has
    // a generic type parameter.
    fn make_baby<T>(&self) -> T;
}
struct Dog;
impl Animal for Dog {
    fn speak(&self) -> String {
        "Woof!".to_string()
    }
    fn make_baby<T>(&self) -> T {
        todo!()
    }
}
// This produces E0038:
//
// let animals: Vec<Box<dyn Animal>> = vec![Box::new(Dog)];
//
// error[E0038]: the trait `Animal` is not dyn compatible
//
// The reason is that `make_baby<T>` is generic, so the compiler
// cannot create one fixed vtable entry for it.
// ------------------------------------------------------------
// Fix: move the generic method into another trait
// ------------------------------------------------------------
trait Animal {
    fn speak(&self) -> String;
}
trait CanMakeBaby {
    fn make_baby<T>(&self) -> T;
}
impl Animal for Dog {
    fn speak(&self) -> String {
        "Woof!".to_string()
    }
}
impl CanMakeBaby for Dog {
    fn make_baby<T>(&self) -> T {
        todo!()
    }
}
fn animal_example() {
    // Animal is now object-safe.
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog),
    ];
    for animal in &animals {
        println!("{}", animal.speak());
    }
}

trait Shape: Debug {
    fn area(&self) -> f64;
    // Returns a boxed clone of the concrete object.
    fn clone_box(&self) -> Box<dyn Shape>;
}
impl Clone for Box<dyn Shape> {
    fn clone(&self) -> Self {
        self.clone_box()
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
    fn clone_box(&self) -> Box<dyn Shape> {
        Box::new(Circle {
            radius: self.radius,
        })
    }
}
#[derive(Debug)]
struct Square {
    side: f64,
}
impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
    fn clone_box(&self) -> Box<dyn Shape> {
        Box::new(Square {
            side: self.side,
        })
    }
}
fn clone_shapes() {
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Square { side: 4.0 }),
    ];
    // Clone the entire heterogeneous collection.
    let mut cloned_shapes: Vec<Box<dyn Shape>> =
        shapes.iter()
            .map(|shape| shape.clone_box())
            .collect();
    println!("Original:");
    for shape in &shapes {
        println!("{shape:?}, area = {}", shape.area());
    }
    println!("\nClone:");
    for shape in &cloned_shapes {
        println!("{shape:?}, area = {}", shape.area());
    }
    // --------------------------------------------------------
    // Prove that the clone is independent.
    //
    // We cannot directly mutate through dyn Shape because Shape
    // doesn't expose a mutation method. So we demonstrate the
    // independence by replacing the cloned Box with a different
    // concrete shape.
    // --------------------------------------------------------
    cloned_shapes[0] = Box::new(Circle { radius: 10.0 });
    println!("\nAfter mutating/replacing clone:");
    println!("Original first shape:");
    println!("{:?}", shapes[0]);
    println!("area = {}", shapes[0].area());
    println!("\nCloned first shape:");
    println!("{:?}", cloned_shapes[0]);
    println!("area = {}", cloned_shapes[0].area());
}
// ============================================================
// main
// ============================================================
fn main() {
    animal_example();
    println!();
    clone_shapes();
}