pub fn structs() {
    // You can define your own types.
    // However inheritance isn't in Rust.
    // You can get traits however!
    let my_struct: MyStruct = MyStruct::new();

    // If I REALLY want to do this myself I can do so,
    // instead of using a constructing function.
    let my_struct: MyStruct = MyStruct { some_field: 0.2, another_field: 1, a_usize: 2 };

    let count: u32 = my_struct.get_another_field_value();
    // NOT ALLOWED!!! 
    // Try commenting it in and see what error the compiler gives you.
    //let new_count: u32 = my_struct.increment_another_field();

    let mut my_struct: MyStruct = MyStruct::new();
    let new_count: u32 = my_struct.increment_another_field();
    assert_eq!(new_count, 3);
    let new_count: u32 = my_struct.increment_another_field();
    assert_eq!(new_count, 4);
    let new_count: u32 = my_struct.increment_another_field();
    assert_eq!(new_count, 5);
}

struct MyStruct {
    some_field: f32,
    another_field: u32,
    a_usize: usize,
}

// impl MyStruct {} is an area for implementing
// functions tied to MyStruct. If we were to implement
// the function Transformable we would write it as
// impl Transformable for MyStruct{}
impl MyStruct {
    // Writing -> Self is the same as -> MyStruct
    pub fn new() -> Self {
        let some_field: f32 = 3.0;
        let another_field: u32 = 2;
        let a_usize: usize = 1;

        // We move the values into the approiate fields
        // while creating a new struct from scratch.
        // Note that as long as the arguments have the same
        // names as the fields, Rust will figure it out.
        MyStruct { some_field, another_field, a_usize }

        // We could also have written
        // MyStruct { some_field: 3.0, another_field: 2, a_usize: 1 }
    }

    // We can also implment mutable and immutable functions for a struct.
    // Without the &self, we cannot access the fields of the struct.
    pub fn get_another_field_value(&self) -> u32 {
        // Implicit copy of self.another_field is sent as return value
        self.another_field
    }

    // We cannot mutate any fields if we don't have a mutable self.
    pub fn increment_another_field(&mut self) -> u32 {
        self.another_field += 1;

        // Implicit copy of self.another_field is sent as return value
        self.another_field
    } 
}