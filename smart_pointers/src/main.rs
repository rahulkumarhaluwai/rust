use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    children: Vec<Rc<RefCell<Node>>>,
}

// Mutate a child through a separate Rc clone.
fn mutate_child(child: Rc<RefCell<Node>>) {
    let mut node = child.borrow_mut();
    node.value = 999;
}

fn main() {
    // Create the root.
    let root = Rc::new(RefCell::new(Node {
        value: 0,
        children: Vec::new(),
    }));

    // Create three children.
    let child1 = Rc::new(RefCell::new(Node {
        value: 1,
        children: Vec::new(),
    }));

    let child2 = Rc::new(RefCell::new(Node {
        value: 2,
        children: Vec::new(),
    }));

    let child3 = Rc::new(RefCell::new(Node {
        value: 3,
        children: Vec::new(),
    }));

    // Build the tree.
    root.borrow_mut().children.push(Rc::clone(&child1));
    root.borrow_mut().children.push(Rc::clone(&child2));
    root.borrow_mut().children.push(Rc::clone(&child3));

    println!("Before mutation:");
    println!("{:#?}", root);

    // Create another Rc pointing to the SAME child.
    let separate_clone = Rc::clone(&child2);

    // Mutate child2 through the separate Rc.
    mutate_child(separate_clone);

    println!("After mutation:");
    println!("{:#?}", root);

    // The original tree sees the mutation.
    assert_eq!(root.borrow().children[1].borrow().value, 999);

    // --------------------------------------------------
    // Deliberately trigger a RefCell panic.
    // --------------------------------------------------

    let cell = RefCell::new(10);

    let first_borrow = cell.borrow_mut();

    // This panics because first_borrow is still alive.
    let _second_borrow = cell.borrow_mut();

    // first_borrow is never dropped before the second borrow.
}

/*Rc<RefCell<T>> is fine in a single thread because its non-atomic reference counting 
and runtime borrowing can be controlled there, but Rust refuses to send it across 
threads because two threads could simultaneously modify the non-atomic Rc count, 
causing a data race and potentially corrupting the reference count. */