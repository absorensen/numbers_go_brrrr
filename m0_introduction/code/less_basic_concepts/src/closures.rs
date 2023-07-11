pub fn closures() {
    // Closures are anonymous functions and are
    // pretty ubiquitous in idiomatic Rust,
    // so let's take a look.

    let our_closure = |x: u32| x * x;

    let power_two: u32 = our_closure(2); // 4
    let power_four: u32 = our_closure(power_two); // 16

    let data: Vec<u32> = vec![2, 4, 6, 8, 10];
    // We also use closures for iterator adaptors
    let powered: Vec<u32> = data.iter().map(|x| *x * *x).collect();

    // Closures, compared to functions, capture the environment
    // (the variables defined at the time), and can be saved for later.
    // This has some subtleties, does it manipulate the variables
    // that might still be there or does it copy it's entire
    // own environment to keep for later?

    let multiply_by_what: u32 = 1337;
    let multiply_closure = |x: u32| x * multiply_by_what;
    let powered: Vec<u32> = data.iter().map(|x| multiply_closure(*x)).collect();

    // We can save a closure for later, which means it will have to
    // keep a copy of the environment saved.
    // You should circle back to these subtleties later, for now
    // just keep it simple and your usage of closures narrow.

    // You can read more about closures here: https://doc.rust-lang.org/book/ch13-01-closures.html
}