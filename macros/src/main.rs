macro_rules! max {
    // Base case: only one expression
    ($x:expr) => {
        $x
    };

    // Recursive case
    ($x:expr, $($rest:expr),+) => {
        std::cmp::max($x, max!($($rest),+))
    };
}

macro_rules! square {
    ($x:expr) => {
        $x * $x
    };
}

macro_rules! run_closure {
    ($closure:expr) => {{
        let x = 100;

        println!("macro x = {}", x);

        let result = $closure();

        println!("closure result = {}", result);
    }};
}


fn main() {
    println!("max!(3) = {}", max!(3));
    println!("max!(3, 7) = {}", max!(3, 7));
    println!("max!(3, 7, 2, 9, 4) = {}", max!(3, 7, 2, 9, 4));

    // Before running:
    // square!(1 + 2)
    //
    // expands roughly to:
    // 1 + 2 * 1 + 2
    //
    // Because * has higher precedence than +:
    // 1 + (2 * 1) + 2 = 5

    println!("square!(1 + 2) = {}", square!(1 + 2));

    let x = 10;

    run_closure!(|| {
        let x = 20;
        x
    });

    println!("outer x = {}", x);
}