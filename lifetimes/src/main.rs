// `longest` takes two string slices that share the same lifetime
// `'a` and returns a string slice with that same lifetime.
//
// The returned reference can only live as long as the references
// it came from.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// `ImportantExcerpt` stores a reference to a string slice.
//
// `'a` represents the lifetime of the data that `part` borrows from.
// The struct cannot outlive the string it is borrowing from.
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    // ========================================================
    // 1. Calling longest() with two genuinely different scopes
    // ========================================================

    // `string1` lives until the end of main().
    let string1 = String::from("long string");

    {
        // `string2` has a smaller scope.
        // It will be dropped when this block ends.
        let string2 = String::from("much longer string");

        // Both strings are alive here, so both references are valid.
        let result = longest(&string1, &string2);

        println!("The longest string is: {}", result);
    }

    // `string2` no longer exists here.
    // `string1` is still valid.
    println!("string1 is still valid: {}", string1);

    // ========================================================
    // 2. Using ImportantExcerpt correctly
    // ========================================================

    // `novel` owns the actual String.
    let novel = String::from("Call me Ishmael.");

    // `excerpt` borrows part of `novel`.
    let excerpt = ImportantExcerpt { part: &novel };

    // This is valid because `novel` is still alive.
    println!("Excerpt: {}", excerpt.part);

    // `excerpt` is dropped before `novel`, so there is no
    // dangling reference.

    // Uncomment this section to see the compiler error.
    //
    // The compiler will complain that `novel2` does not
    // live long enough.
    //
    // let excerpt2;
    //
    // {
    //     // `novel2` only lives inside this block.
    //     let novel2 = String::from("Call me Ishmael.");
    //
    //     // `excerpt2` borrows from `novel2`.
    //     excerpt2 = ImportantExcerpt {
    //         part: &novel2,
    //     };
    //
    // } // `novel2` is dropped here.
    //
    // // ERROR:
    // // `novel2` does not live long enough.
    // //
    // // `excerpt2` still contains a reference to `novel2`,
    // // but `novel2` no longer exists.
    //
    // println!("{}", excerpt2.part);

    // This is an intentionally BROKEN version of `longest`.
    //
    // Uncomment it and the compiler will reject it.
    //
    // fn longest_different<'a, 'b>(
    //     x: &'a str,
    //     y: &'b str,
    // ) -> &'a str {
    //     if x.len() > y.len() {
    //         x
    //     } else {
    //         // ERROR:
    //         //
    //         // `y` has lifetime `'b`, but the function promises
    //         // to return a reference with lifetime `'a`.
    //         //
    //         // Rust cannot guarantee that `'b` lasts as long
    //         // as `'a`.
    //         y
    //     }
    // }

    // The reason this fails:
    //
    // `'a` and `'b` are independent lifetimes, so `y` might
    // stop being valid before the returned `&'a str` is expected
    // to remain valid.
}
