use modules::{print_all, shapes};

fn main() {
    let shape = shapes::create_random_shape();

    println!("{}", shape.describe());

    let numbers = vec![1, 2, 3, 4];
    print_all(&numbers);

    // This will NOT compile because random_shape is private:
    // let shape = shapes::random_shape();
}
