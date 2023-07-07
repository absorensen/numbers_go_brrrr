struct AStruct{
    first_field: u32,
    second_field: f32,
}

enum MyEnum {
    // Completely empty type
    FirstType,
    // Note the intialization like a struct, this means
    // we even get name fields!
    SecondType{first_field: u32, second_field: f32},
    // Note the intialization like it is a tuple.
    // This also results in anonymous fields.
    ThirdType(u8, u8),
    // Can even contain named structs!
    FourthType(AStruct),
}

impl MyEnum {

    // A cheeky and contrived example, but
    // look how we can implement functions on
    // the enum, as well as sending a self
    // reference as an argument
    fn handle_this_enum(&self) -> u32 {
        handle_my_enum(&self)
    }
}

pub fn enums_and_match() {
    // Enums and Match statements are very powerful parts of Rust.
    let first_enum: MyEnum = MyEnum::FirstType;
    let second_enum: MyEnum = MyEnum::SecondType { first_field: 2, second_field: 3.2 };
    let third_enum: MyEnum = MyEnum::ThirdType(53, 42);
    
    // Not allowed! Type needs to be MyEnum!
    // let fourth_enum: MyEnum::FourthType = MyEnum::FourthType(AStruct { first_field: 42, second_field: 42.0 });
    let fourth_enum: MyEnum = MyEnum::FourthType(AStruct { first_field: 42, second_field: 42.0 });

    // Enums are sort of equivalent to what is called unions in C++.
    // They are a sort of struct which can be one or more 
    // different structs. They a value hidden away which communicates
    // Whether that specific enum is type A or type B within that 
    // type of enum. Having that functionality necessitates that 
    // the size in memory of enum type T is the maximum size of
    // all of the different types T covers.

    // Read more about Enum here: https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html

    // Since we can't just define the variables as being the specific subtype,
    // how do we handle which is which?
    // This is where Rust's match statement comes in. It is highly useful
    // and has the added benefit that Rust will check whether you handled
    // all cases in the match statement.
    handle_my_enum(&first_enum);
    handle_my_enum(&second_enum);

    third_enum.handle_this_enum();
    fourth_enum.handle_this_enum();

    // When you only care about a few of the cases you can
    // write a '_' case as a way of communicating that
    // "I don't care about the rest"

    match fourth_enum {
        // You can also communicate that you don't 
        // care about the underlying fields in the struct.
        MyEnum::FourthType(_) => {
            println!("In the fourth enum match case!");
        },
        _ => {
            println!("Just printing in the three other cases");
        }
    }

}

fn handle_my_enum(our_enum: &MyEnum) -> u32 {
    match our_enum {
        MyEnum::FirstType => 1,
        MyEnum::SecondType { first_field, second_field } => {
            first_field.clone()
        },
        MyEnum::ThirdType(first, second ) => {
            first.clone() as u32
        },
        MyEnum::FourthType(the_struct_within) => {
            the_struct_within.first_field.clone()
        },
    }
}