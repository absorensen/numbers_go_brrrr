// The pub keyword results in the function being viewable outside the file. 
// For more explanation: https://doc.rust-lang.org/std/keyword.pub.html
// The fn keyword denotes a function
// run() is the name of the function, 
// normally there might be arguments such as run(my_int: i32, my_float: f32)
// after () there is an invisible -> (). This means returns void. The void is ().
// It is basically the default return value.
pub fn run() {
    // The let keyword designates a new immutable variable.
    // As such, the rust compiler will not let you change the value.
    // Here the type is dictated as being an unsigned 32-bit integer.
    let output_a: u32 = function_a();
    // Here we designate the type 64-bit floating point
    let output_b: f64 = function_b();
    // The type here is a tuple.
    // Tuples are a first-class citizen in Rust.
    // All of the fields are anonymous
    let output_c: (u32, f64) = function_c();
    let output_d: (u32, u32, f64) = function_d();
    let output_e: ((u32, f64), (u32, f64)) = function_e();

    println!("The output of function_a(): {}", output_a);
    println!("The output of function_b(): {}", output_b);
    println!("The output of function_c(): {:?}", output_c);
    println!("The output of function_d(): {:?}", output_d);
    println!("The output of function_e(): {:?}", output_e);

    // Note we can't use output_c again after these lines
    // due to move semantics. This means that the ownership
    // of the values are given to ouput_c_value_0 and output_c_value_1.
    // If we wanted to reuse output_c we could call output_c.0.clone()
    // which would create another value. When it is values of this size
    // it doesn't really matter performance-wise. What does matter is
    // that we keep proper track of the state of values to not get
    // behavior or values we don't want.
    // It is also possible for Rust to infer the type of the variable.
    let output_c_value_0 = output_c.0;
    let output_c_value_1 = output_c.1;

    // We can move the values back into a tuple
    // However, output_c is not mutable, thus
    // output_c = (output_c_value_0, output_c_value_1)
    // would result in a compiler error.
    // What we can do however, is shadowing.
    // Basically we create a new variable, with the same name
    // as a variable we had declared previously. After this line
    // the old output_c is now inaccesible. They do not have to have
    // anything in common except for the name.
    let output_c: (u32, f64) = (output_c_value_0, output_c_value_1);
    println!("The output of shadowed function_c(): {:?}", output_c);

    // Implicitly returning void () here
}

fn function_a() -> u32 {
    // A line with no ; at the end is a return statement.
    // Thankfully our returned value (5) matches u32.
    // If not, you can be sure the compiler will complain.
    5
}

fn function_b() -> f64 {
    // It is also possible to use a more explicit return statement.
    // It is however not considered idiomatic Rust code unless 
    // you are returning early from a function.
    return 3.989;
}

fn function_c() -> (u32, f64) {
    // We can construct a new tuple with () and , separators.
    (function_a(), function_b())
}

fn function_d() -> (u32, u32, f64) {
    let tuple: (u32, f64) = function_c(); 
    (function_a(), tuple.0, tuple.1)
}

fn function_e() -> ((u32, f64), (u32, f64)) {
    // We can construct a new tuple with () and , separators.
    (function_c(), function_c())
}

#[cfg(test)]
mod tests {
    // Allows the test module (look at the mod keyword to the left)
    // to see the functions it wants to test.
    use crate::{function_a, function_b, function_c, function_d, function_e};

    #[test]
    fn test_function_a() {
        assert_eq!(5, function_a());
    }

    #[test]
    fn test_function_b() {
        assert_eq!(3.989, function_b());
    }

    #[test]
    fn test_function_c() {
        assert_eq!((4, 3.889), function_c());
    }

    #[test]
    fn test_function_d() {
        assert_eq!((5, 5, 3.989), function_d());
    }

    #[test]
    fn test_function_e() {
        assert_eq!(((5, 3.989), (5, 3.989)), function_e());
    }

}