// Rust does not support object-oriented programming (OOP).
// In general, it seems that OOP is falling in popularity.
// Traits are basically interfaces. Interfaces being
// a list of methods that a given struct needs to implement
// to fulfill that interface. 


// One example - we could make the trait Fruit
trait Fruit {
    // Every struct implementing the Fruit trait has to 
    // implement is_edible just like this.
    fn is_edible(&self) -> bool;

    // We can also have methods on a trait, but with a default,
    // overrideable, implementation
    fn print(&self) {
        println!("Is fruit. Sad face emoji.");
    }

    fn is_juggleable(&self) -> bool {
        false
    }
}

// One example - we could make the trait Fruit
trait FoodItem {
    fn get_price(&self) -> f32 {
        0.0
    }
}



struct Apple {
    was_washed: bool,
}

impl Apple {
    fn some_function(&self) {
        // do something
    }
}

// Implementing a trait is almost the same
// as implementing other methods
impl Fruit for Apple {
    fn is_edible(&self) -> bool {
        self.was_washed
    }

    // We can also override methods with a default implementation
    // by just implementing that function.
    fn is_juggleable(&self) -> bool {
        true
    }
}

// A struct can implement more than 1 trait.
impl FoodItem for Apple {
    fn get_price(&self) -> f32 {
        3.50
    }
}

// Most of the time, traits are deceptively simple.
// We can also use traits to take an argument as any
// struct which implements a trait. However, what
// you can do with that struct is limited to
// the methods of that trait.
fn eat_fruit(item: impl Fruit) -> bool {
    item.is_edible()
}

// Or we can return a Fruit
fn get_fruit() -> impl Fruit {
    Apple { was_washed: true }
}


// Deriving traits is an absolute super power.
// A lot of the standard traits can be automatically
// derived by writing a single line above the
// struct definition. Usually, this requires that 
// all fields of the struct implement said trait.
#[derive(Copy, Clone, Debug)]
struct StructWithDerivedTraits {
    field_one: f32,
    field_two: u32,
}


pub fn traits() {
    // Copy and Clone have already been explained, but
    // let's try out Debug.
    let data: StructWithDerivedTraits = StructWithDerivedTraits { field_one: 3.50, field_two: 42 };
    println!("Debug print: {:?}", data );
}
