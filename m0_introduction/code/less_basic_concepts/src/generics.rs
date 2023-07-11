// Generics is a powerful tool. You denote
// some types in your function and as long 
// as all of these operations are valid for
// the types the compiler will generate
// however many versions of the function
// needed to fulfill the actual calls in
// your code to that function.

use std::ops::Add;

// This function requires that all types
// T implements the add operation and the
// output type T as well.
fn generic_add<T>(a: T, b: T) -> T 
where T : Add + Add<Output = T> {
    a + b
}

fn generic_function() {
    // Compiler generates ONE version of generic_add
    generic_add::<f32>(2.0, 3.4);

    // Compiler still generates ONE version of generic_add
    generic_add::<f32>(5.0, 1.0);

    // Compiler now generates TWO versions of generic_add
    generic_add::<i32>(1, 1);

    // Compiler now generates THREE version of generic_add
    generic_add::<f64>(3.50, 42.0);
}

// We can even implement a structs generically
// and with however many generic types we want.
struct TwoAndTwoValues<T, U> where T: Copy, U: Copy {
    x: T,
    y: T,
    z: U,
    w: U,
} 

impl <T, U> TwoAndTwoValues<T, U> where T: Copy, U: Copy {
    fn get_middle_values(&self) -> (T, U) {
        (self.y, self.z)
    }
}

fn generic_struct(){
    let our_struct: TwoAndTwoValues<f32, u32> = 
        TwoAndTwoValues { x: 3.14, y: 42.0, z: 42, w: 5 };
}

pub fn generics() {
    generic_function();
    generic_struct();
}